//! Model UX for Lab frontend polish (timeline filter, search selection, paging).

use lab_slint::UiSnapshot;

#[test]
fn filtered_timeline_labels_substring() {
    let mut snap = UiSnapshot {
        timeline_labels: vec![
            "2024-01-01T10:00:00Z create".into(),
            "2024-02-15T12:00:00Z modify".into(),
            "noise line".into(),
        ],
        ..Default::default()
    };
    snap.set_timeline_filter("modify");
    let filtered = snap.filtered_timeline_labels();
    assert_eq!(filtered.len(), 1);
    assert!(filtered[0].contains("modify"));
}

#[test]
fn filtered_timeline_labels_from_prefix() {
    let mut snap = UiSnapshot {
        timeline_labels: vec![
            "2024-01-01T10:00:00Z a".into(),
            "2024-06-01T10:00:00Z b".into(),
        ],
        ..Default::default()
    };
    snap.set_timeline_filter("from:2024-06");
    let filtered = snap.filtered_timeline_labels();
    assert_eq!(filtered.len(), 1);
    assert!(filtered[0].starts_with("2024-06"));
}

#[test]
fn select_search_hit_and_clear() {
    let mut snap = UiSnapshot {
        search_results: vec!["a".into(), "b".into(), "c".into()],
        ..Default::default()
    };
    assert!(snap.select_search_hit(1));
    assert_eq!(snap.selected_search_index, Some(1));
    assert!(!snap.select_search_hit(9));
    snap.clear_search_selection();
    assert_eq!(snap.selected_search_index, None);
}

#[test]
fn evidence_page_prev_next() {
    let mut snap = UiSnapshot {
        list_total: 500,
        list_page_size: 200,
        list_offset: 0,
        ..Default::default()
    };
    snap.page_next();
    assert_eq!(snap.list_offset, 200);
    snap.page_next();
    assert_eq!(snap.list_offset, 400);
    snap.page_next();
    assert_eq!(snap.list_offset, 400); // clamped
    snap.page_prev();
    assert_eq!(snap.list_offset, 200);
}
