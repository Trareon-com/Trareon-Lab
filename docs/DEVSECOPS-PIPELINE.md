# DevSecOps pipeline map (NFR-SEC-008)

Maps each Official security-release obligation to the CI job and artifact path that currently implements or blocks it.

| NFR-SEC-008 bullet | CI / process | Artifact / evidence path | Status |
|---|---|---|---|
| SBOM | Job `sbom-cargo-metadata` | `release-evidence/sbom/sbom-cargo-metadata.json`, `package-inventory.txt` (CI artifact); Official CycloneDX still required at O4 | PARTIAL — cargo metadata inventory exists; CycloneDX for Official tag remains operator/CI upgrade |
| Dependency / license policy | `docs/DEPENDENCY-AND-LICENSE-POLICY.md` + `cargo audit` | Policy docs; audit ignores in `.cargo/audit.toml` / `docs/DEPENDENCY-AUDIT.md` | IMPLEMENTED for advisory gate |
| Vulnerability review | Job `cargo-audit` fails on unignored HIGH+ advisories | CI run URL recorded into Official manifest at release | IMPLEMENTED |
| SAST | Clippy `-D warnings` in `cargo-test-workspace`; optional `cargo-geiger` warn-only job | CI logs | IMPLEMENTED (Clippy); geiger warn-only |
| Secret scan | Job `secrets` (gitleaks OSS CLI; no org license required) | CI logs | IMPLEMENTED |
| Provenance / attestation | Release gather + signed package evidence | `release-evidence/OFFICIAL-1.0.0/` | BLOCKED until O1–O3 signed artifacts exist |
| Platform signing status | Packaging scripts + Official runbook O1–O3 | `windows-sig.txt`, `macos-notarization.json`, `linux-sig.txt` | BLOCKED — certs / notarization are operator Path C |

## Job inventory (`.github/workflows/ci.yml`)

| Job | Purpose |
|---|---|
| `cargo-test-workspace` | Tests, Clippy `-D warnings`, `cargo fmt --check` |
| `cargo-audit` | Dependency vulnerability gate |
| `secrets` | Gitleaks secret scan |
| `sbom-cargo-metadata` | Dependency inventory SBOM upload |
| `cargo-geiger` | Unsafe-code inventory (warn-only; does not fail the build) |
