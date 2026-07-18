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
        {
            // ponytail: UTC clock from epoch; local TZ needs a time crate later
            use std::time::{SystemTime, UNIX_EPOCH};
            let secs = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let h = (secs / 3600) % 24;
            let m = (secs / 60) % 60;
            let s = secs % 60;
            let _ = (h, m, s); // clock folded into last_action / status; keep UTC tick for future
            ui.set_footer_clock(format!("{h:02}:{m:02}:{s:02} UTC").into());
        }
        ui.set_evidence_count(snap.evidence_count);
        ui.set_coverage_count(snap.coverage_count);
        ui.set_bookmark_count(snap.bookmark_count);
        ui.set_open_case_focused(snap.open_case_focused);
        ui.set_active_screen(snap.active_screen.label().into());
        ui.set_about_line(snap.about_disclosure.clone().into());
        ui.set_version_line(format!("Trareon Lab {}", env!("CARGO_PKG_VERSION")).into());
        ui.set_last_action(snap.last_action.clone().into());
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
        ui.set_inspector_tab(snap.inspector_tab.clone().into());
        ui.set_transfer_status(snap.transfer_status.clone().into());
        ui.set_transfer_trust_label(snap.transfer_trust_label.clone().into());
        let cap_ids: Vec<SharedString> = snap
            .capabilities
            .iter()
            .map(|(id, _, _)| id.clone().into())
            .collect();
        let cap_st: Vec<SharedString> = snap
            .capabilities
            .iter()
            .map(|(_, st, _)| st.clone().into())
            .collect();
        let cap_notes: Vec<SharedString> = snap
            .capabilities
            .iter()
            .map(|(_, _, n)| n.clone().into())
            .collect();
        ui.set_capability_ids(ModelRc::new(VecModel::from(cap_ids)));
        ui.set_capability_statuses(ModelRc::new(VecModel::from(cap_st)));
        ui.set_capability_notes(ModelRc::new(VecModel::from(cap_notes)));
        let edges: Vec<SharedString> = snap.graph_edges.iter().map(|s| s.clone().into()).collect();
        ui.set_graph_edges(ModelRc::new(VecModel::from(edges)));

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
        let integrity: Vec<SharedString> =
            page.iter().map(|f| f.integrity.as_str().into()).collect();
        ui.set_evidence_names(ModelRc::new(VecModel::from(names)));
        ui.set_evidence_paths(ModelRc::new(VecModel::from(paths)));
        ui.set_evidence_sizes(ModelRc::new(VecModel::from(sizes)));
        ui.set_evidence_deleted(ModelRc::new(VecModel::from(deleted)));
        ui.set_evidence_integrity(ModelRc::new(VecModel::from(integrity)));
        let designations: Vec<SharedString> =
            page.iter().map(|f| f.designation.clone().into()).collect();
        ui.set_evidence_designations(ModelRc::new(VecModel::from(designations)));

        ui.set_nav_collapsed(snap.nav_collapsed);
        ui.set_layout_compact(snap.layout_compact);
        ui.set_nav_expanded(snap.nav_expanded());
        ui.set_inspector_as_side(snap.inspector_as_side());
        ui.set_inspector_as_overlay(snap.inspector_as_overlay());
        ui.set_inspector_open(snap.inspector_open);
        ui.set_log_open(snap.log_open);
        ui.set_palette_open(snap.palette_open);
        ui.set_cheatsheet_open(snap.cheatsheet_open);
        ui.set_search_coverage_label(snap.search_coverage_label.clone().into());
        ui.set_export_status(snap.export_status.clone().into());
        ui.set_placeholder_title(snap.placeholder_title.clone().into());
        ui.set_placeholder_body(snap.placeholder_body.clone().into());
        let carve_hits: Vec<SharedString> = snap
            .carve_hit_lines
            .iter()
            .map(|s| s.clone().into())
            .collect();
        ui.set_carve_hit_lines(ModelRc::new(VecModel::from(carve_hits)));
        let qv_meta: Vec<SharedString> = snap
            .quick_verify_meta_lines
            .iter()
            .map(|s| s.clone().into())
            .collect();
        ui.set_quick_verify_meta_lines(ModelRc::new(VecModel::from(qv_meta)));
        let qv_tl: Vec<SharedString> = snap
            .quick_verify_timeline_lines
            .iter()
            .map(|s| s.clone().into())
            .collect();
        ui.set_quick_verify_timeline_lines(ModelRc::new(VecModel::from(qv_tl)));

        let exceptions: Vec<SharedString> = snap
            .exception_lines
            .iter()
            .map(|s| s.clone().into())
            .collect();
        ui.set_exception_lines(ModelRc::new(VecModel::from(exceptions)));
        let runs: Vec<SharedString> = snap
            .run_compare_lines
            .iter()
            .map(|s| s.clone().into())
            .collect();
        ui.set_run_lines(ModelRc::new(VecModel::from(runs)));
        let logs: Vec<SharedString> = snap.log_lines.iter().map(|s| s.clone().into()).collect();
        ui.set_log_lines(ModelRc::new(VecModel::from(logs)));
        let cmds: Vec<SharedString> = snap
            .palette_commands
            .iter()
            .map(|s| s.clone().into())
            .collect();
        ui.set_palette_commands(ModelRc::new(VecModel::from(cmds)));
        let yara: Vec<SharedString> = snap
            .yara_hit_lines
            .iter()
            .map(|s| s.clone().into())
            .collect();
        ui.set_yara_hit_lines(ModelRc::new(VecModel::from(yara)));
        let hashset: Vec<SharedString> = snap
            .hashset_hit_lines
            .iter()
            .map(|s| s.clone().into())
            .collect();
        ui.set_hashset_hit_lines(ModelRc::new(VecModel::from(hashset)));

        if let Some(f) = snap.selected_file() {
            ui.set_inspector_title(f.name.clone().into());
            ui.set_inspector_path(f.path.clone().into());
            ui.set_inspector_size(f.size.to_string().into());
            ui.set_inspector_designation(f.designation.clone().into());
            ui.set_inspector_integrity(f.integrity.as_str().into());
            ui.set_inspector_deleted(f.deleted);
            ui.set_inspector_tagged(snap.bookmark_count > 0 && snap.selected_file_index.is_some());
        } else {
            ui.set_inspector_title("".into());
            ui.set_inspector_path("".into());
            ui.set_inspector_size("".into());
            ui.set_inspector_designation("".into());
            ui.set_inspector_integrity("".into());
            ui.set_inspector_deleted(false);
            ui.set_inspector_tagged(false);
        }

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
    {
        let mut snap = snapshot.borrow_mut();
        let w = ui.window().size().width as i32;
        snap.apply_layout_width(if w > 0 { w } else { 1280 });
    }
    apply(&ui, &snapshot.borrow());

    let ui_weak = ui.as_weak();
    let snap_open = Rc::clone(&snapshot);
    let sess_open = Rc::clone(&session);
    ui.on_open_case_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_open.borrow_mut();
            // Prefer folder picker; TRAREON_CASE_DIR remains for headless/automation.
            let case_dir = std::env::var_os("TRAREON_CASE_DIR")
                .map(PathBuf::from)
                .or_else(|| {
                    rfd::FileDialog::new()
                        .set_title("Open or create case folder")
                        .pick_folder()
                });
            let Some(case_dir) = case_dir else {
                snap.hex_status = "Open case cancelled".into();
                apply(&ui, &snap);
                return;
            };
            match LabSession::open_or_create(&case_dir) {
                Ok(sess) => {
                    let _ = sess.sync_snapshot(&mut snap);
                    snap.intake_accepted = false;
                    snap.prefs_last_case = case_dir.display().to_string();
                    if let Some(home) = dirs_next_home() {
                        let prefs = home.join(".trareon-lab");
                        let _ = std::fs::create_dir_all(&prefs);
                        let _ = std::fs::write(
                            prefs.join("prefs.txt"),
                            snap.prefs_last_case.as_bytes(),
                        );
                    }
                    *sess_open.borrow_mut() = Some(sess);
                    let status = format!("Case open · {}", case_dir.display());
                    snap.hex_status = status.clone();
                    snap.push_log(status);
                }
                Err(e) => {
                    snap.open_case("Untitled Case", 0, 0);
                    snap.hex_status = format!("Case open Limited: {e:?}");
                }
            }
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
                apply(&ui, &snap);
                return;
            }
            if sess_import.borrow().is_none() {
                snap.hex_status = "Import: open a case first".into();
                snap.clear_progress();
                apply(&ui, &snap);
                return;
            }
            let path = std::env::var_os("TRAREON_IMPORT_PATH")
                .map(PathBuf::from)
                .or_else(|| {
                    rfd::FileDialog::new()
                        .set_title("Import disk image")
                        .add_filter("Disk image", &["raw", "dd", "img", "bin", "e01"])
                        .pick_file()
                });
            let Some(path) = path else {
                snap.hex_status = "Import cancelled".into();
                snap.clear_progress();
                apply(&ui, &snap);
                return;
            };
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
                snap.inspector_open = true;
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
                    snap.hex_status =
                        format!("{} · {} · no local path", f.name, f.integrity.as_str());
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
            snap.add_graph_edge(&q, "related-search");
            snap.set_search(q, Vec::new());
            snap.push_log("find related · edge recorded");
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

    let ui_weak = ui.as_weak();
    let snap_pf = Rc::clone(&snapshot);
    ui.on_palette_filter_changed(move |f| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_pf.borrow_mut();
            snap.filter_palette(f.as_str());
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_pc = Rc::clone(&snapshot);
    ui.on_palette_command_activated(move |cmd| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_pc.borrow_mut();
            let c = cmd.to_string();
            if c == "Open Case" {
                drop(snap);
                ui.invoke_open_case_clicked();
                return;
            }
            if c == "Import Evidence" {
                drop(snap);
                ui.invoke_import_evidence_clicked();
                return;
            }
            if c == "Quick Verify" {
                drop(snap);
                ui.invoke_quick_verify_clicked();
                return;
            }
            if c == "Load Demo Case" {
                drop(snap);
                ui.invoke_load_demo_clicked();
                return;
            }
            if c == "Run Carving" {
                drop(snap);
                ui.invoke_run_carving_clicked();
                return;
            }
            snap.activate_palette_command(&c);
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_pd = Rc::clone(&snapshot);
    ui.on_palette_dismissed(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_pd.borrow_mut();
            snap.palette_open = false;
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_ti = Rc::clone(&snapshot);
    ui.on_toggle_inspector_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_ti.borrow_mut();
            snap.handle_shortcut("inspector");
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_tl = Rc::clone(&snapshot);
    ui.on_toggle_log_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_tl.borrow_mut();
            snap.handle_shortcut("log");
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_tn = Rc::clone(&snapshot);
    ui.on_toggle_nav_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_tn.borrow_mut();
            snap.handle_shortcut("nav");
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_ex = Rc::clone(&snapshot);
    let sess_ex = Rc::clone(&session);
    ui.on_export_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_ex.borrow_mut();
            if let Some(sess) = sess_ex.borrow().as_ref() {
                match sess.export_formats(lab_core::ExportMode::Draft) {
                    Ok(parts) => {
                        snap.export_status = parts
                            .iter()
                            .map(|(k, d)| format!("{k}={d}"))
                            .collect::<Vec<_>>()
                            .join("; ");
                        snap.push_log("export · formats written (digests)");
                    }
                    Err(e) => {
                        snap.export_status = format!("export failed: {e:?}");
                    }
                }
            } else {
                snap.export_status = "export: open a case first".into();
            }
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_tr = Rc::clone(&snapshot);
    let sess_tr = Rc::clone(&session);
    ui.on_transfer_export_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_tr.borrow_mut();
            if let Some(sess) = sess_tr.borrow_mut().as_mut() {
                match sess.export_transfer_package("ui") {
                    Ok(trust) => {
                        snap.transfer_status = "package exported".into();
                        snap.transfer_trust_label = format!("{trust:?}");
                        snap.set_transfer_status("package exported");
                        snap.push_log(format!("transfer · {trust:?}"));
                        snap.navigate_to(NavScreen::Transfer);
                    }
                    Err(e) => {
                        snap.transfer_status = format!("transfer failed: {e:?}");
                        snap.navigate_to(NavScreen::Transfer);
                    }
                }
            } else {
                snap.transfer_status = "transfer: open a case first".into();
                snap.navigate_to(NavScreen::Transfer);
            }
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_tv = Rc::clone(&snapshot);
    let sess_tv = Rc::clone(&session);
    ui.on_transfer_verify_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_tv.borrow_mut();
            if let Some(sess) = sess_tv.borrow().as_ref() {
                match sess.verify_last_transfer_tampered() {
                    Ok(()) => {
                        snap.transfer_trust_label = "Invalid (tamper detected as expected)".into();
                        snap.transfer_status = "tamper check complete".into();
                        snap.push_log("transfer verify · tamper rejected (ok)");
                    }
                    Err(e) => {
                        snap.transfer_status = format!("verify failed: {e:?}");
                        snap.push_log(format!("transfer verify failed: {e:?}"));
                    }
                }
            } else {
                snap.transfer_status = "verify: open a case / export first".into();
            }
            snap.navigate_to(NavScreen::Transfer);
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_tab = Rc::clone(&snapshot);
    ui.on_inspector_tab_changed(move |tab| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_tab.borrow_mut();
            snap.inspector_tab = tab.to_string();
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_hp = Rc::clone(&snapshot);
    let sess_hp = Rc::clone(&session);
    ui.on_hex_prev_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_hp.borrow_mut();
            snap.step_hex_offset(-256);
            if let (Some(idx), Some(sess)) = (snap.selected_file_index, sess_hp.borrow().as_ref()) {
                if let Some(path) = snap.evidence_files.get(idx).map(|f| f.path.clone()) {
                    if !path.is_empty() {
                        match sess.read_hex(std::path::Path::new(&path), snap.hex_offset, 256) {
                            Ok((lines, status)) => {
                                snap.hex_lines = lines;
                                snap.hex_status = status;
                            }
                            Err(e) => {
                                snap.hex_lines =
                                    vec![format!("Error/Limited — hex read failed: {e:?}")];
                                snap.hex_status = "Error/Limited".into();
                            }
                        }
                    }
                }
            }
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_hn = Rc::clone(&snapshot);
    let sess_hn = Rc::clone(&session);
    ui.on_hex_next_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_hn.borrow_mut();
            snap.step_hex_offset(256);
            if let (Some(idx), Some(sess)) = (snap.selected_file_index, sess_hn.borrow().as_ref()) {
                if let Some(path) = snap.evidence_files.get(idx).map(|f| f.path.clone()) {
                    if !path.is_empty() {
                        match sess.read_hex(std::path::Path::new(&path), snap.hex_offset, 256) {
                            Ok((lines, status)) => {
                                snap.hex_lines = lines;
                                snap.hex_status = status;
                            }
                            Err(e) => {
                                snap.hex_lines =
                                    vec![format!("Error/Limited — hex read failed: {e:?}")];
                                snap.hex_status = "Error/Limited".into();
                            }
                        }
                    }
                }
            }
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_po = Rc::clone(&snapshot);
    ui.on_hex_popout_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_po.borrow_mut();
            snap.navigate_to(NavScreen::Hex);
            snap.inspector_tab = "hex".into();
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_it = Rc::clone(&snapshot);
    ui.on_import_timeline_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_it.borrow_mut();
            let path = std::env::var_os("TRAREON_TIMELINE_CSV")
                .map(PathBuf::from)
                .or_else(|| {
                    rfd::FileDialog::new()
                        .set_title("Import Plaso/Hayabusa CSV")
                        .add_filter("CSV", &["csv", "tsv", "txt"])
                        .pick_file()
                });
            let Some(path) = path else {
                snap.push_log("timeline import cancelled");
                apply(&ui, &snap);
                return;
            };
            match std::fs::read_to_string(&path) {
                Ok(raw) => {
                    let lines: Vec<String> = raw
                        .lines()
                        .filter(|l| !l.trim().is_empty())
                        .take(500)
                        .map(|l| l.to_string())
                        .collect();
                    snap.import_timeline_csv_lines(lines);
                }
                Err(e) => snap.push_log(format!("timeline import failed: {e}")),
            }
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_sk = Rc::clone(&snapshot);
    ui.on_shortcut_key(move |key| {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_sk.borrow_mut();
            snap.handle_shortcut(key.as_str());
            apply(&ui, &snap);
        }
    });

    // Prefs: remember last case dir when opened.
    let prefs_path = dirs_next_home()
        .map(|h| h.join(".trareon-lab").join("prefs.txt"))
        .unwrap_or_else(|| PathBuf::from("trareon-lab-prefs.txt"));
    if let Ok(prev) = std::fs::read_to_string(&prefs_path) {
        snapshot.borrow_mut().prefs_last_case = prev.trim().to_string();
    }
    apply(&ui, &snapshot.borrow());

    // Quick Verify: ephemeral case + hash + common-media carve.
    let ui_weak = ui.as_weak();
    let snap_qv = Rc::clone(&snapshot);
    let sess_qv = Rc::clone(&session);
    ui.on_quick_verify_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_qv.borrow_mut();
            let path = std::env::var_os("TRAREON_QUICK_VERIFY_PATH")
                .map(PathBuf::from)
                .or_else(|| {
                    rfd::FileDialog::new()
                        .set_title("Quick Verify — pick a file")
                        .pick_file()
                });
            let Some(path) = path else {
                snap.push_log("quick verify cancelled");
                apply(&ui, &snap);
                return;
            };
            if let Some(prev) = snap.ephemeral_case_dir.take() {
                let _ = std::fs::remove_dir_all(&prev);
            }
            let temp = std::env::temp_dir().join(format!(
                "trareon-quick-verify-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
            ));
            let _ = std::fs::create_dir_all(&temp);
            match LabSession::open_or_create(&temp) {
                Ok(mut sess) => {
                    if let Err(e) = sess.import_image(&path) {
                        snap.push_log(format!("quick verify import: {e:?}"));
                        apply(&ui, &snap);
                        return;
                    }
                    let meta = std::fs::metadata(&path).ok();
                    let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                    let mtime = meta
                        .and_then(|m| m.modified().ok())
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    let file_sha = {
                        use sha2::{Digest, Sha256};
                        match std::fs::read(&path) {
                            Ok(bytes) if bytes.len() as u64 <= 64 * 1024 * 1024 => {
                                let mut h = Sha256::new();
                                h.update(&bytes);
                                hex::encode(h.finalize())
                            }
                            Ok(_) => "sha256 deferred (file >64MiB)".into(),
                            Err(e) => format!("sha256 error: {e}"),
                        }
                    };
                    snap.quick_verify_meta_lines = vec![
                        format!("path · {}", path.display()),
                        format!("size · {size} bytes"),
                        format!("sha256 · {file_sha}"),
                        "banner · ephemeral · not a custody case".into(),
                    ];
                    snap.quick_verify_timeline_lines =
                        vec![format!("host mtime · {mtime}Z (no FS timeline fabricated)")];
                    match LabSession::carve_common_media(&path) {
                        Ok(hits) => {
                            let n = hits.len();
                            snap.carve_hit_lines = hits;
                            snap.push_log(format!("quick verify · {n} carve hit(s)"));
                        }
                        Err(e) => {
                            snap.carve_hit_lines = vec![format!("carve Limited: {e:?}")];
                            snap.push_log(format!("quick verify carve: {e:?}"));
                        }
                    }
                    let _ = sess.sync_snapshot(&mut snap);
                    snap.case_title = format!("QV · {}", path.display());
                    snap.ephemeral_case_dir = Some(temp.display().to_string());
                    snap.navigate_to(NavScreen::QuickVerify);
                    *sess_qv.borrow_mut() = Some(sess);
                }
                Err(e) => snap.push_log(format!("quick verify case: {e:?}")),
            }
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_demo = Rc::clone(&snapshot);
    let sess_demo = Rc::clone(&session);
    ui.on_load_demo_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_demo.borrow_mut();
            let demo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../testdata/demo/demo-disk.dd")
                .canonicalize()
                .unwrap_or_else(|_| {
                    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("../../testdata/demo/demo-disk.dd")
                });
            if !demo.is_file() {
                snap.push_log(format!("demo missing: {}", demo.display()));
                apply(&ui, &snap);
                return;
            }
            let temp = std::env::temp_dir().join(format!(
                "trareon-demo-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
            ));
            let _ = std::fs::create_dir_all(&temp);
            match LabSession::open_or_create(&temp) {
                Ok(mut sess) => match sess.import_image(&demo) {
                    Ok(()) => {
                        let _ = sess.sync_snapshot(&mut snap);
                        snap.case_title = "DEMO · demo-disk.dd".into();
                        if let Ok(hits) = LabSession::carve_common_media(&demo) {
                            snap.carve_hit_lines = hits;
                        }
                        snap.push_log("demo case loaded · UNSIGNED · lab use only");
                        snap.navigate_to(NavScreen::Evidence);
                        *sess_demo.borrow_mut() = Some(sess);
                    }
                    Err(e) => snap.push_log(format!("demo import: {e:?}")),
                },
                Err(e) => snap.push_log(format!("demo case: {e:?}")),
            }
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_carve = Rc::clone(&snapshot);
    let sess_carve = Rc::clone(&session);
    ui.on_run_carving_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_carve.borrow_mut();
            let path = snap
                .selected_file()
                .map(|f| PathBuf::from(&f.path))
                .or_else(|| snap.evidence_files.first().map(|f| PathBuf::from(&f.path)));
            let Some(path) = path else {
                snap.push_log("carve: no evidence path");
                apply(&ui, &snap);
                return;
            };
            if sess_carve.borrow().is_none() {
                snap.push_log("carve: open a case first");
                apply(&ui, &snap);
                return;
            }
            snap.set_progress("carve", 0, Some(1), "signature carve");
            match LabSession::carve_common_media(&path) {
                Ok(hits) => {
                    let n = hits.len();
                    snap.artifact_hits = hits
                        .iter()
                        .take(200)
                        .map(|line| lab_slint::ArtifactHitRow {
                            kind: "carve".into(),
                            summary: line.clone(),
                            provenance_ref: path.display().to_string(),
                            claim_type: lab_core::ClaimType::Indication,
                            uncertainty: "signature carve".into(),
                        })
                        .collect();
                    snap.carve_hit_lines = hits;
                    snap.set_progress("carve", 1, Some(1), format!("{n} hit(s)"));
                    snap.push_log(format!("carve · {n} hit(s) on {}", path.display()));
                }
                Err(e) => {
                    snap.carve_hit_lines = vec![format!("carve Limited: {e:?}")];
                    snap.clear_progress();
                    snap.push_log(format!("carve: {e:?}"));
                }
            }
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_disc = Rc::clone(&snapshot);
    let sess_disc = Rc::clone(&session);
    ui.on_discard_ephemeral_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_disc.borrow_mut();
            if let Some(dir) = snap.ephemeral_case_dir.take() {
                let _ = std::fs::remove_dir_all(&dir);
                snap.push_log(format!("discarded ephemeral · {dir}"));
            }
            *sess_disc.borrow_mut() = None;
            snap.carve_hit_lines.clear();
            snap.quick_verify_meta_lines.clear();
            snap.quick_verify_timeline_lines.clear();
            snap.open_case("(no case)", 0, 0);
            snap.navigate_to(NavScreen::CaseHome);
            apply(&ui, &snap);
        }
    });

    ui.invoke_focus_open_case();

    // Docs / screenshot automation (no picker when TRAREON_CASE_DIR is set).
    if std::env::var_os("TRAREON_AUTO_OPEN").is_some()
        && std::env::var_os("TRAREON_CASE_DIR").is_some()
    {
        ui.invoke_open_case_clicked();
    }
    if std::env::var_os("TRAREON_IMPORT_AUTO").is_some()
        && std::env::var_os("TRAREON_IMPORT_PATH").is_some()
    {
        ui.invoke_import_evidence_clicked();
    }
    if let Ok(screen) = std::env::var("TRAREON_START_SCREEN") {
        let mut snap = snapshot.borrow_mut();
        snap.navigate_to(NavScreen::from_label(screen.trim()));
        if std::env::var_os("TRAREON_PALETTE").is_some() {
            snap.palette_open = true;
            snap.filter_palette("");
        }
        apply(&ui, &snap);
    }

    ui.run()
}

#[cfg(feature = "gui")]
fn dirs_next_home() -> Option<std::path::PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(std::path::PathBuf::from)
}

#[cfg(not(feature = "gui"))]
fn main() {
    eprintln!("lab-slint built without `gui` feature");
    std::process::exit(1);
}
