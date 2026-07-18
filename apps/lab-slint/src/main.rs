#[cfg(feature = "gui")]
fn main() -> Result<(), slint::PlatformError> {
    use lab_slint::{AppWindow, FindingRow, LabSession, NavScreen, UiSnapshot};
    use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;

    let ui = AppWindow::new()?;
    // M0 honesty: default snapshot is empty — no fabricated case/search/import.
    let snapshot = Rc::new(RefCell::new(UiSnapshot::default()));
    let session: Rc<RefCell<Option<LabSession>>> = Rc::new(RefCell::new(None));

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
        ui.set_selected_file_index(snap.selected_file_index.map(|i| i as i32).unwrap_or(-1));
        ui.set_disclosure_line(snap.disclosure_line.clone().into());
        ui.set_coverage_status_label(snap.coverage_status.as_str().into());
        ui.set_report_finalizable(snap.report_finalizable);
        ui.set_list_total(snap.list_total);
        ui.set_list_offset(snap.list_offset);
        ui.set_list_page_size(snap.list_page_size);
        ui.set_hex_status(snap.hex_status.clone().into());

        let blockers: Vec<SharedString> = snap
            .report_blockers
            .iter()
            .map(|s| s.clone().into())
            .collect();
        ui.set_report_blockers(ModelRc::new(VecModel::from(blockers)));

        let activity = vec![
            SharedString::from(format!("case · {}", snap.case_title)),
            SharedString::from(format!("{} evidence", snap.evidence_count)),
            SharedString::from(format!(
                "coverage · {} (p{}/s{}/f{})",
                snap.coverage_status.as_str(),
                snap.coverage_processed,
                snap.coverage_skipped,
                snap.coverage_failed
            )),
            SharedString::from(format!(
                "showing {}–{} of {}",
                snap.list_offset + 1,
                (snap.list_offset + snap.list_page_size).min(snap.list_total.max(0)),
                snap.list_total
            )),
            SharedString::from(format!("{} bookmark(s)", snap.bookmark_count)),
        ];
        ui.set_activity_lines(ModelRc::new(VecModel::from(activity)));

        let (start, end) = snap.page_slice_indices();
        let page = &snap.evidence_files[start..end];
        let names: Vec<SharedString> = page.iter().map(|f| f.name.clone().into()).collect();
        let paths: Vec<SharedString> = page.iter().map(|f| f.path.clone().into()).collect();
        let sizes: Vec<SharedString> = page.iter().map(|f| f.size.to_string().into()).collect();
        let deleted: Vec<bool> = page.iter().map(|f| f.deleted).collect();
        let integrity: Vec<SharedString> = page
            .iter()
            .map(|f| f.integrity.as_str().into())
            .collect();
        ui.set_evidence_names(ModelRc::new(VecModel::from(names)));
        ui.set_evidence_paths(ModelRc::new(VecModel::from(paths)));
        ui.set_evidence_sizes(ModelRc::new(VecModel::from(sizes)));
        ui.set_evidence_deleted(ModelRc::new(VecModel::from(deleted)));
        ui.set_evidence_integrity(ModelRc::new(VecModel::from(integrity)));

        let tree_labels: Vec<SharedString> = snap
            .tree_nodes
            .iter()
            .map(|n| n.label.clone().into())
            .collect();
        let tree_depths: Vec<i32> = snap.tree_nodes.iter().map(|n| n.depth).collect();
        let tree_kinds: Vec<SharedString> = snap
            .tree_nodes
            .iter()
            .map(|n| n.kind.clone().into())
            .collect();
        let tree_idx: Vec<i32> = snap.tree_nodes.iter().map(|n| n.file_index).collect();
        ui.set_tree_labels(ModelRc::new(VecModel::from(tree_labels)));
        ui.set_tree_depths(ModelRc::new(VecModel::from(tree_depths)));
        ui.set_tree_kinds(ModelRc::new(VecModel::from(tree_kinds)));
        ui.set_tree_file_index(ModelRc::new(VecModel::from(tree_idx)));

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

        let hex: Vec<SharedString> = snap.hex_lines.iter().map(|s| s.clone().into()).collect();
        ui.set_hex_lines(ModelRc::new(VecModel::from(hex)));

        ui.set_progress_ratio(snap.progress_ratio as f32);
        ui.set_progress_stage(snap.progress_stage.clone().into());
        ui.set_progress_message(snap.progress_message.clone().into());
        ui.set_progress_visible(snap.progress_visible);
    };
    apply(&ui, &snapshot.borrow());

    let ui_weak = ui.as_weak();
    let snap_open = Rc::clone(&snapshot);
    let sess_open = Rc::clone(&session);
    ui.on_open_case_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_open.borrow_mut();
            let case_dir = std::env::var("TRAREON_CASE_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    std::env::temp_dir().join(format!(
                        "trareon-lab-case-{}",
                        std::process::id()
                    ))
                });
            let case_uuid = "00000000-0000-4000-8000-000000000001";
            match LabSession::create(&case_dir, case_uuid, "Untitled Case")
                .or_else(|_| LabSession::open(&case_dir, case_uuid, "Untitled Case"))
            {
                Ok(sess) => {
                    let _ = sess.sync_snapshot(&mut snap);
                    snap.intake_accepted = false;
                    *sess_open.borrow_mut() = Some(sess);
                }
                Err(e) => {
                    snap.open_case("Untitled Case", 0, 0);
                    snap.hex_status = format!("Case open Limited: {e:?}");
                }
            }
            snap.coverage_status = lab_core::CoverageStatus::NotRun;
            snap.recompute_report_gate();
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_nav = Rc::clone(&snapshot);
    ui.on_navigate(move |label| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_nav.borrow_mut();
            snap.navigate_to(NavScreen::from_label(label.as_str()));
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_import = Rc::clone(&snapshot);
    let sess_import = Rc::clone(&session);
    ui.on_import_evidence_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_import.borrow_mut();
            if snap.demo_seed {
                snap.set_progress("import", 0, Some(1), "demo_seed import");
                snap.import_evidence_stub();
                snap.set_progress("import", 1, Some(1), "demo import complete");
            } else if let Some(path) = std::env::var_os("TRAREON_IMPORT_PATH") {
                let path = PathBuf::from(path);
                snap.set_progress("import", 0, Some(1), "importing…");
                if let Some(sess) = sess_import.borrow_mut().as_mut() {
                    match sess.import_image(&path) {
                        Ok(()) => {
                            let _ = sess.sync_snapshot(&mut snap);
                            snap.set_progress("import", 1, Some(1), "import complete");
                        }
                        Err(e) => {
                            snap.hex_status = format!("Import failed: {e:?}");
                            snap.clear_progress();
                        }
                    }
                } else {
                    snap.hex_status = "Import: open a case first".into();
                    snap.clear_progress();
                }
            } else {
                snap.set_progress(
                    "import",
                    0,
                    None,
                    "Set TRAREON_IMPORT_PATH to a .dd/.e01 file (stub disabled)",
                );
                snap.hex_status =
                    "Import: TRAREON_IMPORT_PATH required (demo_seed off)".into();
                snap.clear_progress();
            }
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
    let sess_search = Rc::clone(&session);
    ui.on_search_submitted(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_search.borrow_mut();
            let q = ui.get_search_query().to_string();
            if snap.demo_seed {
                snap.set_search(q.clone(), vec![format!("hit: demo matching '{q}'")]);
            } else if let Some(sess) = sess_search.borrow_mut().as_mut() {
                if let Err(e) = sess.search(&q, &mut snap) {
                    snap.set_search(q, vec![format!("search error: {e:?}")]);
                }
            } else {
                snap.set_search(q, Vec::new());
                snap.artifact_hits.clear();
            }
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_sel = Rc::clone(&snapshot);
    let sess_sel = Rc::clone(&session);
    ui.on_evidence_row_selected(move |index| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_sel.borrow_mut();
            if index >= 0 {
                let (start, _) = snap.page_slice_indices();
                let abs = start + index as usize;
                snap.select_file(abs);
                let path = snap
                    .evidence_files
                    .get(abs)
                    .map(|f| f.path.clone())
                    .unwrap_or_default();
                if !path.is_empty() {
                    if let Some(sess) = sess_sel.borrow().as_ref() {
                        match sess.read_hex(std::path::Path::new(&path), 0, 256) {
                            Ok((lines, status)) => {
                                snap.hex_lines = lines;
                                snap.hex_status = status;
                                snap.hex_offset = 0;
                            }
                            Err(e) => {
                                snap.hex_lines =
                                    vec![format!("Error/Limited — hex read failed: {e:?}")];
                                snap.hex_status = "Error/Limited".into();
                            }
                        }
                    }
                } else if let Some(f) = snap.evidence_files.get(abs) {
                    snap.hex_status = format!(
                        "{} · {} · no local path",
                        f.name,
                        f.integrity.as_str()
                    );
                    snap.hex_lines = vec!["Status: Limited — provenance path missing".into()];
                }
            }
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_bm = Rc::clone(&snapshot);
    ui.on_bookmark_selection_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_bm.borrow_mut();
            let claim = if let Some(idx) = snap.selected_file_index {
                snap.evidence_files
                    .get(idx)
                    .map(|f| format!("Bookmarked file {}", f.name))
                    .unwrap_or_else(|| "Bookmarked selection".into())
            } else {
                "Bookmarked examination hit".into()
            };
            snap.bookmark_count += 1;
            let n = snap.bookmark_count;
            snap.findings.push(FindingRow {
                claim,
                bookmark_uuid: format!("bm-{n:03}"),
                claim_type: lab_core::ClaimType::Observation,
            });
            snap.navigate_to(NavScreen::Bookmarks);
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_rel = Rc::clone(&snapshot);
    ui.on_find_related_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_rel.borrow_mut();
            let q = snap
                .selected_file_index
                .and_then(|i| snap.evidence_files.get(i).map(|f| f.name.clone()))
                .unwrap_or_else(|| "related".into());
            snap.set_search(q, Vec::new());
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_hs = Rc::clone(&snapshot);
    ui.on_header_search_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_hs.borrow_mut();
            snap.handle_shortcut("/");
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_cancel = Rc::clone(&snapshot);
    ui.on_cancel_progress_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_cancel.borrow_mut();
            snap.request_cancel_progress();
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
