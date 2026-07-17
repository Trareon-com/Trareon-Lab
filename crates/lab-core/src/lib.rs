//! Authoritative forensic core for Trareon Lab (Foundation).

pub mod error;
pub mod export;
pub mod progress;
pub mod result;
pub mod schema_validate;

pub use error::LabError;
pub use progress::{FnProgress, NullProgress, ProgressEvent, ProgressSink, SharedProgress};
pub use result::LabResult;

/// Workspace package version as a semver string (`MAJOR.MINOR.PATCH`).
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!version().is_empty());
    }
}
