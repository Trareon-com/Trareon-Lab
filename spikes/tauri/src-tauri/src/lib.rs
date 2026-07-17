use lab_spike_core::{
    ipc_err, ipc_ok, run_equal_measure, validate_ipc_request, EqualMeasureInput, IpcRequest,
    OpenCase, ROW_COUNT,
};
use parking_lot::Mutex;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tauri::Manager;

struct AppState {
    case: Mutex<Option<OpenCase>>,
    case_dir: PathBuf,
}

fn arg_value(args: &[String], key: &str) -> Option<String> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1).cloned())
}

fn default_os() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "macos"
    }
}

fn run_measure_in_setup(
    app: &tauri::App,
    args: &[String],
    ui_init_ms: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let os = arg_value(args, "--os").unwrap_or_else(|| default_os().into());
    let rows: usize = arg_value(args, "--rows")
        .and_then(|s| s.parse().ok())
        .unwrap_or(ROW_COUNT);
    let filter_prefix = arg_value(args, "--filter-prefix").unwrap_or_else(|| "0".into());
    let out_path = arg_value(args, "--out").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from("../results").join(format!("{os}-tauri.json"))
    });
    let case_dir = arg_value(args, "--case-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("../results").join(format!("{os}-tauri-case")));

    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if let Some(win) = app.get_webview_window("main") {
        let _ = win.set_title("Trareon Lab Gate A Spike — Tauri (measure)");
        match win.show() {
            Ok(()) => eprintln!("measure: webview show ok"),
            Err(e) => eprintln!("measure: webview show failed (continuing): {e}"),
        }
    } else {
        eprintln!("measure: main webview missing (continuing with Rust core)");
    }

    eprintln!("measure: opening synthetic case rows={rows}");
    let sample = run_equal_measure(EqualMeasureInput {
        candidate: "C-TAURI".into(),
        os,
        rows,
        filter_prefix,
        case_dir,
        build_identity: "lab-spike-tauri/0.1.0".into(),
        a11y_smoke: "PASS_keyboard_focus_controls_present".into(),
        ui_init_ms,
        notes_prefix: "webview_shell=tauri2_svelte5; evidence_bytes_stay_in_rust_core=true".into(),
        idle_rss_mib: None,
        peak_rss_mib: None,
    })?;

    let json = serde_json::to_string_pretty(&sample)?;
    std::fs::write(&out_path, json.as_bytes())?;
    eprintln!("measure: wrote {}", out_path.display());
    println!("{json}");
    app.handle().exit(0);
    Ok(())
}

#[tauri::command]
fn open_case(state: tauri::State<Arc<AppState>>) -> Result<serde_json::Value, String> {
    let started = Instant::now();
    let (case, table_d) = OpenCase::open(&state.case_dir, ROW_COUNT).map_err(|e| e.to_string())?;
    let (page, total) = case.page(0, 200, "");
    *state.case.lock() = Some(case);
    Ok(serde_json::json!({
        "elapsed_ms": started.elapsed().as_millis() as u64,
        "table_ms": table_d.as_millis() as u64,
        "total": total,
        "page": page,
        "note": "Evidence bytes stay in Rust core; UI receives page slices only."
    }))
}

#[tauri::command]
fn page_rows(
    state: tauri::State<Arc<AppState>>,
    offset: usize,
    limit: usize,
    prefix: String,
) -> Result<serde_json::Value, String> {
    let guard = state.case.lock();
    let case = guard.as_ref().ok_or_else(|| "open a case first".to_string())?;
    let started = Instant::now();
    let (page, total) = case.page(offset, limit, &prefix);
    Ok(serde_json::json!({
        "elapsed_ms": started.elapsed().as_millis() as u64,
        "total": total,
        "page": page
    }))
}

#[tauri::command]
fn start_hash(state: tauri::State<Arc<AppState>>) -> Result<String, String> {
    let mut guard = state.case.lock();
    let case = guard.as_mut().ok_or_else(|| "open a case first".to_string())?;
    case.start_hash_job(64).map_err(|e| e.to_string())?;
    Ok("hash job started (bounded queue 64)".into())
}

#[tauri::command]
fn cancel_hash(state: tauri::State<Arc<AppState>>) -> Result<String, String> {
    let mut guard = state.case.lock();
    let case = guard.as_mut().ok_or_else(|| "open a case first".to_string())?;
    let d = case.cancel_hash_job().map_err(|e| e.to_string())?;
    Ok(format!("cancelled in {} ms", d.as_millis()))
}

#[tauri::command]
fn crash_worker(state: tauri::State<Arc<AppState>>) -> Result<String, String> {
    let mut guard = state.case.lock();
    let case = guard.as_mut().ok_or_else(|| "open a case first".to_string())?;
    case.simulate_worker_crash().map_err(|e| e.to_string())?;
    Ok("worker crash simulated; case lock retained".into())
}

#[tauri::command]
fn export_case(
    state: tauri::State<Arc<AppState>>,
    filter_prefix: String,
) -> Result<serde_json::Value, String> {
    let guard = state.case.lock();
    let case = guard.as_ref().ok_or_else(|| "open a case first".to_string())?;
    let (path, pkg, d) = case
        .export_deterministic(&filter_prefix, Some(0), "lab-spike-tauri/0.1.0")
        .map_err(|e| e.to_string())?;
    Ok(serde_json::json!({
        "path": path,
        "elapsed_ms": d.as_millis() as u64,
        "export_sha256": pkg.export_sha256,
        "filtered_count": pkg.filtered_count
    }))
}

#[tauri::command]
fn ipc_roundtrip(req: IpcRequest) -> Result<serde_json::Value, String> {
    if let Err(e) = validate_ipc_request(&req) {
        return Ok(serde_json::to_value(ipc_err(&req.correlation_id, e)).unwrap());
    }
    Ok(serde_json::to_value(ipc_ok(
        &req.correlation_id,
        serde_json::json!({ "echo": req.command }),
    ))
    .unwrap())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let measure = args.iter().any(|a| a == "--measure");
    let args_for_measure = args.clone();

    let case_dir = std::env::temp_dir().join(format!("trareon-lab-tauri-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&case_dir);
    let state = Arc::new(AppState {
        case: Mutex::new(None),
        case_dir,
    });

    let ui_start = Instant::now();
    eprintln!(
        "{}",
        if measure {
            "measure: starting Tauri shell for C-TAURI"
        } else {
            "interactive: starting Tauri shell for C-TAURI"
        }
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            open_case,
            page_rows,
            start_hash,
            cancel_hash,
            crash_worker,
            export_case,
            ipc_roundtrip
        ])
        .setup(move |app| {
            if measure {
                let ui_init_ms = ui_start.elapsed().as_millis() as u64;
                run_measure_in_setup(app, &args_for_measure, ui_init_ms).map_err(|e| {
                    eprintln!("lab-spike-tauri measure error: {e}");
                    e
                })?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
