# Design: Storefront binary distribution (source public, binary private)

**Date:** 2026-07-17  
**Status:** ACCEPTED (user approved 2026-07-17)  
**Decision:** Option 1 + Approach A with light B  
**Related:** `docs/SELLING-PAGE.md`, `docs/COMMERCIAL-TERMS.md`, `docs/PACKAGING-UNSIGNED-BUILDS.md`, Official plan O12

## 1. Problem

The Official track assumed GitHub Releases with signed installers and heavy Path C/D gates. The commercial model is different: sell a **full** binary on Lynk.id / Gumroad, with **no in-app license management**, and keep binaries **off GitHub** (built and stored only on the operator’s machines).

## 2. Goals

- Ship a full-featured desktop Lab binary per supported OS (no trial, no license key, no online activation, no DRM).
- Public GitHub repo remains the **source** home under GPL-3.0-only.
- Paid delivery happens only via storefront(s) (Lynk.id and/or Gumroad).
- GitHub must not publish downloadable installers (no Release assets, no public workflow artifacts that are the product binary).
- Honesty: unsigned/smartscreen limits, GPL obligations, and non-accreditation claims stay visible on the selling page.

## 3. Non-goals

- In-app license server, seat counting, phone-home, watermarking, or feature flags gated by purchase.
- Changing away from GPL-3.0-only for this edition.
- Blocking sales on Apple/Windows/Linux code-signing certificates (signing is optional hardening, not a sell gate).
- ISO / court-readiness claims.
- Hosting binaries on GitHub “temporarily” or “for CI only” in a publicly downloadable form.

## 4. Distribution model

```text
[public GitHub: source + docs + CI tests]
        │
        │  operator builds offline on owned machines
        ▼
[local dist/ + SHA256SUMS + optional SBOM]  ← never committed; already gitignored
        │
        │  manual upload
        ▼
[Lynk.id / Gumroad product page] ──► buyer download
```

| Channel | What ships |
|---|---|
| GitHub | Source, docs, CI (tests/lint/audit). Optional **source-only** annotated tag (e.g. `v1.0.0`). |
| Operator disk | Release binaries, checksums, local evidence notes. |
| Lynk.id / Gumroad | The product zip/tarball buyers pay for. |

## 5. Product behavior

- One SKU mindset: **full Lab** for the declared OS matrix.
- Purchase proof lives at the storefront (email/receipt), not inside the app.
- About / docs may state GPL + “commercial copy obtained via Trareon storefront” without enforcing it in code.

## 6. GPL note (honest constraint)

GPL-3.0-only allows selling binaries. Recipients who receive the binary are entitled to corresponding source. With a **public** GitHub repo pointed from the selling page / `LICENSE`, that obligation is met without bundling a second private source tree—as long as the sold binary matches a public tag/commit buyers can identify (publish the freeze SHA next to the download).

## 7. Changes to Official / sellable docs (intent)

| Old assumption | New assumption |
|---|---|
| O12 = GitHub Release with signed installers | O12 = storefront publish + optional source-only git tag; local checksum evidence |
| Signing/notarization blocks “Official sell” | Signing optional; SmartScreen/Gatekeeper warnings disclosed |
| Windows lab / Authenticode required before sell | Windows lab queued for later hardening, not sell blocker |
| In-app or commercial license management | None |

Go-live scripts that attach binaries to `gh release create` must be rewritten or retired for this edition so they cannot accidentally publish product binaries.

## 8. Operator checklist (sell path)

1. Freeze source tip; optional `git tag -a v1.0.0` (source only).
2. Build full unsigned (or optionally signed) packages into local `dist/` per OS.
3. Write `SHA256SUMS` next to artifacts; keep offline.
4. Upload to Lynk.id / Gumroad; list freeze SHA + OS matrix + unsigned warnings + GPL link to GitHub.
5. Do not attach binaries to GitHub Release.

## 9. Risks

| Risk | Mitigation |
|---|---|
| Buyers redistribute binary freely (GPL) | Accepted; price is convenience/support/build, not DRM |
| SmartScreen / Gatekeeper friction | Disclose on selling page; optional later signing |
| Accidental GitHub binary publish | `.gitignore` `dist/`; rewrite publish scripts; CI must not upload product zips as public artifacts |
| Binary ≠ claimed SHA | Always publish freeze SHA + checksums on storefront |

## 10. Success criteria

- No license/activation code paths required for v1 sellable build.
- Selling docs describe storefront-only binary delivery.
- Official/sellable plans no longer require GitHub binary Release as the sell gate.
- Operator can build and sell from an offline machine without publishing installers to GitHub.

## 11. Open follow-ups (out of this spec’s implementation unless requested)

- Exact Gumroad vs Lynk.id copy/pricing.
- Whether CI retains SBOM as a non-installer artifact only.
- Later optional Authenticode/notarization as a paid “signed build” SKU.
