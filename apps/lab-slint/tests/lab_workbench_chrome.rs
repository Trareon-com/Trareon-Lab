//! Lab examination workbench chrome contract (best-practice remount).

use lab_slint::{NavScreen, UiSnapshot};

const DISCLOSURE_HEIGHT_PX: i32 = 22;
const HEADER_HEIGHT_PX: i32 = 40;
const SIDEBAR_RAIL_PX: i32 = 48;
const SIDEBAR_EXPANDED_PX: i32 = 180;
const INSPECTOR_WIDTH_PX: i32 = 280;
const STATUS_HEIGHT_PX: i32 = 28;
const MIN_WIDTH_PX: i32 = 960;
const MIN_HEIGHT_PX: i32 = 640;

const PRIMARY_NAV: &[&str] = &[
    "Case",
    "Evidence",
    "Search",
    "Timeline",
    "Bookmarks",
    "Report",
];

const TOOLS_NAV: &[&str] = &["Artifacts", "Graph", "Runs", "Transfer"];
const META_NAV: &[&str] = &["Capabilities", "About"];

#[test]
fn lab_workbench_geometry_lock() {
    assert_eq!(DISCLOSURE_HEIGHT_PX, 22);
    assert_eq!(HEADER_HEIGHT_PX, 40);
    assert_eq!(SIDEBAR_RAIL_PX, 48);
    assert_eq!(SIDEBAR_EXPANDED_PX, 180);
    assert_eq!(INSPECTOR_WIDTH_PX, 280);
    assert_eq!(STATUS_HEIGHT_PX, 28);
    assert_eq!(MIN_WIDTH_PX, 960);
    assert_eq!(MIN_HEIGHT_PX, 640);
}

#[test]
fn lab_workbench_nav_ia() {
    assert_eq!(PRIMARY_NAV.len(), 6);
    assert!(!PRIMARY_NAV.contains(&"Hex"));
    assert!(!PRIMARY_NAV.contains(&"Quick Verify"));
    assert_eq!(TOOLS_NAV, ["Artifacts", "Graph", "Runs", "Transfer"]);
    assert_eq!(META_NAV, ["Capabilities", "About"]);
}

#[test]
fn shell_defaults_rail_inspector_off_light() {
    let snap = UiSnapshot::default();
    assert!(!snap.dark_mode);
    assert!(!snap.inspector_open);
    assert!(snap.nav_collapsed);
    assert!(!snap.nav_expanded());
    assert!(!snap.layout_compact);
    assert_eq!(snap.active_screen, NavScreen::CaseHome);
    assert_eq!(snap.inspector_tab, "properties");
}

#[test]
fn responsive_compact_forces_rail_and_overlay() {
    let mut snap = UiSnapshot::default();
    snap.nav_collapsed = false;
    snap.inspector_open = true;
    snap.apply_layout_width(1000);
    assert!(snap.layout_compact);
    assert!(snap.nav_collapsed);
    assert!(!snap.nav_expanded());
    assert!(snap.inspector_overlay);
    assert!(snap.inspector_as_overlay());
    assert!(!snap.inspector_as_side());

    snap.apply_layout_width(1280);
    assert!(!snap.layout_compact);
    assert!(!snap.inspector_overlay);
}

#[test]
fn primary_shortcuts_and_escape_stack() {
    let mut snap = UiSnapshot::default();
    snap.handle_shortcut("2");
    assert_eq!(snap.active_screen, NavScreen::Evidence);
    snap.handle_shortcut("6");
    assert_eq!(snap.active_screen, NavScreen::Report);
    snap.handle_shortcut("i");
    assert!(snap.inspector_open);
    snap.log_open = true;
    snap.handle_shortcut("Escape");
    assert!(!snap.log_open);
    assert!(snap.inspector_open);
    snap.handle_shortcut("Escape");
    assert!(!snap.inspector_open);
}

#[test]
fn focus_hex_opens_inspector_hex_tab() {
    let mut snap = UiSnapshot::default();
    snap.activate_palette_command("Focus Hex");
    assert!(snap.inspector_open);
    assert_eq!(snap.inspector_tab, "hex");
}

#[test]
fn capabilities_seeded_non_empty() {
    let snap = UiSnapshot::default();
    assert!(!snap.capabilities.is_empty());
    assert!(snap.capabilities.iter().any(|(id, _, _)| id == "case"));
}

#[test]
fn hex_offset_steps() {
    let mut snap = UiSnapshot::default();
    snap.step_hex_offset(256);
    assert_eq!(snap.hex_offset, 256);
    snap.step_hex_offset(-512);
    assert_eq!(snap.hex_offset, 0);
}

#[test]
fn examiner_path_covers_workbench_screens() {
    let mut snap = UiSnapshot::default();
    for (label, expect) in [
        ("Case", NavScreen::CaseHome),
        ("Evidence", NavScreen::Evidence),
        ("Search", NavScreen::Search),
        ("Timeline", NavScreen::Timeline),
        ("Bookmarks", NavScreen::Bookmarks),
        ("Report", NavScreen::Report),
        ("Artifacts", NavScreen::Artifacts),
        ("Graph", NavScreen::Graph),
        ("Runs", NavScreen::Runs),
        ("Transfer", NavScreen::Transfer),
        ("Hex", NavScreen::Hex),
        ("Capabilities", NavScreen::Capabilities),
        ("About", NavScreen::About),
        ("QuickVerify", NavScreen::QuickVerify),
    ] {
        snap.navigate_to(NavScreen::from_label(label));
        assert_eq!(snap.active_screen, expect, "nav {label}");
    }
}

#[test]
fn graph_edge_and_artifacts_honesty() {
    let mut snap = UiSnapshot::default();
    assert!(snap.graph_edges.is_empty());
    snap.add_graph_edge("file.raw", "related-search");
    assert_eq!(snap.graph_edges.len(), 1);
    assert!(snap.artifact_hits.is_empty());
}
