use lab_core::{
    export_case_pdfa, export_case_uco, ExportMode, CASE_UCO_PROFILE_VERSION,
};

const CASE_ID: &str = "11111111-1111-4111-8111-111111111111";

#[test]
fn pdfa_export_has_metadata_content_and_consistent_xref() {
    let artifact = export_case_pdfa(
        CASE_ID,
        "Case (A)",
        "First line\nSecond \\ line",
        ExportMode::Sealed,
    )
    .unwrap();

    assert_eq!(artifact.validation_level, "Validated");
    assert!(artifact.body.starts_with("%PDF-1.4\n"));
    assert!(artifact.body.contains("pdfaid:part=\"1\""));
    assert!(artifact.body.contains("pdfaid:conformance=\"B\""));
    assert!(artifact
        .body
        .contains("trareon:validationLevel=\"Validated\""));
    assert!(artifact.body.contains("/Type /Metadata /Subtype /XML"));
    assert!(artifact.body.contains("First line"));
    assert!(artifact.body.contains("Second \\\\ line"));
    assert!(artifact.body.ends_with("%%EOF\n"));

    let startxref = artifact
        .body
        .rsplit_once("startxref\n")
        .unwrap()
        .1
        .lines()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert_eq!(&artifact.body.as_bytes()[startxref..startxref + 4], b"xref");
}

#[test]
fn case_uco_export_is_versioned_json_ld_subset() {
    let artifact = export_case_uco(
        CASE_ID,
        "Case A",
        "Evidence-backed report",
        ExportMode::Draft,
    )
    .unwrap();
    let value: serde_json::Value = serde_json::from_str(&artifact.body).unwrap();

    assert_eq!(artifact.validation_level, "Validated");
    assert!(artifact.labeled_non_final);
    assert_eq!(value["@id"], format!("urn:trareon:lab:{CASE_ID}"));
    assert_eq!(value["@type"], "case:Case");
    assert_eq!(value["trareon:profileVersion"], CASE_UCO_PROFILE_VERSION);
    assert_eq!(value["trareon:reportText"], "Evidence-backed report");
    assert_eq!(value["trareon:labeledNonFinal"], true);
}
