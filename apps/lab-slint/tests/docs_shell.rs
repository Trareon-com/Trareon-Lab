//! F12: offline docs index opens and lists Foundation topics.

use lab_slint::{guide_status, load_docs_index, validate_capability_status, GuideStatus};

#[test]
fn offline_docs_index_lists_foundation_topics() {
    let index = load_docs_index().expect("docs index");
    assert!(index.contains("Case lifecycle"));
    assert!(index.contains(".fsnap"));
    assert!(lab_slint::docs_root().join("INDEX.md").is_file());
}

#[test]
fn validated_capability_requires_present_guide() {
    assert_eq!(guide_status("E01"), GuideStatus::Present);
    assert!(validate_capability_status("E01", "Validated").is_ok());
    assert_eq!(guide_status("unwritten parser"), GuideStatus::Missing);
    assert!(validate_capability_status("unwritten parser", "Validated").is_err());
    assert!(validate_capability_status("unwritten parser", "Limited").is_ok());
}
