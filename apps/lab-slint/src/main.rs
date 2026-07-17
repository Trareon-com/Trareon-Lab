#[cfg(feature = "gui")]
fn main() -> Result<(), slint::PlatformError> {
    use lab_slint::{AppWindow, NavScreen, UiSnapshot};
    use slint::ComponentHandle;
    use std::cell::RefCell;
    use std::rc::Rc;

    let ui = AppWindow::new()?;
    let snapshot = Rc::new(RefCell::new(UiSnapshot::default()));
    {
        let mut snap = snapshot.borrow_mut();
        snap.open_case("Foundation Demo Case", 1, 2);
        snap.set_bookmark_count(0);
    }

    let apply = |ui: &AppWindow, snap: &UiSnapshot| {
        ui.set_case_title(snap.case_title.clone().into());
        ui.set_evidence_count(snap.evidence_count);
        ui.set_coverage_count(snap.coverage_count);
        ui.set_bookmark_count(snap.bookmark_count);
        ui.set_open_case_focused(snap.open_case_focused);
        ui.set_active_screen(snap.active_screen.label().into());
    };
    apply(&ui, &snapshot.borrow());

    let ui_weak = ui.as_weak();
    let snap_open = Rc::clone(&snapshot);
    ui.on_open_case_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_open.borrow_mut();
            snap.open_case("Opened Case", 1, 1);
            snap.set_bookmark_count(0);
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_nav = Rc::clone(&snapshot);
    ui.on_navigate(move |label| {
        if let Some(ui) = ui_weak.upgrade() {
            let screen = match label.as_str() {
                "Evidence" => NavScreen::Evidence,
                "Search" => NavScreen::Search,
                "Timeline" => NavScreen::Timeline,
                "Bookmarks" => NavScreen::Bookmarks,
                "Report" => NavScreen::Report,
                _ => NavScreen::CaseHome,
            };
            let mut snap = snap_nav.borrow_mut();
            snap.navigate_to(screen);
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_import = Rc::clone(&snapshot);
    ui.on_import_evidence_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_import.borrow_mut();
            snap.import_evidence_stub();
            apply(&ui, &snap);
        }
    });

    ui.invoke_focus_open_case();
    ui.run()
}

#[cfg(not(feature = "gui"))]
fn main() {
    eprintln!("lab-slint built without `gui` feature");
    std::process::exit(1);
}
