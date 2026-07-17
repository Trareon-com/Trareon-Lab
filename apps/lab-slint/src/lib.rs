//! Minimal Slint foundation UI.

pub mod docs_shell;
pub mod ui_model;

pub use docs_shell::{docs_root, load_docs_index};
pub use ui_model::UiSnapshot;

#[cfg(feature = "gui")]
slint::include_modules!();
