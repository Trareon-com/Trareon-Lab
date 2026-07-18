//! Minimal Slint foundation UI.

pub mod docs_shell;
pub mod session;
pub mod ui_model;

pub use docs_shell::{docs_root, load_docs_index};
pub use session::{sellable_disclosure, LabSession};
pub use ui_model::{ArtifactHitRow, EvidenceFileRow, FindingRow, NavScreen, UiSnapshot};

#[cfg(feature = "gui")]
slint::include_modules!();
