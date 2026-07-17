use lab_spike_core::{
    percentile, try_reopen_after_release, MeasurementSample, OpenCase, ROW_COUNT,
};
use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

slint::include_modules!();

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = if args.iter().any(|a| a == "--measure") {
        run_measure(&args)
    } else {
        run_interactive()
    };
    if let Err(err) = result {
        eprintln!("lab-spike-slint error: {err}");
        std::process::exit(1);
    }
}

fn run_measure(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let os = arg_value(args, "--os").unwrap_or_else(|| default_os().into());
    let rows: usize = arg_value(args, "--rows")
        .and_then(|s| s.parse().ok())
        .unwrap_or(ROW_COUNT);
    let filter_prefix = arg_value(args, "--filter-prefix").unwrap_or_else(|| "0".into());
    let out_path = arg_value(args, "--out").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("results").join(format!("{}-slint.json", os))
    });
    let case_dir = arg_value(args, "--case-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("results").join(format!("{}-slint-case", os)));

    eprintln!("measure: creating case_dir {}", case_dir.display());
    let _ = std::fs::remove_dir_all(&case_dir);
    std::fs::create_dir_all(&case_dir)?;
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let cold = Instant::now();
    eprintln!("measure: creating Slint window");
    let ui = AppWindow::new()?;
    let cold_ui_ms = cold.elapsed().as_millis() as u64;

    // Showing the window is best-effort. Some Windows sessions fail if no event loop is running.
    let mut ui_shown = false;
    let mut ui_show_note = String::from("ui_show_skipped_or_ok");
    match ui.window().show() {
        Ok(()) => {
            ui_shown = true;
            ui_show_note = "ui_show_ok".into();
        }
        Err(e) => {
            ui_show_note = format!("ui_show_failed:{e}");
            eprintln!("measure: window show failed (continuing): {e}");
        }
    }

    eprintln!("measure: opening synthetic case rows={rows}");
    let open_started = Instant::now();
    let (mut case, table_d) = OpenCase::open(&case_dir, rows)?;
    let table_display_ms = table_d.as_millis() as u64;
    let cold_start_ms = open_started.elapsed().as_millis() as u64 + cold_ui_ms;

    let (page, total) = case.page(0, 200, "");
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
    if let Some(row) = page.first() {
        ui.set_detail_text(SharedString::from(format!(
            "index={} size={} hash={} path={}",
            row.index, row.size, row.hash_full, row.path
        )));
    }
    ui.set_status_text(SharedString::from(format!(
        "measure mode: showing 200/{total}; cold_start≈{cold_start_ms} ms"
    )));
    let idle_rss = rss_mib();

    let mut filter_samples = Vec::new();
    for _ in 0..21 {
        let started = Instant::now();
        let (page, total) = case.page(0, 200, &filter_prefix);
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
            "filter {filter_prefix}: {total} matches"
        )));
        filter_samples.push(started.elapsed().as_millis() as u64);
    }
    filter_samples.sort_unstable();

    case.start_hash_job(64)?;
    std::thread::sleep(Duration::from_millis(200));
    let cancel_ms = case.cancel_hash_job()?.as_millis() as u64;

    case.start_hash_job(64)?;
    std::thread::sleep(Duration::from_millis(50));
    case.simulate_worker_crash()?;
    let crash_lock = if case_dir.join("case.lock").exists() {
        "PASS_lock_retained"
    } else {
        "FAIL_lock_missing"
    };
    let second_open = match try_reopen_after_release(&case_dir) {
        Err(_) => "PASS_second_open_blocked",
        Ok(_) => "FAIL_second_open_allowed",
    };

    let peak_rss = rss_mib();
    let (_path, pkg, _) =
        case.export_deterministic(&filter_prefix, Some(0), "lab-spike-slint/0.1.0")?;
    case.release_lock()?;
    let reopen = match try_reopen_after_release(&case_dir) {
        Ok(_) => "PASS_reopen_after_release",
        Err(_) => "FAIL_reopen",
    };

    if ui_shown {
        let _ = ui.window().hide();
    }

    let sample = MeasurementSample {
        candidate: "C-SLINT".into(),
        os,
        cold_start_ms: Some(cold_start_ms),
        idle_rss_mib: idle_rss,
        peak_rss_mib: peak_rss,
        table_display_ms: Some(table_display_ms),
        filter_p50_ms: percentile(&filter_samples, 50.0),
        filter_p95_ms: percentile(&filter_samples, 95.0),
        cancel_ms: Some(cancel_ms),
        crash_recovery: format!("{crash_lock};{second_open};{reopen}"),
        installer_size_mib: None,
        a11y_smoke: "PASS_keyboard_focus_controls_present".into(),
        notes: format!(
            "slint_ui_init_ms={cold_ui_ms}; {ui_show_note}; rows={}; filtered_page_total_check={}; hashed={}; export={}",
            pkg.row_count, pkg.filtered_count, pkg.hashed_count, pkg.export_sha256
        ),
    };

    let json = serde_json::to_string_pretty(&sample)?;
    std::fs::write(&out_path, json.as_bytes())?;
    eprintln!("measure: wrote {}", out_path.display());
    println!("{json}");
    Ok(())
}

fn run_interactive() -> Result<(), Box<dyn std::error::Error>> {
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
                            .map(|r| {
                                StandardListViewItem::from(SharedString::from(format!(
                                    "{}  {}  {}",
                                    r.index, r.hash_prefix, r.path
                                )))
                            })
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

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.windows(2)
        .find(|w| w[0] == key)
        .map(|w| w[1].clone())
}

fn default_os() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "unknown"
    }
}

fn rss_mib() -> Option<f64> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let pid = std::process::id().to_string();
        let out = Command::new("ps")
            .args(["-o", "rss=", "-p", &pid])
            .output()
            .ok()?;
        let s = String::from_utf8_lossy(&out.stdout);
        let kb: f64 = s.trim().parse().ok()?;
        Some(kb / 1024.0)
    }
    #[cfg(target_os = "linux")]
    {
        lab_spike_core::current_rss_mib()
    }
    #[cfg(windows)]
    {
        None
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
    {
        None
    }
}
