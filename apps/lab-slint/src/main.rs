#[cfg(feature = "gui")]
fn main() -> Result<(), slint::PlatformError> {
    use lab_slint::{
        AppWindow, ArtifactHitRow, EvidenceFileRow, FindingRow, NavScreen, UiSnapshot,
    };
    use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
    use std::cell::RefCell;
    use std::rc::Rc;

    let ui = AppWindow::new()?;
    let snapshot = Rc::new(RefCell::new(UiSnapshot::default()));
    {
        let mut snap = snapshot.borrow_mut();
        snap.open_case("Foundation Demo Case", 2, 5);
        snap.set_bookmark_count(1);
        snap.evidence_files = vec![
            EvidenceFileRow {
                path: "/evidence/disk.E01".into(),
                name: "disk.E01".into(),
                size: 4_294_967_296,
                deleted: false,
            },
            EvidenceFileRow {
                path: "/evidence/unalloc.bin".into(),
                name: "unalloc.bin".into(),
                size: 1024,
                deleted: true,
            },
        ];
        snap.timeline_labels = vec![
            "2026-07-17 10:02:11 | FILE | disk.E01 mounted".into(),
            "2026-07-17 10:05:44 | ART  | Chrome History parsed".into(),
        ];
        snap.findings = vec![FindingRow {
            claim: "Browser history indicates overnight activity".into(),
            bookmark_uuid: "bm-001-demo".into(),
        }];
        snap.report_state = "draft".into();
    }

    let apply = |ui: &AppWindow, snap: &UiSnapshot| {
        ui.set_case_title(snap.case_title.clone().into());
        ui.set_case_state_label(snap.case_state.as_str().into());
        ui.set_evidence_count(snap.evidence_count);
        ui.set_coverage_count(snap.coverage_count);
        ui.set_bookmark_count(snap.bookmark_count);
        ui.set_open_case_focused(snap.open_case_focused);
        ui.set_active_screen(snap.active_screen.label().into());
        ui.set_about_line(snap.about_disclosure.clone().into());
        ui.set_dark_mode(snap.dark_mode);
        ui.set_locale(snap.locale.clone().into());
        ui.set_report_state(snap.report_state.clone().into());
        ui.set_search_query(snap.search_query.clone().into());
        ui.set_selected_file_index(
            snap.selected_file_index
                .map(|i| i as i32)
                .unwrap_or(-1),
        );

        let names: Vec<SharedString> = snap
            .evidence_files
            .iter()
            .map(|f| f.name.clone().into())
            .collect();
        let paths: Vec<SharedString> = snap
            .evidence_files
            .iter()
            .map(|f| f.path.clone().into())
            .collect();
        let sizes: Vec<SharedString> = snap
            .evidence_files
            .iter()
            .map(|f| f.size.to_string().into())
            .collect();
        let deleted: Vec<bool> = snap.evidence_files.iter().map(|f| f.deleted).collect();
        ui.set_evidence_names(ModelRc::new(VecModel::from(names)));
        ui.set_evidence_paths(ModelRc::new(VecModel::from(paths)));
        ui.set_evidence_sizes(ModelRc::new(VecModel::from(sizes)));
        ui.set_evidence_deleted(ModelRc::new(VecModel::from(deleted)));

        let results: Vec<SharedString> = snap
            .search_results
            .iter()
            .map(|s| s.clone().into())
            .collect();
        ui.set_search_results(ModelRc::new(VecModel::from(results)));

        let kinds: Vec<SharedString> = snap
            .artifact_hits
            .iter()
            .map(|h| h.kind.clone().into())
            .collect();
        let summaries: Vec<SharedString> = snap
            .artifact_hits
            .iter()
            .map(|h| h.summary.clone().into())
            .collect();
        let provenances: Vec<SharedString> = snap
            .artifact_hits
            .iter()
            .map(|h| h.provenance_ref.clone().into())
            .collect();
        ui.set_artifact_kinds(ModelRc::new(VecModel::from(kinds)));
        ui.set_artifact_summaries(ModelRc::new(VecModel::from(summaries)));
        ui.set_artifact_provenances(ModelRc::new(VecModel::from(provenances)));

        let timeline: Vec<SharedString> = snap
            .timeline_labels
            .iter()
            .map(|s| s.clone().into())
            .collect();
        ui.set_timeline_labels(ModelRc::new(VecModel::from(timeline)));

        let claims: Vec<SharedString> = snap
            .findings
            .iter()
            .map(|f| f.claim.clone().into())
            .collect();
        let uuids: Vec<SharedString> = snap
            .findings
            .iter()
            .map(|f| f.bookmark_uuid.clone().into())
            .collect();
        ui.set_finding_claims(ModelRc::new(VecModel::from(claims)));
        ui.set_finding_uuids(ModelRc::new(VecModel::from(uuids)));
    };
    apply(&ui, &snapshot.borrow());

    let ui_weak = ui.as_weak();
    let snap_open = Rc::clone(&snapshot);
    ui.on_open_case_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_open.borrow_mut();
            let evidence = snap.evidence_count.max(1);
            let coverage = snap.coverage_count.max(1);
            snap.open_case("Opened Case", evidence, coverage);
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
            let n = snap.evidence_count;
            snap.evidence_files.push(EvidenceFileRow {
                path: format!("/evidence/import-{n}.bin"),
                name: format!("import-{n}.bin"),
                size: 4096,
                deleted: false,
            });
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_theme = Rc::clone(&snapshot);
    ui.on_theme_toggled(move |dark| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_theme.borrow_mut();
            snap.set_dark_mode(dark);
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_locale = Rc::clone(&snapshot);
    ui.on_locale_toggled(move |loc| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_locale.borrow_mut();
            snap.set_locale(loc.as_str());
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_search = Rc::clone(&snapshot);
    ui.on_search_submitted(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_search.borrow_mut();
            let q = ui.get_search_query().to_string();
            snap.set_search(
                q.clone(),
                vec![
                    format!("hit: browser history matching '{q}'"),
                    format!("hit: prefetch matching '{q}'"),
                ],
            );
            snap.set_artifact_hits(vec![
                ArtifactHitRow {
                    kind: "Browser".into(),
                    summary: format!("History entry for '{q}'"),
                    provenance_ref: "art://chrome/history#1".into(),
                },
                ArtifactHitRow {
                    kind: "Prefetch".into(),
                    summary: format!("Prefetch hit for '{q}'"),
                    provenance_ref: "art://prefetch#2".into(),
                },
            ]);
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_sel = Rc::clone(&snapshot);
    ui.on_evidence_row_selected(move |index| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_sel.borrow_mut();
            if index >= 0 {
                snap.select_file(index as usize);
            }
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
