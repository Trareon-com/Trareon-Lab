#[cfg(feature = "gui")]
fn main() -> Result<(), slint::PlatformError> {
    use lab_slint::{sellable_disclosure, AppWindow, LabSession, NavScreen, UiSnapshot};
    use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
    use std::cell::RefCell;
    use std::rc::Rc;

    type ApplyFn = dyn Fn(&AppWindow, &UiSnapshot);

    const PALETTE_ALL: &[&str] = &[
        "Open Case",
        "Import Evidence",
        "Go: Home",
        "Go: Evidence",
        "Go: Search",
        "Go: Timeline",
        "Go: Bookmarks",
        "Go: Report",
        "Bookmark selection",
        "Toggle nav",
        "Toggle inspector",
        "Toggle log",
        "Toggle theme",
        "Toggle locale",
    ];

    let ui = AppWindow::new()?;
    let snapshot = Rc::new(RefCell::new(UiSnapshot {
        about_disclosure: sellable_disclosure(),
        ..UiSnapshot::default()
    }));
    let session: Rc<RefCell<Option<LabSession>>> = Rc::new(RefCell::new(None));
    let status = Rc::new(RefCell::new(String::from(
        "Open or create a case folder to begin.",
    )));
    let palette_filter = Rc::new(RefCell::new(String::new()));

    let push_status: Rc<dyn Fn(String)> = Rc::new({
        let status = Rc::clone(&status);
        let snapshot = Rc::clone(&snapshot);
        move |msg: String| {
            *status.borrow_mut() = msg.clone();
            snapshot.borrow_mut().push_log(msg);
        }
    });

    let filtered_commands: Rc<dyn Fn() -> Vec<SharedString>> = Rc::new({
        let palette_filter = Rc::clone(&palette_filter);
        move || -> Vec<SharedString> {
            let q = palette_filter.borrow().to_lowercase();
            PALETTE_ALL
                .iter()
                .filter(|c| q.is_empty() || c.to_lowercase().contains(&q))
                .map(|c| SharedString::from(*c))
                .collect()
        }
    });

    let apply: Rc<ApplyFn> = Rc::new({
        let status = Rc::clone(&status);
        let filtered_commands = filtered_commands.clone();
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
            ui.set_nav_collapsed(snap.nav_collapsed);
            ui.set_inspector_open(snap.inspector_open);
            ui.set_log_open(snap.log_open);
            ui.set_palette_open(snap.palette_open);

            let (insp_title, insp_path, insp_size, insp_del, insp_has) =
                if let Some(idx) = snap.selected_file_index {
                    if let Some(f) = snap.evidence_files.get(idx) {
                        (
                            f.name.clone(),
                            f.path.clone(),
                            f.size.to_string(),
                            f.deleted,
                            true,
                        )
                    } else {
                        (String::new(), String::new(), String::new(), false, false)
                    }
                } else if let Some(hit) = snap.artifact_hits.first() {
                    (
                        hit.summary.clone(),
                        hit.provenance_ref.clone(),
                        hit.kind.clone(),
                        false,
                        true,
                    )
                } else {
                    (String::new(), String::new(), String::new(), false, false)
                };
            ui.set_inspector_title(insp_title.into());
            ui.set_inspector_path(insp_path.into());
            ui.set_inspector_size(insp_size.into());
            ui.set_inspector_deleted(insp_del);
            ui.set_inspector_has_selection(insp_has);

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
                    if snap.bookmark_count > 0 {
                        lines.push(SharedString::from(format!(
                            "bookmarks · {}",
                            snap.bookmark_count
                        )));
                    }
                    lines
                };
            ui.set_activity_lines(ModelRc::new(VecModel::from(activity)));

            let logs: Vec<SharedString> = snap.log_lines.iter().map(|s| s.clone().into()).collect();
            ui.set_log_lines(ModelRc::new(VecModel::from(logs)));
            ui.set_palette_commands(ModelRc::new(VecModel::from(filtered_commands())));

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

    let do_import: Rc<dyn Fn(&AppWindow)> = Rc::new({
        let snapshot = Rc::clone(&snapshot);
        let session = Rc::clone(&session);
        let apply = apply.clone();
        let push_status = Rc::clone(&push_status);
        move |ui: &AppWindow| {
            if session.borrow().is_none() {
                push_status("Open a case folder before importing evidence.".into());
                apply(ui, &snapshot.borrow());
                return;
            }
            let Some(path) = rfd::FileDialog::new()
                .set_title("Import raw/dd disk image")
                .add_filter("Raw disk image", &["raw", "dd", "img", "bin"])
                .pick_file()
            else {
                push_status("Import cancelled.".into());
                apply(ui, &snapshot.borrow());
                return;
            };
            push_status(format!("Hashing {}…", path.display()));
            apply(ui, &snapshot.borrow());
            let mut sess = session.borrow_mut();
            let Some(lab) = sess.as_mut() else {
                return;
            };
            match lab.import_raw_path(&path) {
                Ok(name) => {
                    let mut snap = snapshot.borrow_mut();
                    if let Err(e) = lab.reload_evidence_into(&mut snap) {
                        drop(snap);
                        push_status(format!("Imported {name}, refresh failed: {e}"));
                    } else {
                        snap.inspector_open = true;
                        drop(snap);
                        push_status(format!(
                            "Imported {name} · hashed and registered with provenance"
                        ));
                    }
                    apply(ui, &snapshot.borrow());
                }
                Err(e) => {
                    push_status(format!("Import failed: {e}"));
                    apply(ui, &snapshot.borrow());
                }
            }
        }
    });

    let ui_weak = ui.as_weak();
    let snap_open = Rc::clone(&snapshot);
    let sess_open = Rc::clone(&session);
    let apply_open = apply.clone();
    let push_open = Rc::clone(&push_status);
    ui.on_open_case_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let Some(folder) = rfd::FileDialog::new()
                .set_title("Open or create case folder")
                .pick_folder()
            else {
                push_open("Open case cancelled.".into());
                apply_open(&ui, &snap_open.borrow());
                return;
            };
            match LabSession::open_or_create(&folder) {
                Ok(lab) => {
                    let mut snap = snap_open.borrow_mut();
                    match lab.refresh_snapshot(&mut snap) {
                        Ok(()) => {
                            *sess_open.borrow_mut() = Some(lab);
                            drop(snap);
                            push_open(format!("Case open · {}", folder.display()));
                        }
                        Err(e) => {
                            drop(snap);
                            push_open(format!("Failed to refresh case: {e}"));
                        }
                    }
                    apply_open(&ui, &snap_open.borrow());
                }
                Err(e) => {
                    push_open(format!("Failed to open case: {e}"));
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
    let do_import_cb = Rc::clone(&do_import);
    ui.on_import_evidence_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            do_import_cb(&ui);
        }
    });
    let ui_weak = ui.as_weak();
    let do_import_drop = Rc::clone(&do_import);
    ui.on_drop_import_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            do_import_drop(&ui);
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
    let apply_search = apply.clone();
    let push_search = Rc::clone(&push_status);
    ui.on_search_submitted(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let q = ui.get_search_query().to_string();
            let sess = sess_search.borrow();
            let Some(lab) = sess.as_ref() else {
                push_search("Open a case before searching.".into());
                apply_search(&ui, &snap_search.borrow());
                return;
            };
            match lab.search(&q) {
                Ok((results, artifacts)) => {
                    let mut snap = snap_search.borrow_mut();
                    snap.set_search(q.clone(), results);
                    snap.artifact_hits = artifacts;
                    let n = snap.artifact_hits.len();
                    drop(snap);
                    push_search(format!("Search “{q}” · {n} hit(s)"));
                    apply_search(&ui, &snap_search.borrow());
                }
                Err(e) => {
                    push_search(format!("Search failed: {e}"));
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
                snap.inspector_open = true;
            }
            apply_sel(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_bm = Rc::clone(&snapshot);
    let sess_bm = Rc::clone(&session);
    let apply_bm = apply.clone();
    let push_bm = Rc::clone(&push_status);
    ui.on_bookmark_selection_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let sess = sess_bm.borrow();
            let Some(lab) = sess.as_ref() else {
                push_bm("Open a case before bookmarking.".into());
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
                push_bm(format!("Bookmark failed: {e}"));
                apply_bm(&ui, &snap_bm.borrow());
                return;
            }
            let mut snap = snap_bm.borrow_mut();
            let _ = lab.reload_bookmarks_into(&mut snap);
            snap.navigate_to(NavScreen::Bookmarks);
            drop(snap);
            push_bm("Bookmark saved to case DB.".into());
            apply_bm(&ui, &snap_bm.borrow());
        }
    });

    let ui_weak = ui.as_weak();
    let snap_rel = Rc::clone(&snapshot);
    let sess_rel = Rc::clone(&session);
    let apply_rel = apply.clone();
    let push_rel = Rc::clone(&push_status);
    ui.on_find_related_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let needle = {
                let snap = snap_rel.borrow();
                snap.selected_file_index
                    .and_then(|i| snap.evidence_files.get(i).map(|f| f.name.clone()))
                    .unwrap_or_default()
            };
            if needle.is_empty() {
                push_rel("Select an evidence row first.".into());
                apply_rel(&ui, &snap_rel.borrow());
                return;
            }
            ui.set_search_query(needle.clone().into());
            let sess = sess_rel.borrow();
            let Some(lab) = sess.as_ref() else {
                let mut snap = snap_rel.borrow_mut();
                snap.navigate_to(NavScreen::Search);
                snap.search_query = needle;
                drop(snap);
                push_rel("Open a case before searching.".into());
                apply_rel(&ui, &snap_rel.borrow());
                return;
            };
            match lab.search(&needle) {
                Ok((results, artifacts)) => {
                    let mut snap = snap_rel.borrow_mut();
                    snap.set_search(needle.clone(), results);
                    snap.artifact_hits = artifacts;
                    let n = snap.artifact_hits.len();
                    drop(snap);
                    push_rel(format!("Related index hits for “{needle}” · {n}"));
                    apply_rel(&ui, &snap_rel.borrow());
                }
                Err(e) => {
                    push_rel(format!("Find related failed: {e}"));
                    apply_rel(&ui, &snap_rel.borrow());
                }
            }
        }
    });

    let ui_weak = ui.as_weak();
    let snap_hs = Rc::clone(&snapshot);
    let apply_hs = apply.clone();
    let pf = Rc::clone(&palette_filter);
    ui.on_header_search_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            pf.borrow_mut().clear();
            let mut snap = snap_hs.borrow_mut();
            snap.handle_shortcut("/");
            apply_hs(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_sc = Rc::clone(&snapshot);
    let apply_sc = apply.clone();
    ui.on_shortcut_triggered(move |key| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_sc.borrow_mut();
            snap.handle_shortcut(key.as_str());
            apply_sc(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_tn = Rc::clone(&snapshot);
    let apply_tn = apply.clone();
    ui.on_toggle_nav_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_tn.borrow_mut();
            snap.handle_shortcut("nav");
            apply_tn(&ui, &snap);
        }
    });
    let ui_weak = ui.as_weak();
    let snap_ti = Rc::clone(&snapshot);
    let apply_ti = apply.clone();
    ui.on_toggle_inspector_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_ti.borrow_mut();
            snap.handle_shortcut("inspector");
            apply_ti(&ui, &snap);
        }
    });
    let ui_weak = ui.as_weak();
    let snap_tl = Rc::clone(&snapshot);
    let apply_tl = apply.clone();
    ui.on_toggle_log_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_tl.borrow_mut();
            snap.handle_shortcut("log");
            apply_tl(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let pf2 = Rc::clone(&palette_filter);
    let apply_pf = apply.clone();
    let snap_pf = Rc::clone(&snapshot);
    ui.on_palette_filter_changed(move |t| {
        if let Some(ui) = ui_weak.upgrade() {
            *pf2.borrow_mut() = t.to_string();
            apply_pf(&ui, &snap_pf.borrow());
        }
    });

    let ui_weak = ui.as_weak();
    let snap_cmd = Rc::clone(&snapshot);
    let apply_cmd = apply.clone();
    let do_import_cmd = Rc::clone(&do_import);
    ui.on_command_activated(move |cmd| {
        if let Some(ui) = ui_weak.upgrade() {
            let c = cmd.to_string();
            {
                let mut snap = snap_cmd.borrow_mut();
                snap.palette_open = false;
            }
            match c.as_str() {
                "Open Case" => ui.invoke_open_case_clicked(),
                "Import Evidence" => do_import_cmd(&ui),
                "Go: Home" => snap_cmd.borrow_mut().navigate_to(NavScreen::CaseHome),
                "Go: Evidence" => snap_cmd.borrow_mut().navigate_to(NavScreen::Evidence),
                "Go: Search" => snap_cmd.borrow_mut().navigate_to(NavScreen::Search),
                "Go: Timeline" => snap_cmd.borrow_mut().navigate_to(NavScreen::Timeline),
                "Go: Bookmarks" => snap_cmd.borrow_mut().navigate_to(NavScreen::Bookmarks),
                "Go: Report" => snap_cmd.borrow_mut().navigate_to(NavScreen::Report),
                "Bookmark selection" => ui.invoke_bookmark_selection_clicked(),
                "Toggle nav" => snap_cmd.borrow_mut().handle_shortcut("nav"),
                "Toggle inspector" => snap_cmd.borrow_mut().handle_shortcut("inspector"),
                "Toggle log" => snap_cmd.borrow_mut().handle_shortcut("log"),
                "Toggle theme" => {
                    let mut s = snap_cmd.borrow_mut();
                    let next = !s.dark_mode;
                    s.set_dark_mode(next);
                }
                "Toggle locale" => {
                    let mut s = snap_cmd.borrow_mut();
                    let next = if s.locale == "en" { "id" } else { "en" };
                    s.set_locale(next);
                }
                _ => {}
            }
            apply_cmd(&ui, &snap_cmd.borrow());
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
