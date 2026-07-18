//! Minimal real-format NTFS image builder for CI fixtures.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use lab_core::{LabError, LabResult};

const RECORD_SIZE: usize = 1024;
const CLUSTER_SIZE: usize = 4096;
const MFT_CLUSTER: u64 = 4; // MFT starts at cluster 4

/// Write a tiny NTFS volume: boot + $MFT with root, one live file, one deleted file.
pub fn write_minimal_ntfs_image(path: &Path) -> LabResult<()> {
    // 32 clusters = 128 KiB
    let mut img = vec![0u8; 32 * CLUSTER_SIZE];

    // Boot sector
    img[0] = 0xEB;
    img[1] = 0x52;
    img[2] = 0x90;
    img[3..11].copy_from_slice(b"NTFS    ");
    img[0x0B..0x0D].copy_from_slice(&512u16.to_le_bytes());
    img[0x0D] = (CLUSTER_SIZE / 512) as u8; // 8
    let total_sectors = (img.len() / 512) as u64;
    img[0x28..0x30].copy_from_slice(&total_sectors.to_le_bytes());
    img[0x30..0x38].copy_from_slice(&MFT_CLUSTER.to_le_bytes());
    img[0x38..0x40].copy_from_slice(&MFT_CLUSTER.to_le_bytes());
    img[0x40] = (RECORD_SIZE.trailing_zeros() as i8).wrapping_neg() as u8; // 2^10 = 1024 → -10
    img[0x44] = (4096u32.trailing_zeros() as i8).wrapping_neg() as u8;
    img[0x48..0x50].copy_from_slice(&0x1122_3344_5566_7788u64.to_le_bytes());
    img[0x1FE] = 0x55;
    img[0x1FF] = 0xAA;

    let mft_off = MFT_CLUSTER as usize * CLUSTER_SIZE;

    // Record 0: $MFT with non-resident $DATA covering 8 clusters of MFT area
    let mut rec0 = empty_record(0, 1, true, false);
    let data_runs = encode_run(8, MFT_CLUSTER as i64); // 8 clusters from MFT_CLUSTER
    append_nonresident_data(&mut rec0, &data_runs, 8 * CLUSTER_SIZE as u64);
    img[mft_off..mft_off + RECORD_SIZE].copy_from_slice(&rec0);

    // Records 1-4 unused FILE stubs
    for i in 1usize..5 {
        let rec = empty_record(i as u64, 1, true, false);
        let off = mft_off + i * RECORD_SIZE;
        img[off..off + RECORD_SIZE].copy_from_slice(&rec);
    }

    // Record 5: root directory .
    let mut root = empty_record(5, 1, true, true);
    append_stdinfo(&mut root, 100, 100, 100, 100);
    append_filename(&mut root, 5, ".", 1, true);
    img[mft_off + 5 * RECORD_SIZE..mft_off + 6 * RECORD_SIZE].copy_from_slice(&root);

    // Record 6: live file secret.txt
    let mut live = empty_record(6, 1, true, false);
    append_stdinfo(&mut live, 200, 250, 250, 200);
    append_filename(&mut live, 5, "secret.txt", 1, false);
    append_resident_data(&mut live, b"hello-ntfs");
    img[mft_off + 6 * RECORD_SIZE..mft_off + 7 * RECORD_SIZE].copy_from_slice(&live);

    // Record 7: deleted file gone.txt (in_use = false)
    let mut gone = empty_record(7, 1, false, false);
    append_stdinfo(&mut gone, 300, 300, 300, 300);
    append_filename(&mut gone, 5, "gone.txt", 1, false);
    append_resident_data(&mut gone, b"deleted");
    img[mft_off + 7 * RECORD_SIZE..mft_off + 8 * RECORD_SIZE].copy_from_slice(&gone);

    // USN-like raw blob at end of image for unit tests (not wired as ADS here)
    let usn_blob = build_sample_usn();
    let usn_off = img.len() - 4096;
    img[usn_off..usn_off + usn_blob.len()].copy_from_slice(&usn_blob);

    let mut f = File::create(path).map_err(|e| LabError::Internal {
        detail: format!("create ntfs fixture: {e}"),
    })?;
    f.write_all(&img).map_err(|e| LabError::Internal {
        detail: format!("write ntfs fixture: {e}"),
    })?;
    Ok(())
}

fn empty_record(number: u64, seq: u16, in_use: bool, is_dir: bool) -> Vec<u8> {
    let mut r = vec![0u8; RECORD_SIZE];
    r[0..4].copy_from_slice(b"FILE");
    r[0x04..0x06].copy_from_slice(&0u16.to_le_bytes()); // no USA
    r[0x06..0x08].copy_from_slice(&0u16.to_le_bytes());
    r[0x10..0x12].copy_from_slice(&seq.to_le_bytes());
    r[0x12..0x14].copy_from_slice(&1u16.to_le_bytes()); // link count
    r[0x14..0x16].copy_from_slice(&0x38u16.to_le_bytes()); // first attr
    let mut flags = 0u16;
    if in_use {
        flags |= 0x01;
    }
    if is_dir {
        flags |= 0x02;
    }
    r[0x16..0x18].copy_from_slice(&flags.to_le_bytes());
    r[0x18..0x1C].copy_from_slice(&(RECORD_SIZE as u32).to_le_bytes());
    r[0x1C..0x20].copy_from_slice(&(RECORD_SIZE as u32).to_le_bytes());
    // Store record number at end for debugging (not standard)
    let _ = number;
    // End marker at first attr for empty — will be overwritten when attrs appended
    r[0x38..0x3C].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
    r
}

fn attr_cursor(rec: &[u8]) -> usize {
    let mut off = u16::from_le_bytes(rec[0x14..0x16].try_into().unwrap()) as usize;
    while off + 8 <= rec.len() {
        let ty = u32::from_le_bytes(rec[off..off + 4].try_into().unwrap());
        if ty == 0xFFFF_FFFF {
            return off;
        }
        let len = u32::from_le_bytes(rec[off + 4..off + 8].try_into().unwrap()) as usize;
        if len == 0 {
            return off;
        }
        off += len;
    }
    off
}

fn append_stdinfo(rec: &mut [u8], c: u64, m: u64, mft: u64, a: u64) {
    let mut content = vec![0u8; 48];
    content[0..8].copy_from_slice(&c.to_le_bytes());
    content[8..16].copy_from_slice(&m.to_le_bytes());
    content[16..24].copy_from_slice(&mft.to_le_bytes());
    content[24..32].copy_from_slice(&a.to_le_bytes());
    append_resident_attr(rec, 0x10, &content);
}

fn append_filename(rec: &mut [u8], parent: u64, name: &str, ns: u8, is_dir: bool) {
    let utf16: Vec<u8> = name.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    let mut content = vec![0u8; 66 + utf16.len()];
    let pref = (parent & 0x0000_FFFF_FFFF) | ((1u64) << 48);
    content[0..8].copy_from_slice(&pref.to_le_bytes());
    // timestamps
    for i in 0..4 {
        content[8 + i * 8..16 + i * 8].copy_from_slice(&1000u64.to_le_bytes());
    }
    let flags = if is_dir { 0x1000_0000u32 } else { 0 };
    content[56..60].copy_from_slice(&flags.to_le_bytes());
    content[64] = (name.encode_utf16().count()) as u8;
    content[65] = ns;
    content[66..].copy_from_slice(&utf16);
    append_resident_attr(rec, 0x30, &content);
}

fn append_resident_data(rec: &mut [u8], data: &[u8]) {
    append_resident_attr(rec, 0x80, data);
}

fn append_resident_attr(rec: &mut [u8], ty: u32, content: &[u8]) {
    let off = attr_cursor(rec);
    let content_off = 24u16;
    let total = (content_off as usize + content.len()).div_ceil(8) * 8;
    if off + total + 8 > rec.len() {
        return;
    }
    rec[off..off + 4].copy_from_slice(&ty.to_le_bytes());
    rec[off + 4..off + 8].copy_from_slice(&(total as u32).to_le_bytes());
    rec[off + 8] = 0; // resident
    rec[off + 9] = 0; // name len
    rec[off + 10..off + 12].copy_from_slice(&0u16.to_le_bytes());
    rec[off + 16..off + 20].copy_from_slice(&(content.len() as u32).to_le_bytes());
    rec[off + 20..off + 22].copy_from_slice(&content_off.to_le_bytes());
    rec[off + content_off as usize..off + content_off as usize + content.len()]
        .copy_from_slice(content);
    let end = off + total;
    rec[end..end + 4].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
}

fn append_nonresident_data(rec: &mut [u8], runs: &[u8], data_size: u64) {
    let off = attr_cursor(rec);
    let run_off = 64u16;
    let total = (run_off as usize + runs.len() + 1).div_ceil(8) * 8;
    if off + total + 8 > rec.len() {
        return;
    }
    rec[off..off + 4].copy_from_slice(&0x80u32.to_le_bytes());
    rec[off + 4..off + 8].copy_from_slice(&(total as u32).to_le_bytes());
    rec[off + 8] = 1; // non-resident
    rec[off + 16..off + 24].copy_from_slice(&0u64.to_le_bytes()); // lowest vcn
    let highest = (data_size / CLUSTER_SIZE as u64).saturating_sub(1);
    rec[off + 24..off + 32].copy_from_slice(&highest.to_le_bytes());
    rec[off + 32..off + 34].copy_from_slice(&run_off.to_le_bytes());
    rec[off + 40..off + 48].copy_from_slice(&data_size.to_le_bytes()); // allocated
    rec[off + 48..off + 56].copy_from_slice(&data_size.to_le_bytes()); // real size
    rec[off + run_off as usize..off + run_off as usize + runs.len()].copy_from_slice(runs);
    rec[off + run_off as usize + runs.len()] = 0;
    let end = off + total;
    rec[end..end + 4].copy_from_slice(&0xFFFF_FFFFu32.to_le_bytes());
}

fn encode_run(length: u64, lcn: i64) -> Vec<u8> {
    // 1-byte length, 1-byte offset
    vec![0x11, length as u8, lcn as u8]
}

fn build_sample_usn() -> Vec<u8> {
    let name = "secret.txt"
        .encode_utf16()
        .flat_map(|u| u.to_le_bytes())
        .collect::<Vec<_>>();
    let name_off = 60u16;
    let record_len = (name_off as usize + name.len()).div_ceil(8) * 8;
    let mut r = vec![0u8; record_len];
    r[0..4].copy_from_slice(&(record_len as u32).to_le_bytes());
    r[4..6].copy_from_slice(&2u16.to_le_bytes()); // major
    r[6..8].copy_from_slice(&0u16.to_le_bytes());
    r[8..16].copy_from_slice(&6u64.to_le_bytes()); // file ref
    r[16..24].copy_from_slice(&5u64.to_le_bytes()); // parent
    r[24..32].copy_from_slice(&1u64.to_le_bytes()); // usn
    r[32..40].copy_from_slice(&999u64.to_le_bytes());
    r[40..44].copy_from_slice(&0x0000_0100u32.to_le_bytes()); // FILE_CREATE
    r[56..58].copy_from_slice(&(name.len() as u16).to_le_bytes());
    r[58..60].copy_from_slice(&name_off.to_le_bytes());
    r[name_off as usize..name_off as usize + name.len()].copy_from_slice(&name);
    r
}
