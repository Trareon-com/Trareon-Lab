//! Minimal Slint foundation UI.

pub mod docs_shell;
pub mod session;
pub mod ui_model;

pub use docs_shell::{
    docs_root, guide_status, load_docs_index, validate_capability_status, GuideStatus,
};
pub use session::{DiscrepancyRow, LabSession};
pub use ui_model::{ArtifactHitRow, EvidenceFileRow, FindingRow, NavScreen, UiSnapshot};

#[cfg(feature = "gui")]
slint::include_modules!();
