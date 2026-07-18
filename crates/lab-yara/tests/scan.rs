use lab_yara::YaraEngine;
use std::io::Write;

#[test]
fn builtin_eicar_and_custom_rule() {
    let eng = YaraEngine::load_dir(None).unwrap();
    let hits = eng
        .scan_bytes(b"X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*")
        .unwrap();
    assert!(hits.iter().any(|h| h.rule_name.contains("Eicar")));

    let eng2 = YaraEngine::from_source(
        r#"
rule Hello {
  strings:
    $a = "HELLO_YARA"
  condition:
    $a
}
"#,
    )
    .unwrap();
    assert!(!eng2.scan_bytes(b"xxHELLO_YARAyy").unwrap().is_empty());
    assert!(eng2.scan_bytes(b"nothing").unwrap().is_empty());

    let dir = tempfile::tempdir().unwrap();
    let f = dir.path().join("sample.bin");
    std::fs::File::create(&f)
        .unwrap()
        .write_all(b"HELLO_YARA")
        .unwrap();
    assert!(!eng2.scan_file(&f).unwrap().is_empty());
}
