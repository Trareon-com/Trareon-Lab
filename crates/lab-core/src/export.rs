//! Deterministic export / report skeleton.

use crate::error::LabError;
use crate::result::LabResult;
use crate::run_manifest::CASE_UCO_PROFILE_VERSION;
use serde_json::json;
use sha2::{Digest, Sha256};

/// Export mode — drafts must be labeled non-final.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportMode {
    Draft,
    Sealed,
}

/// Skeleton export artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportArtifact {
    pub mode: ExportMode,
    pub body: String,
    pub digest_sha256: String,
    pub labeled_non_final: bool,
    /// Honest capability status for the emitted format.
    pub validation_level: String,
}

/// Build a deterministic JSON/HTML skeleton export for a sealed or draft case.
pub fn export_case_skeleton(
    case_uuid: &str,
    title: &str,
    evidence_count: u32,
    coverage_count: u32,
    mode: ExportMode,
) -> LabResult<ExportArtifact> {
    if case_uuid.is_empty() || title.is_empty() {
        return Err(LabError::Internal {
            detail: "export requires case_uuid and title".into(),
        });
    }
    let labeled_non_final = matches!(mode, ExportMode::Draft);
    let status = match mode {
        ExportMode::Draft => "draft_non_final",
        ExportMode::Sealed => "sealed",
    };
    // Canonical key order for determinism.
    let body = format!(
        "{{\n  \"schema_version\": \"export-skeleton-1\",\n  \"case_uuid\": \"{case_uuid}\",\n  \"title\": \"{title}\",\n  \"evidence_count\": {evidence_count},\n  \"coverage_count\": {coverage_count},\n  \"status\": \"{status}\",\n  \"labeled_non_final\": {labeled_non_final}\n}}\n"
    );
    let digest_sha256 = hex::encode(Sha256::digest(body.as_bytes()));
    Ok(ExportArtifact {
        mode,
        body,
        digest_sha256,
        labeled_non_final,
        validation_level: "Limited".into(),
    })
}

/// Deterministic HTML report skeleton (Day 38) — draft must stay labeled non-final.
pub fn export_case_html(
    case_uuid: &str,
    title: &str,
    findings: &[(String, String)],
    mode: ExportMode,
) -> LabResult<ExportArtifact> {
    if case_uuid.is_empty() || title.is_empty() {
        return Err(LabError::Internal {
            detail: "html export requires case_uuid and title".into(),
        });
    }
    let labeled_non_final = matches!(mode, ExportMode::Draft);
    let status = match mode {
        ExportMode::Draft => "draft_non_final",
        ExportMode::Sealed => "sealed",
    };
    let mut items = String::new();
    for (claim, bookmark) in findings {
        items.push_str(&format!("<li data-bookmark=\"{bookmark}\">{claim}</li>\n"));
    }
    let body = format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>{title}</title></head><body>\n\
         <h1>{title}</h1>\n\
         <p>case={case_uuid} status={status} labeled_non_final={labeled_non_final}</p>\n\
         <ul>\n{items}</ul>\n</body></html>\n"
    );
    let digest_sha256 = hex::encode(Sha256::digest(body.as_bytes()));
    Ok(ExportArtifact {
        mode,
        body,
        digest_sha256,
        labeled_non_final,
        validation_level: "Limited".into(),
    })
}

/// Emit a deterministic textual PDF/A-1b-oriented report.
///
/// The deterministic textual subset is corpus-validated; this is not a claim
/// that the output passes a full external PDF/A conformance validator.
pub fn export_case_pdfa(
    case_uuid: &str,
    title: &str,
    report_text: &str,
    mode: ExportMode,
) -> LabResult<ExportArtifact> {
    validate_export_input(case_uuid, title, "PDF/A")?;
    let labeled_non_final = matches!(mode, ExportMode::Draft);
    let status = export_status(mode);
    let mut content = String::from("BT\n/F1 10 Tf\n50 760 Td\n");
    for line in std::iter::once(title).chain(report_text.lines()) {
        content.push_str(&format!("({}) Tj\n0 -14 Td\n", pdf_literal(line)));
    }
    content.push_str(&format!(
        "(case={} status={} labeled_non_final={}) Tj\nET\n",
        pdf_literal(case_uuid),
        status,
        labeled_non_final
    ));

    let metadata = format!(
        "<?xpacket begin=\"\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?>\n\
         <x:xmpmeta xmlns:x=\"adobe:ns:meta/\"><rdf:RDF \
         xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"><rdf:Description \
         rdf:about=\"\" xmlns:pdfaid=\"http://www.aiim.org/pdfa/ns/id/\" \
         xmlns:trareon=\"https://trareon.example/ns#\" pdfaid:part=\"1\" \
         pdfaid:conformance=\"B\" trareon:validationLevel=\"Validated\" \
         trareon:caseUuid=\"{}\"/></rdf:RDF></x:xmpmeta>\n\
         <?xpacket end=\"w\"?>",
        xml_attribute(case_uuid)
    );
    let objects = vec![
        "<< /Type /Catalog /Pages 2 0 R /Metadata 6 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] \
         /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>"
            .to_string(),
        format!(
            "<< /Length {} >>\nstream\n{}endstream",
            content.len(),
            content
        ),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
        format!(
            "<< /Type /Metadata /Subtype /XML /Length {} >>\nstream\n{}\nendstream",
            metadata.len(),
            metadata
        ),
        format!(
            "<< /Title ({}) /Producer (Trareon Lab) /Trapped /False >>",
            pdf_literal(title)
        ),
    ];
    let body = build_pdf(&objects);
    Ok(artifact(mode, body, labeled_non_final, "Validated"))
}

/// Emit the pinned Trareon CASE/UCO JSON-LD subset.
pub fn export_case_uco(
    case_uuid: &str,
    title: &str,
    report_text: &str,
    mode: ExportMode,
) -> LabResult<ExportArtifact> {
    validate_export_input(case_uuid, title, "CASE/UCO")?;
    let labeled_non_final = matches!(mode, ExportMode::Draft);
    let document = json!({
        "@context": {
            "case": "https://ontology.caseontology.org/case/",
            "core": "https://ontology.unifiedcyberontology.org/uco/core/",
            "trareon": "https://trareon.example/ns#"
        },
        "@id": format!("urn:trareon:lab:{case_uuid}"),
        "@type": "case:Case",
        "core:name": title,
        "trareon:labeledNonFinal": labeled_non_final,
        "trareon:profileVersion": CASE_UCO_PROFILE_VERSION,
        "trareon:reportText": report_text,
        "trareon:status": export_status(mode)
    });
    let mut body = serde_json::to_string_pretty(&document).map_err(|e| LabError::Internal {
        detail: format!("CASE/UCO serialization: {e}"),
    })?;
    body.push('\n');
    Ok(artifact(mode, body, labeled_non_final, "Validated"))
}

fn artifact(
    mode: ExportMode,
    body: String,
    labeled_non_final: bool,
    validation_level: &str,
) -> ExportArtifact {
    let digest_sha256 = hex::encode(Sha256::digest(body.as_bytes()));
    ExportArtifact {
        mode,
        body,
        digest_sha256,
        labeled_non_final,
        validation_level: validation_level.into(),
    }
}

fn validate_export_input(case_uuid: &str, title: &str, format: &str) -> LabResult<()> {
    if case_uuid.is_empty() || title.is_empty() {
        return Err(LabError::Internal {
            detail: format!("{format} export requires case_uuid and title"),
        });
    }
    Ok(())
}

fn export_status(mode: ExportMode) -> &'static str {
    match mode {
        ExportMode::Draft => "draft_non_final",
        ExportMode::Sealed => "sealed",
    }
}

fn pdf_literal(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace(['\r', '\n'], " ")
}

fn xml_attribute(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn build_pdf(objects: &[String]) -> String {
    let mut body = String::from("%PDF-1.4\n%Trareon-PDFA\n");
    let mut offsets = Vec::with_capacity(objects.len());
    for (index, object) in objects.iter().enumerate() {
        offsets.push(body.len());
        body.push_str(&format!("{} 0 obj\n{}\nendobj\n", index + 1, object));
    }
    let xref_offset = body.len();
    body.push_str(&format!("xref\n0 {}\n", objects.len() + 1));
    body.push_str("0000000000 65535 f \n");
    for offset in offsets {
        body.push_str(&format!("{offset:010} 00000 n \n"));
    }
    body.push_str(&format!(
        "trailer\n<< /Size {} /Root 1 0 R /Info 7 0 R >>\n\
         startxref\n{xref_offset}\n%%EOF\n",
        objects.len() + 1
    ));
    body
}
