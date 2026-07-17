use lab_spike_core::{OpenCase, ROW_COUNT};
use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    let case: Rc<RefCell<Option<OpenCase>>> = Rc::new(RefCell::new(None));
    let case_dir = std::env::temp_dir().join(format!("trareon-lab-slint-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&case_dir);

    {
        let ui_weak = ui.as_weak();
        let case = Rc::clone(&case);
        let case_dir = case_dir.clone();
        ui.on_open_case(move || {
            let started = Instant::now();
            match OpenCase::open(&case_dir, ROW_COUNT) {
                Ok((opened, table_d)) => {
                    let (page, total) = opened.page(0, 200, "");
                    *case.borrow_mut() = Some(opened);
                    if let Some(ui) = ui_weak.upgrade() {
                        let model: Vec<StandardListViewItem> = page
                            .iter()
                            .map(|r| StandardListViewItem::from(SharedString::from(format!(
                                "{}  {}  {}",
                                r.index, r.hash_prefix, r.path
                            ))))
                            .collect();
                        ui.set_rows(ModelRc::new(VecModel::from(model)));
                        ui.set_status_text(SharedString::from(format!(
                            "Opened case in {} ms; table materialize {} ms; showing 200/{} rows (virtualized page). Evidence bytes stay in Rust core.",
                            started.elapsed().as_millis(),
                            table_d.as_millis(),
                            total
                        )));
                    }
                }
                Err(e) => {
                    if let Some(ui) = ui_weak.upgrade() {
                        ui.set_status_text(SharedString::from(format!("open failed: {e}")));
                    }
                }
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let case = Rc::clone(&case);
        ui.on_apply_filter(move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            let prefix = ui.get_filter_text().to_string();
            let mut borrow = case.borrow_mut();
            let Some(c) = borrow.as_mut() else {
                ui.set_status_text(SharedString::from("open a case first"));
                return;
            };
            let started = Instant::now();
            let (page, total) = c.page(0, 200, &prefix);
            let model: Vec<StandardListViewItem> = page
                .iter()
                .map(|r| {
                    StandardListViewItem::from(SharedString::from(format!(
                        "{}  {}  {}",
                        r.index, r.hash_prefix, r.path
                    )))
                })
                .collect();
            ui.set_rows(ModelRc::new(VecModel::from(model)));
            ui.set_status_text(SharedString::from(format!(
                "Filter '{prefix}' => {total} matches; page 200 in {} ms",
                started.elapsed().as_millis()
            )));
        });
    }

    {
        let ui_weak = ui.as_weak();
        let case = Rc::clone(&case);
        ui.on_start_hash(move || {
            let mut borrow = case.borrow_mut();
            if let Some(c) = borrow.as_mut() {
                let msg = match c.start_hash_job(64) {
                    Ok(()) => "hash job started (bounded queue 64)".into(),
                    Err(e) => format!("start failed: {e}"),
                };
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_status_text(SharedString::from(msg));
                }
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let case = Rc::clone(&case);
        ui.on_cancel_hash(move || {
            let mut borrow = case.borrow_mut();
            if let Some(c) = borrow.as_mut() {
                let msg = match c.cancel_hash_job() {
                    Ok(d) => format!("cancelled in {} ms", d.as_millis()),
                    Err(e) => format!("cancel failed: {e}"),
                };
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_status_text(SharedString::from(msg));
                }
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let case = Rc::clone(&case);
        ui.on_crash_worker(move || {
            let mut borrow = case.borrow_mut();
            if let Some(c) = borrow.as_mut() {
                let msg = match c.simulate_worker_crash() {
                    Ok(()) => "worker crash simulated; case lock retained".into(),
                    Err(e) => format!("crash sim failed: {e}"),
                };
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_status_text(SharedString::from(msg));
                }
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let case = Rc::clone(&case);
        ui.on_export_json(move || {
            let borrow = case.borrow();
            if let Some(c) = borrow.as_ref() {
                let prefix = ui_weak
                    .upgrade()
                    .map(|u| u.get_filter_text().to_string())
                    .unwrap_or_default();
                let msg = match c.export_deterministic(&prefix, Some(0), "lab-spike-slint/0.1.0") {
                    Ok((path, pkg, d)) => format!(
                        "exported {} in {} ms sha256={}",
                        path.display(),
                        d.as_millis(),
                        pkg.export_sha256
                    ),
                    Err(e) => format!("export failed: {e}"),
                };
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_status_text(SharedString::from(msg));
                }
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let case = Rc::clone(&case);
        ui.on_select_row(move |idx| {
            let borrow = case.borrow();
            let Some(c) = borrow.as_ref() else { return };
            let Some(ui) = ui_weak.upgrade() else { return };
            let prefix = ui.get_filter_text().to_string();
            let (page, _) = c.page(0, 200, &prefix);
            if let Some(row) = page.get(idx as usize) {
                ui.set_detail_text(SharedString::from(format!(
                    "index={} size={} hash={} path={}",
                    row.index, row.size, row.hash_full, row.path
                )));
            }
        });
    }

    ui.run()?;
    Ok(())
}
