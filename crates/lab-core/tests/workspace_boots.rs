//! F1: workspace must boot and expose a semver version string.

#[test]
fn lab_core_version_is_semver() {
    let version = lab_core::version();
    let mut parts = version.split('.');
    let major = parts.next().expect("major");
    let minor = parts.next().expect("minor");
    let patch = parts.next().expect("patch");
    assert!(parts.next().is_none(), "unexpected extra semver segments");
    assert!(
        major.chars().all(|c| c.is_ascii_digit()),
        "major not numeric"
    );
    assert!(
        minor.chars().all(|c| c.is_ascii_digit()),
        "minor not numeric"
    );
    assert!(
        patch.chars().all(|c| c.is_ascii_digit()),
        "patch not numeric"
    );
    assert!(!version.is_empty());
}
