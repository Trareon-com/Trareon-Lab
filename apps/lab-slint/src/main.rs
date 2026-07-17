#[cfg(feature = "gui")]
fn main() -> Result<(), slint::PlatformError> {
    use lab_slint::{sellable_disclosure, AppWindow, LabSession, NavScreen, UiSnapshot};
    use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
    use std::cell::RefCell;
    use std::rc::Rc;

    type ApplyFn = dyn Fn(&AppWindow, &UiSnapshot);

    let ui = AppWindow::new()?;
    let snapshot = Rc::new(RefCell::new(UiSnapshot {
        about_disclosure: sellable_disclosure(),
        ..UiSnapshot::default()
    }));
    let session: Rc<RefCell<Option<LabSession>>> = Rc::new(RefCell::new(None));
    let status = Rc::new(RefCell::new(String::from(
        "Open or create a case folder to begin.",
    )));

    let apply: Rc<ApplyFn> = Rc::new({
        let status = Rc::clone(&status);
        move |ui: &AppWindow, snap: &UiSnapshot| {
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
            ui.set_selected_file_index(snap.selected_file_index.map(|i| i as i32).unwrap_or(-1));
            ui.set_status_message(status.borrow().clone().into());

            let activity: Vec<SharedString> =
                if snap.case_title.is_empty() || snap.case_title == "(no case)" {
                    Vec::new()
                } else {
                    let mut lines = vec![SharedString::from(status.borrow().clone())];
                    if snap.evidence_count > 0 {
                        lines.push(SharedString::from(format!(
                            "evidence objects · {}",
                            snap.evidence_count
                        )));
                    }
                    if snap.coverage_count > 0 {
                        lines.push(SharedString::from(format!(
                            "coverage records · {}",
                            snap.coverage_count
                        )));
                    }
                    if snap.bookmark_count > 0 {
                        lines.push(SharedString::from(format!(
                            "bookmarks · {}",
                            snap.bookmark_count
                        )));
                    }
                    lines
                };
            ui.set_activity_lines(ModelRc::new(VecModel::from(activity)));

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
        }
    });
    apply(&ui, &snapshot.borrow());

    let ui_weak = ui.as_weak();
    let snap_open = Rc::clone(&snapshot);
    let sess_open = Rc::clone(&session);
    let status_open = Rc::clone(&status);
    let apply_open = apply.clone();
    ui.on_open_case_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let Some(folder) = rfd::FileDialog::new()
                .set_title("Open or create case folder")
                .pick_folder()
            else {
                *status_open.borrow_mut() = "Open case cancelled.".into();
                apply_open(&ui, &snap_open.borrow());
                return;
            };
            match LabSession::open_or_create(&folder) {
                Ok(lab) => {
                    let mut snap = snap_open.borrow_mut();
                    match lab.refresh_snapshot(&mut snap) {
                        Ok(()) => {
                            *status_open.borrow_mut() = format!("Case open · {}", folder.display());
                            *sess_open.borrow_mut() = Some(lab);
                        }
                        Err(e) => {
                            *status_open.borrow_mut() = format!("Failed to refresh case: {e}");
                        }
                    }
                    apply_open(&ui, &snap);
                }
                Err(e) => {
                    *status_open.borrow_mut() = format!("Failed to open case: {e}");
                    apply_open(&ui, &snap_open.borrow());
                }
            }
        }
    });

    let ui_weak = ui.as_weak();
    let snap_nav = Rc::clone(&snapshot);
    let apply_nav = apply.clone();
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
            apply_nav(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_import = Rc::clone(&snapshot);
    let sess_import = Rc::clone(&session);
    let status_import = Rc::clone(&status);
    let apply_import = apply.clone();
    ui.on_import_evidence_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            if sess_import.borrow().is_none() {
                *status_import.borrow_mut() =
                    "Open a case folder before importing evidence.".into();
                apply_import(&ui, &snap_import.borrow());
                return;
            }
            let Some(path) = rfd::FileDialog::new()
                .set_title("Import raw/dd disk image")
                .add_filter("Raw disk image", &["raw", "dd", "img", "bin"])
                .pick_file()
            else {
                *status_import.borrow_mut() = "Import cancelled.".into();
                apply_import(&ui, &snap_import.borrow());
                return;
            };
            let mut sess = sess_import.borrow_mut();
            let Some(lab) = sess.as_mut() else {
                return;
            };
            match lab.import_raw_path(&path) {
                Ok(name) => {
                    let mut snap = snap_import.borrow_mut();
                    if let Err(e) = lab.reload_evidence_into(&mut snap) {
                        *status_import.borrow_mut() =
                            format!("Imported {name}, refresh failed: {e}");
                    } else {
                        *status_import.borrow_mut() =
                            format!("Imported {name} · hashed and registered with provenance");
                    }
                    apply_import(&ui, &snap);
                }
                Err(e) => {
                    *status_import.borrow_mut() = format!("Import failed: {e}");
                    apply_import(&ui, &snap_import.borrow());
                }
            }
        }
    });

    let ui_weak = ui.as_weak();
    let snap_theme = Rc::clone(&snapshot);
    let apply_theme = apply.clone();
    ui.on_theme_toggled(move |dark| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_theme.borrow_mut();
            snap.set_dark_mode(dark);
            apply_theme(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_locale = Rc::clone(&snapshot);
    let apply_locale = apply.clone();
    ui.on_locale_toggled(move |loc| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_locale.borrow_mut();
            snap.set_locale(loc.as_str());
            apply_locale(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_search = Rc::clone(&snapshot);
    let sess_search = Rc::clone(&session);
    let status_search = Rc::clone(&status);
    let apply_search = apply.clone();
    ui.on_search_submitted(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let q = ui.get_search_query().to_string();
            let sess = sess_search.borrow();
            let Some(lab) = sess.as_ref() else {
                *status_search.borrow_mut() = "Open a case before searching.".into();
                apply_search(&ui, &snap_search.borrow());
                return;
            };
            match lab.search(&q) {
                Ok((results, artifacts)) => {
                    let mut snap = snap_search.borrow_mut();
                    snap.set_search(q.clone(), results);
                    snap.artifact_hits = artifacts;
                    *status_search.borrow_mut() =
                        format!("Search “{q}” · {} hit(s)", snap.artifact_hits.len());
                    apply_search(&ui, &snap);
                }
                Err(e) => {
                    *status_search.borrow_mut() = format!("Search failed: {e}");
                    apply_search(&ui, &snap_search.borrow());
                }
            }
        }
    });

    let ui_weak = ui.as_weak();
    let snap_sel = Rc::clone(&snapshot);
    let apply_sel = apply.clone();
    ui.on_evidence_row_selected(move |index| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_sel.borrow_mut();
            if index >= 0 {
                snap.select_file(index as usize);
            }
            apply_sel(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_bm = Rc::clone(&snapshot);
    let sess_bm = Rc::clone(&session);
    let status_bm = Rc::clone(&status);
    let apply_bm = apply.clone();
    ui.on_bookmark_selection_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let sess = sess_bm.borrow();
            let Some(lab) = sess.as_ref() else {
                *status_bm.borrow_mut() = "Open a case before bookmarking.".into();
                apply_bm(&ui, &snap_bm.borrow());
                return;
            };
            let citation = {
                let snap = snap_bm.borrow();
                if let Some(idx) = snap.selected_file_index {
                    snap.evidence_files
                        .get(idx)
                        .map(|f| format!("Evidence: {}", f.name))
                        .unwrap_or_else(|| "Evidence selection".into())
                } else if let Some(hit) = snap.artifact_hits.first() {
                    format!("Index hit: {}", hit.summary)
                } else {
                    "Case bookmark".into()
                }
            };
            if let Err(e) = lab.bookmark_selection(&snap_bm.borrow(), citation) {
                *status_bm.borrow_mut() = format!("Bookmark failed: {e}");
                apply_bm(&ui, &snap_bm.borrow());
                return;
            }
            let mut snap = snap_bm.borrow_mut();
            let _ = lab.reload_bookmarks_into(&mut snap);
            snap.navigate_to(NavScreen::Bookmarks);
            *status_bm.borrow_mut() = "Bookmark saved to case DB.".into();
            apply_bm(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_rel = Rc::clone(&snapshot);
    let sess_rel = Rc::clone(&session);
    let status_rel = Rc::clone(&status);
    let apply_rel = apply.clone();
    ui.on_find_related_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let needle = {
                let snap = snap_rel.borrow();
                snap.selected_file_index
                    .and_then(|i| snap.evidence_files.get(i).map(|f| f.name.clone()))
                    .unwrap_or_default()
            };
            if needle.is_empty() {
                *status_rel.borrow_mut() = "Select an evidence row first.".into();
                apply_rel(&ui, &snap_rel.borrow());
                return;
            }
            ui.set_search_query(needle.clone().into());
            let sess = sess_rel.borrow();
            let Some(lab) = sess.as_ref() else {
                let mut snap = snap_rel.borrow_mut();
                snap.navigate_to(NavScreen::Search);
                snap.search_query = needle;
                *status_rel.borrow_mut() = "Open a case before searching.".into();
                apply_rel(&ui, &snap);
                return;
            };
            match lab.search(&needle) {
                Ok((results, artifacts)) => {
                    let mut snap = snap_rel.borrow_mut();
                    snap.set_search(needle.clone(), results);
                    snap.artifact_hits = artifacts;
                    *status_rel.borrow_mut() = format!(
                        "Related index hits for “{needle}” · {}",
                        snap.artifact_hits.len()
                    );
                    apply_rel(&ui, &snap);
                }
                Err(e) => {
                    *status_rel.borrow_mut() = format!("Find related failed: {e}");
                    apply_rel(&ui, &snap_rel.borrow());
                }
            }
        }
    });

    let ui_weak = ui.as_weak();
    let snap_hs = Rc::clone(&snapshot);
    let apply_hs = apply.clone();
    ui.on_header_search_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_hs.borrow_mut();
            snap.handle_shortcut("/");
            apply_hs(&ui, &snap);
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
