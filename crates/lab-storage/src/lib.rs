//! Disk image / raw evidence readers for Trareon Lab.

pub mod e01;
pub mod image;
pub mod import;
mod raw;

pub use e01::{write_simple_e01, E01Image, E01Metadata};
pub use image::{ImageReader, IntegrityReport};
pub use import::{import_raw_image, open_image, ImportImageResult};
pub use raw::{detect_image_kind, ImageKind, RawImage};
