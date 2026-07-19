use lab_carving::{Carver, DEFAULT_MAX_CARVE_BYTES};
use lab_core::NullProgress;
use lab_storage::RawImage;
use std::io::Write;

#[test]
fn carve_jpeg_and_pdf() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("disk.raw");
    let mut blob = vec![0u8; 4096];
    let jpeg = [0xFFu8, 0xD8, 0xFF, 0xE0, 1, 2, 3, 4, 0xFF, 0xD9];
    blob[100..100 + jpeg.len()].copy_from_slice(&jpeg);
    let pdf = b"%PDF-1.4\n%\xe2\xe3\xcf\xd3\n1 0 obj\n<<>>\nendobj\n%%EOF\n";
    blob[500..500 + pdf.len()].copy_from_slice(pdf);
    std::fs::File::create(&path)
        .unwrap()
        .write_all(&blob)
        .unwrap();

    let mut img = RawImage::open_raw(&path).unwrap();
    let carved = Carver::default()
        .carve(&mut img, &mut NullProgress)
        .unwrap();
    assert!(carved.iter().any(|c| c.signature_name == "jpg"));
    assert!(carved.iter().any(|c| c.signature_name == "pdf"));
}

#[test]
fn carve_gif_ole_7z_and_mp4() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("multi.raw");
    let mut blob = vec![0u8; 8192];

    let gif = b"GIF89a\x01\x00\x01\x00\x00\x00\x00\x00\x3b";
    blob[32..32 + gif.len()].copy_from_slice(gif);

    let ole = [
        0xD0u8, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1, 0x00, 0x00, 0x00, 0x00,
    ];
    // pad to min_size 512
    let ole_start = 200;
    blob[ole_start..ole_start + ole.len()].copy_from_slice(&ole);
    // ensure region looks long enough
    for i in 0..512 {
        if blob[ole_start + i] == 0 {
            blob[ole_start + i] = 0x11;
        }
    }

    let seven = [0x37u8, 0x7A, 0xBC, 0xAF, 0x27, 0x1C, 0, 0, 0, 0];
    blob[900..900 + seven.len()].copy_from_slice(&seven);

    // MP4: size(4) + 'ftyp'
    let mp4 = [
        0x00u8, 0x00, 0x00, 0x18, b'f', b't', b'y', b'p', b'i', b's', b'o', b'm', 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];
    blob[1200..1200 + mp4.len()].copy_from_slice(&mp4);

    std::fs::File::create(&path)
        .unwrap()
        .write_all(&blob)
        .unwrap();
    let mut img = RawImage::open_raw(&path).unwrap();
    let carved = Carver::common_media()
        .carve(&mut img, &mut NullProgress)
        .unwrap();
    assert!(carved.iter().any(|c| c.signature_name == "gif"));
    assert!(carved.iter().any(|c| c.signature_name == "ole"));
    assert!(carved.iter().any(|c| c.signature_name == "7z"));
    assert!(carved.iter().any(|c| c.signature_name == "mp4"));
    assert!(!carved.iter().any(|c| c.signature_name == "exe"));
}

#[test]
fn carve_refuses_oversize_image() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("big.raw");
    // Don't write 65MiB — use a tiny file but override max_image_bytes to 8.
    std::fs::File::create(&path)
        .unwrap()
        .write_all(&[0u8; 64])
        .unwrap();
    let mut img = RawImage::open_raw(&path).unwrap();
    let carver = Carver {
        signatures: lab_carving::default_signatures(),
        max_image_bytes: 8,
    };
    let err = carver.carve(&mut img, &mut NullProgress).unwrap_err();
    let msg = format!("{err:?}");
    assert!(msg.contains("carve refused") || msg.contains("Limited"));
    const {
        assert!(DEFAULT_MAX_CARVE_BYTES >= 64 * 1024 * 1024);
    };
}
