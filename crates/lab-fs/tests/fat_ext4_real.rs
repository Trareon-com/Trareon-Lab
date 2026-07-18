use lab_core::NullProgress;
use lab_fs::{
    enumerate_exfat, enumerate_ext4, enumerate_fat32, write_minimal_exfat_image,
    write_minimal_ext4_image, write_minimal_fat32_image,
};
use lab_storage::RawImage;

#[test]
fn fat32_lists_hello() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("fat.img");
    write_minimal_fat32_image(&path).unwrap();
    let mut img = RawImage::open_raw(&path).unwrap();
    let entries = enumerate_fat32(&mut img, &mut NullProgress).unwrap();
    assert!(entries.iter().any(|e| e.name.starts_with("HELLO")));
    assert!(entries.iter().any(|e| e.deleted));
}

#[test]
fn exfat_lists_hi() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("exfat.img");
    write_minimal_exfat_image(&path).unwrap();
    let mut img = RawImage::open_raw(&path).unwrap();
    let entries = enumerate_exfat(&mut img, &mut NullProgress).unwrap();
    assert!(entries.iter().any(|e| e.name == "hi"));
}

#[test]
fn ext4_lists_hello_txt() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("ext4.img");
    write_minimal_ext4_image(&path).unwrap();
    let mut img = RawImage::open_raw(&path).unwrap();
    let entries = enumerate_ext4(&mut img, &mut NullProgress).unwrap();
    assert!(entries.iter().any(|e| e.name == "hello.txt"), "{entries:?}");
}
