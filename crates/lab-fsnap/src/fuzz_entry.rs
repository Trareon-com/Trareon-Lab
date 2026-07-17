//! Fuzz harness entry for `.fsnap` preflight (cargo-fuzz compatible stub).

use crate::preflight::{preflight_package, PreflightLimits};
use std::path::Path;

/// Entry used by fuzzers / property tests: never panics on hostile bytes as paths.
pub fn fuzz_preflight_path(path: &Path) {
    let _ = preflight_package(path, PreflightLimits::default());
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn fuzz_entry_handles_empty_dir() {
        let dir = tempdir().unwrap();
        fuzz_preflight_path(dir.path());
    }
}
