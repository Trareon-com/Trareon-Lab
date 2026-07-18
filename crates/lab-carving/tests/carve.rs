use lab_carving::Carver;
use lab_core::NullProgress;
use lab_storage::RawImage;
use std::io::Write;

#[test]
fn carve_jpeg_and_pdf() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("disk.raw");
    let mut blob = vec![0u8; 4096];
    // JPEG
    let jpeg = [0xFFu8, 0xD8, 0xFF, 0xE0, 1, 2, 3, 4, 0xFF, 0xD9];
    blob[100..100 + jpeg.len()].copy_from_slice(&jpeg);
    // PDF
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
