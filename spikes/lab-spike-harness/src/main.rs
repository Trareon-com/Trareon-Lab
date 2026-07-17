use clap::{Parser, Subcommand};
use lab_spike_core::{
    probe_rss_mib, run_equal_measure, try_reopen_after_release, EqualMeasureInput, ROW_COUNT,
};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lab-spike-harness")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run the equal workflow and print a MeasurementSample JSON.
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
        #[arg(long)]
        out: Option<PathBuf>,
        #[arg(long, default_value_t = 0)]
        ui_init_ms: u64,
        #[arg(long, default_value = "N/A_headless")]
        a11y_smoke: String,
        #[arg(long, default_value = "headless_harness")]
        notes_prefix: String,
        #[arg(long, default_value = "lab-spike-harness/0.1.0")]
        build_identity: String,
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
            out,
            ui_init_ms,
            a11y_smoke,
            notes_prefix,
            build_identity,
        } => {
            let case_dir = case_dir.unwrap_or_else(|| {
                std::env::temp_dir().join(format!("trareon-lab-spike-{}", std::process::id()))
            });
            let out_path = out.unwrap_or_else(|| case_dir.join("measurement.json"));
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).expect("out parent");
            }

            let idle = probe_rss_mib();
            let sample = run_equal_measure(EqualMeasureInput {
                candidate,
                os,
                rows,
                filter_prefix,
                case_dir: case_dir.clone(),
                build_identity,
                a11y_smoke,
                ui_init_ms,
                notes_prefix,
                idle_rss_mib: idle,
                peak_rss_mib: None,
            })
            .expect("equal measure");

            // Refresh peak after workflow.
            let mut sample = sample;
            sample.peak_rss_mib = probe_rss_mib().or(sample.peak_rss_mib);

            let json = serde_json::to_string_pretty(&sample).unwrap();
            println!("{json}");
            fs::write(&out_path, json.as_bytes()).unwrap();
            eprintln!("measure: wrote {}", out_path.display());
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
