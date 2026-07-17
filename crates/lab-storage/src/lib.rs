//! Disk image / raw evidence readers for Trareon Lab.

pub mod import;
mod raw;

pub use import::{import_raw_image, ImportImageResult};
pub use raw::{detect_image_kind, ImageKind, RawImage};
