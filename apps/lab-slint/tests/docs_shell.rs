//! F12: offline docs index opens and lists Foundation topics.

use lab_slint::load_docs_index;

#[test]
fn offline_docs_index_lists_foundation_topics() {
    let index = load_docs_index().expect("docs index");
    assert!(index.contains("Case lifecycle"));
    assert!(index.contains(".fsnap"));
    assert!(lab_slint::docs_root().join("INDEX.md").is_file());
}
