use lab_antiforensics::{analyze, detect_timestomping, TimestampDiscrepancy};
use lab_core::NullProgress;
use lab_fs::{
    write_minimal_ntfs_image, Attribute, FileNameAttr, MftIterator, MftRecord, MftRecordFlags,
    NtfsVolume, StdInfo, Timestamps,
};
use lab_storage::RawImage;

#[test]
fn timestomp_and_analyze_on_fixture() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mini.img");
    write_minimal_ntfs_image(&path).unwrap();
    let mut img = RawImage::open_raw(&path).unwrap();
    let vol = NtfsVolume::open(&mut img).unwrap();
    let mut mft = MftIterator::open(&mut img, vol).unwrap();
    let mut records = Vec::new();
    for i in 0..mft.total_records() {
        if let Ok(Some(r)) = mft.read_record(i) {
            records.push(r);
        }
    }
    records.push(MftRecord {
        record_number: 99,
        sequence: 1,
        flags: MftRecordFlags {
            in_use: true,
            is_directory: false,
        },
        base_record: Some(9999),
        link_count: 1,
        attributes: vec![
            Attribute::StandardInformation(StdInfo {
                timestamps: Timestamps {
                    created: 500,
                    modified: 100,
                    mft_modified: 100,
                    accessed: 100,
                },
                file_attributes: 0,
            }),
            Attribute::FileName(FileNameAttr {
                parent_record: 5,
                parent_sequence: 1,
                timestamps: Timestamps {
                    created: 200,
                    modified: 200,
                    mft_modified: 200,
                    accessed: 200,
                },
                allocated_size: 0,
                real_size: 0,
                flags: 0,
                name_space: 1,
                name: "stomped.txt".into(),
            }),
        ],
        raw: Vec::new(),
    });

    let hits = detect_timestomping(&records);
    assert!(hits
        .iter()
        .any(|h| h.discrepancy == TimestampDiscrepancy::Timestomped));

    let result = analyze(&records, &[], &mut NullProgress).unwrap();
    assert!(!result.timestomping_hits.is_empty());
    assert!(!result.usn_anomalies.is_empty());
    assert!(result
        .mft_anomalies
        .iter()
        .any(|a| a.kind == "dangling_base"));
}
