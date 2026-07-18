use lab_yara::{YaraEngine, YARA_ENGINE_ID};

#[test]
fn yara_x_golden_rules_preserve_conditions_matches_and_metadata() {
    assert_eq!(YARA_ENGINE_ID, "yara-x/1.19.0");
    assert_eq!(YARA_ENGINE_ID, format!("yara-x/{}", yara_x::VERSION));

    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("production.yar"),
        include_str!("fixtures/production.yar"),
    )
    .unwrap();
    let engine = YaraEngine::load_dir(Some(dir.path())).unwrap();

    let only_text = engine.scan_bytes(b"prefix ALPHA MARKER suffix").unwrap();
    assert!(!only_text.iter().any(|hit| hit.rule_name == "Text_And_Hex"));

    let hits = engine
        .scan_bytes(b"prefix ALPHA MARKER \xDE\xAD\xBE\xEF and second choice")
        .unwrap();
    let golden: Vec<_> = hits
        .iter()
        .filter(|hit| hit.rule_name != "Trareon_Eicar_Substring")
        .map(|hit| {
            (
                hit.rule_name.as_str(),
                hit.meta.get("severity").map(String::as_str),
                hit.offset,
                hit.matched_data.as_slice(),
            )
        })
        .collect();
    assert_eq!(
        golden,
        vec![
            ("Text_And_Hex", Some("HIGH"), 7, b"ALPHA MARKER".as_slice()),
            ("Text_Or_Text", Some("LOW"), 29, b"second choice".as_slice()),
        ]
    );
}
