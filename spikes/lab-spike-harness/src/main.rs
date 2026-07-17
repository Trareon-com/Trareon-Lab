use clap::{Parser, Subcommand};
use lab_spike_core::{
    percentile, try_reopen_after_release, MeasurementSample, OpenCase, ROW_COUNT,
};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "lab-spike-harness")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run the equal headless workflow and print a MeasurementSample JSON.
    Measure {
        #[arg(long, default_value = "harness-core")]
        candidate: String,
        #[arg(long, default_value = "macos")]
        os: String,
        #[arg(long, default_value_t = ROW_COUNT)]
        rows: usize,
        #[arg(long)]
        case_dir: Option<PathBuf>,
        #[arg(long, default_value = "0")]
        filter_prefix: String,
    },
    /// Prove second process cannot open while lock is held.
    LockProbe {
        #[arg(long)]
        case_dir: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Measure {
            candidate,
            os,
            rows,
            case_dir,
            filter_prefix,
        } => {
            let case_dir = case_dir.unwrap_or_else(|| {
                std::env::temp_dir().join(format!("trareon-lab-spike-{}", std::process::id()))
            });
            let _ = fs::remove_dir_all(&case_dir);
            fs::create_dir_all(&case_dir).expect("case dir");

            let cold = Instant::now();
            let (mut case, table_display) =
                OpenCase::open(&case_dir, rows).expect("open case");
            let cold_start_ms = cold.elapsed().as_millis() as u64;
            let table_display_ms = table_display.as_millis() as u64;
            let idle_rss = rss_mib();

            let mut filter_samples = Vec::new();
            for _ in 0..21 {
                let (_idxs, d) = case.filter_by_hash_prefix(&filter_prefix);
                filter_samples.push(d.as_millis() as u64);
            }
            filter_samples.sort_unstable();
            let filter_p50 = percentile(&filter_samples, 50.0);
            let filter_p95 = percentile(&filter_samples, 95.0);

            let _ = case.detail(0);
            case.start_hash_job(64).expect("start hash");
            std::thread::sleep(std::time::Duration::from_millis(200));
            let cancel_ms = case.cancel_hash_job().expect("cancel").as_millis() as u64;

            case.start_hash_job(64).expect("restart hash");
            std::thread::sleep(std::time::Duration::from_millis(50));
            case.simulate_worker_crash().expect("crash");
            let crash_recovery = match ensure_lock_still_held(&case_dir) {
                Ok(()) => "PASS_lock_retained".to_string(),
                Err(e) => format!("FAIL_{e}"),
            };

            let peak_rss = rss_mib();
            let (_path, pkg, _) = case
                .export_deterministic(&filter_prefix, Some(0), "lab-spike-harness/0.1.0")
                .expect("export");

            // Second process must fail while we still hold the lock.
            let lock_blocked = match try_reopen_after_release(&case_dir) {
                Err(_) => "PASS_second_open_blocked".to_string(),
                Ok(_) => "FAIL_second_open_allowed".to_string(),
            };

            case.release_lock().expect("release");
            let reopen_ok = match try_reopen_after_release(&case_dir) {
                Ok(_) => "PASS_reopen_after_release".to_string(),
                Err(e) => format!("FAIL_reopen_{e}"),
            };

            let sample = MeasurementSample {
                candidate,
                os,
                cold_start_ms: Some(cold_start_ms),
                idle_rss_mib: idle_rss,
                peak_rss_mib: peak_rss,
                table_display_ms: Some(table_display_ms),
                filter_p50_ms: filter_p50,
                filter_p95_ms: filter_p95,
                cancel_ms: Some(cancel_ms),
                crash_recovery: format!("{crash_recovery};{lock_blocked};{reopen_ok}"),
                installer_size_mib: None,
                a11y_smoke: "N/A_headless".into(),
                notes: format!(
                    "rows={}; filtered={}; hashed={}; export={}",
                    pkg.row_count, pkg.filtered_count, pkg.hashed_count, pkg.export_sha256
                ),
            };
            println!("{}", serde_json::to_string_pretty(&sample).unwrap());
            let out = case_dir.join("measurement.json");
            fs::write(&out, serde_json::to_vec_pretty(&sample).unwrap()).unwrap();
        }
        Cmd::LockProbe { case_dir } => match try_reopen_after_release(case_dir) {
            Ok(id) => {
                println!("unexpected open ok case_id={id}");
                std::process::exit(2);
            }
            Err(e) => {
                println!("blocked_as_expected: {e}");
            }
        },
    }
}

fn ensure_lock_still_held(case_dir: &std::path::Path) -> Result<(), String> {
    let lock = case_dir.join("case.lock");
    if !lock.exists() {
        return Err("lock missing after crash simulation".into());
    }
    Ok(())
}

fn rss_mib() -> Option<f64> {
    #[cfg(target_os = "macos")]
    {
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
