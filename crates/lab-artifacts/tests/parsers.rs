use lab_artifacts::{
    artifacts_to_timeline, parse_jumplist_synth, parse_linux_auth_syslog_synth, parse_lnk_synth,
    parse_prefetch_synth, parse_unified_log_synth,
};

#[test]
fn prefetch_lnk_jumplist_parsers() {
    let pf = r#"{"schema":"prefetch-synth-1","hits":[{"exe_name":"NOTEPAD.EXE","run_count":3,"last_run_utc":"2026-01-02T10:00:00Z","path":"C:\\Windows\\prefetch\\NOTEPAD.EXE-123.pf"}]}"#;
    let hits = parse_prefetch_synth(pf).unwrap();
    assert_eq!(hits[0].kind, "windows.prefetch");

    let lnk = r#"{"schema":"lnk-synth-1","links":[{"target_path":"C:\\Tools\\x.exe","created_utc":"2026-01-01T09:00:00Z","link_path":"Desktop\\x.lnk"}]}"#;
    assert_eq!(parse_lnk_synth(lnk).unwrap().len(), 1);

    let jl = r#"{"schema":"jumplist-synth-1","entries":[{"app_id":"chrome","target":"C:\\a.pdf","accessed_utc":"2026-01-03T11:00:00Z"}]}"#;
    let tl = artifacts_to_timeline(&parse_jumplist_synth(jl).unwrap());
    assert_eq!(tl[0].kind, "windows.jumplist");
}

#[test]
fn macos_linux_synth_parsers() {
    let ul = r#"{"schema":"ulog-synth-1","events":[{"subsystem":"com.apple.login","message":"user login","timestamp_utc":"2026-02-01T08:00:00Z"}]}"#;
    assert_eq!(
        parse_unified_log_synth(ul).unwrap()[0].kind,
        "macos.unified_log"
    );
    let sy = r#"{"schema":"linux-auth-synth-1","lines":[{"facility":"sshd","message":"Accepted publickey","timestamp_utc":"2026-02-02T08:00:00Z"}]}"#;
    assert_eq!(
        parse_linux_auth_syslog_synth(sy).unwrap()[0].kind,
        "linux.auth_syslog"
    );
}
