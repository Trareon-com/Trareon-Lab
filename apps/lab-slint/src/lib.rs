//! Minimal Slint foundation UI.

pub mod ui_model;

pub use ui_model::UiSnapshot;

#[cfg(feature = "gui")]
slint::include_modules!();
