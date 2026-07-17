# Official end-state — what “selesai” means

**Distribution model:** `docs/superpowers/specs/2026-07-17-storefront-binary-distribution-design.md`  
**Operator path:** `docs/DISTRIBUTION-STOREFRONT.md`

## Repo creatable track — COMPLETE when

- Storefront docs + publish scripts exist and refuse GitHub installer uploads
- No in-app license/DRM required for sell
- Windows/signing work listed as optional hardening queues

## Storefront sell track — COMPLETE when

1. Full binaries built into local `dist/<ver>/` + `SHA256SUMS`
2. Uploaded to Lynk.id / Gumroad with freeze SHA + GPL source link
3. Optional source-only tag `v1.0.0` (no GitHub binary assets)
4. Selling page honesty intact (`docs/SELLING-PAGE.md`)

```bash
STOREFRONT_SELL=1 bash scripts/cut-official-v1.sh v1.0.0   # after dist/SHA256SUMS exists
bash scripts/publish-storefront-release.sh 1.0.0
bash scripts/check-no-github-binaries.sh v1.0.0
```

## Optional hardening (not sell blockers)

- Path C signing / notarization / Authenticode (`docs/MACOS-LINUX-SIGNING-QUEUE.md`)
- Windows lab queue W1–W6 (`docs/WINDOWS-LAB-QUEUE.md`)
- External crypto review / Indonesia wet sign-off (enterprise/compliance customers)

## Honesty

Engineering Alpha / `v1.0.0-rc1-unsigned` ≠ “signed Official enterprise pack”. Storefront v1 may be full + unsigned. Do not invent signatures or attach installers to GitHub Releases.
