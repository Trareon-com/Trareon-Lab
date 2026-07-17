# Trareon Lab Official R1.0.0 Release Runbook

**Target date:** 2026-10-16  
**Program state:** ACTIVE — Official release is blocked until O1–O12 are `PASS`  
**Source plan:** `docs/superpowers/plans/2026-07-17-trareon-lab-3month-official-release.md`

## Status rules

- `NOT_STARTED`: required evidence has not been recorded.
- `IN_PROGRESS`: work has started, but the gate evidence is incomplete.
- `PASS`: the named owner reviewed the evidence and accepted the gate.
- `BLOCKED`: an external dependency prevents progress; record the dependency and next decision date.
- `FAIL`: evidence does not satisfy the gate. Official release must not proceed.

No gate may be marked `PASS` from a synthetic fixture, unsigned artifact, draft review, or CI result from a different release tag when the gate requires stronger evidence.

## Official Definition of Done

| Gate | Requirement | Accountable owner | Required evidence | Current status |
|---|---|---|---|---|
| O1 | Signed Windows installer (Authenticode) | Release Engineering | `release-evidence/OFFICIAL-1.0.0/windows-sig.txt` | NOT_STARTED |
| O2 | macOS Developer ID signature and notarization staple | Release Engineering | `release-evidence/OFFICIAL-1.0.0/macos-notarization.json` | NOT_STARTED |
| O3 | Signed Linux `.deb` and/or AppImage | Release Engineering | `release-evidence/OFFICIAL-1.0.0/linux-sig.txt` | NOT_STARTED |
| O4 | CycloneDX SBOM, license inventory, and vulnerability review | Security Engineering | `release-evidence/OFFICIAL-1.0.0/sbom.cdx.json` plus review record | NOT_STARTED |
| O5 | SAST, secret scan, and dependency review green on the release tag | Security Engineering | CI run URL recorded in `release-evidence/OFFICIAL-1.0.0/MANIFEST.txt` | NOT_STARTED |
| O6 | Capability matrix frozen with no unsupported `Validated` claims | Product Lead | `docs/RELEASE-01-CAPABILITY-MATRIX.md` plus freeze SHA in the manifest | NOT_STARTED |
| O7 | Every `Validated` method has a dossier and offline documentation | Validation Lead | `docs/validation/` and `docs/user/` inventory in the manifest | NOT_STARTED |
| O8 | Indonesia legal and quality sign-off | Legal/Quality Reviewer | `docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md` | NOT_STARTED |
| O9 | External cryptography review or approved ADR waiver | Security Lead | `docs/reviews/CRYPTO-EXTERNAL-REVIEW.md` | NOT_STARTED |
| O10 | Physical validation on Windows, macOS, and Linux reference machines | QA Lead | Platform smoke logs under `release-evidence/OFFICIAL-1.0.0/` | NOT_STARTED |
| O11 | Bookmark workflow and signed transfer package verified | Product Lead | Tests, UI evidence, and transfer verification log in the manifest | NOT_STARTED |
| O12 | `v1.0.0` tag, GitHub Release, known issues, and support period published | Release Manager | Release URL and immutable tag SHA in the manifest | NOT_STARTED |

## External dependency register

This register records facts only. Missing commercial or reviewer details remain explicit blockers rather than guessed values.

| Dependency | Owner | Current record (2026-07-17) | Required next evidence |
|---|---|---|---|
| Apple Developer Program | Release Engineering | NOT_STARTED; no invoice or Team ID recorded | Invoice reference and Team ID stored in the operator record; no secret keys committed |
| Windows Authenticode certificate | Release Engineering | NOT_STARTED; no vendor or ETA recorded | Vendor, certificate type, order reference, and ETA |
| Linux package-signing key | Release Engineering | PLAN READY (`docs/LINUX-PACKAGE-SIGNING-KEY-PLAN.md`); public key not yet issued | Generate key offline; commit public key only to `release-evidence/OFFICIAL-1.0.0/linux-signing-pubkey.asc` |
| Indonesia legal/quality review | Product Lead | DRAFT READY (`docs/reviews/REVIEWER-BOOKING.md`); invitation not yet sent | Operator sends draft; record recipient + send date here; draft deadline 2026-09-25; sign-off 2026-10-09 |
| External cryptography review | Security Lead | DRAFT READY (`docs/reviews/REVIEWER-BOOKING.md`); invitation not yet sent | Operator sends draft; record recipient + send date here; deadline 2026-09-30 |

## Release decision

The Release Manager may publish Official R1.0.0 only when every O1–O12 row is `PASS`, the evidence manifest points to immutable artifacts, and the tag SHA matches the tested build. If signing or either external review is incomplete, publish at most an Engineering Alpha/RC with accurate claims; do not relabel it Official.
