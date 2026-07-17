#[cfg(feature = "gui")]
fn main() -> Result<(), slint::PlatformError> {
    use lab_slint::{AppWindow, UiSnapshot};
    use slint::ComponentHandle;

    let ui = AppWindow::new()?;
    let mut snapshot = UiSnapshot::default();
    snapshot.open_case("Foundation Demo Case", 1, 2);

    ui.set_case_title(snapshot.case_title.clone().into());
    ui.set_evidence_count(snapshot.evidence_count);
    ui.set_coverage_count(snapshot.coverage_count);
    ui.set_open_case_focused(snapshot.open_case_focused);

    let ui_weak = ui.as_weak();
    ui.on_open_case_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            ui.set_case_title("Opened Case".into());
            ui.set_evidence_count(1);
            ui.set_coverage_count(1);
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
