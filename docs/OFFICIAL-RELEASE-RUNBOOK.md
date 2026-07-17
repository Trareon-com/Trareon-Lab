# Trareon Lab Official R1.0.0 Release Runbook

**Target date:** 2026-10-16  
**Program state:** ACTIVE — **storefront sell** uses DIST-001; signed O1–O3 are optional hardening  
**Source plan:** `docs/superpowers/plans/2026-07-17-trareon-lab-3month-official-release.md`  
**Distribution spec:** `docs/superpowers/specs/2026-07-17-storefront-binary-distribution-design.md`

## Status rules

- `NOT_STARTED`: required evidence has not been recorded.
- `IN_PROGRESS`: work has started, but the gate evidence is incomplete.
- `PASS`: the named owner reviewed the evidence and accepted the gate.
- `BLOCKED`: an external dependency prevents progress; record the dependency and next decision date.
- `FAIL`: evidence does not satisfy the gate.
- `OPTIONAL`: hardening/compliance; **not** a Lynk.id/Gumroad sell blocker.

No gate may be marked `PASS` from a synthetic fixture when the gate requires stronger evidence. Storefront sell must not attach installers to GitHub Releases.

## Official Definition of Done

| Gate | Requirement | Accountable owner | Required evidence | Current status |
|---|---|---|---|---|
| O1 | Signed Windows installer (Authenticode) | Release Engineering | `release-evidence/OFFICIAL-1.0.0/windows-sig.txt` | OPTIONAL (not sell blocker) |
| O2 | macOS Developer ID signature and notarization staple | Release Engineering | `release-evidence/OFFICIAL-1.0.0/macos-notarization.json` | OPTIONAL |
| O3 | Signed Linux package | Release Engineering | `release-evidence/OFFICIAL-1.0.0/linux-sig.txt` | OPTIONAL |
| O4 | CycloneDX SBOM, license inventory, and vulnerability review | Security Engineering | `sbom.cdx.json` plus review record | IN_PROGRESS (fallback SBOM present) |
| O5 | SAST, secret scan, and dependency review green on freeze tip | Security Engineering | CI run URL in `MANIFEST.txt` | IN_PROGRESS |
| O6 | Capability matrix frozen with honest claims | Product Lead | matrix + freeze SHA | IN_PROGRESS |
| O7 | Every `Validated` method has dossier + offline docs | Validation Lead | `docs/validation/` inventory | IN_PROGRESS |
| O8 | Indonesia legal and quality sign-off | Legal/Quality Reviewer | `INDONESIA-OFFICIAL-SIGNOFF.md` | OPTIONAL (compliance) |
| O9 | External cryptography review or ADR waiver | Security Lead | `CRYPTO-EXTERNAL-REVIEW.md` | OPTIONAL (compliance) |
| O10 | Physical validation on reference machines | QA Lead | `o10/` smoke logs | OPTIONAL for unsigned sell |
| O11 | Bookmark + signed transfer verified | Product Lead | tests + optional O11 log | IN_PROGRESS (Alpha review PASS) |
| O12 | Storefront publish of full binary + source-only tag; **no** GitHub installer assets | Release Manager | Storefront URL + freeze SHA + local `dist/.../SHA256SUMS`; `scripts/check-no-github-binaries.sh` | NOT_STARTED |

## External dependency register

| Dependency | Owner | Current record (2026-07-17) | Required next evidence |
|---|---|---|---|
| Apple Developer Program | Release Engineering | OPTIONAL for sell; NOT_STARTED | Only if pursuing notarized builds |
| Windows Authenticode certificate | Release Engineering | OPTIONAL for sell; queued in WINDOWS-LAB-QUEUE | Only if pursuing Authenticode |
| Linux package-signing key | Release Engineering | OPTIONAL; plan ready | Public key if signing |
| Indonesia legal/quality review | Product Lead | OPTIONAL compliance | Sign-off if enterprise claim needed |
| External cryptography review | Security Lead | OPTIONAL compliance | Review if production signing keys claimed |
| Storefront (Lynk.id / Gumroad) | Release Manager | REQUIRED for sell | Product page live + SHA256SUMS |

## Release decision

- **Storefront v1 sell:** full unsigned (or optionally signed) binaries via Lynk.id/Gumroad; source on GitHub; no in-app license; no GitHub installer assets. Use `STOREFRONT_SELL=1` gather/cut.
- **Signed enterprise pack:** only when optional O1–O3 (and chosen compliance gates) are real `PASS`. Do not relabel an unsigned storefront build as “notarized Official.”

## Go-live scripts (storefront sell)

1. Build into `dist/<ver>/` + `SHA256SUMS` — `docs/DISTRIBUTION-STOREFRONT.md`
2. `STOREFRONT_SELL=1 bash release-evidence/OFFICIAL-1.0.0/gather.sh`
3. `STOREFRONT_SELL=1 bash scripts/cut-official-v1.sh v1.0.0` (source-only tag)
4. `bash scripts/publish-storefront-release.sh 1.0.0` (checklist; manual upload)
5. `bash scripts/check-no-github-binaries.sh v1.0.0`

Hardening queues: `docs/MACOS-LINUX-SIGNING-QUEUE.md`, then `docs/WINDOWS-LAB-QUEUE.md`.  
End-state: `docs/OFFICIAL-END-STATE.md`. Sell gates: `docs/operator/FINAL-8-GATES.md`.
