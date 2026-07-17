//! Authoritative forensic core for Trareon Lab (Foundation).

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
