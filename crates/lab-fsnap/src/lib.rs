//! Safe `.fsnap` preflight and import.

pub mod import;
pub mod preflight;

pub use import::import_package;
pub use preflight::{PreflightLimits, PreflightOk, preflight_package};
