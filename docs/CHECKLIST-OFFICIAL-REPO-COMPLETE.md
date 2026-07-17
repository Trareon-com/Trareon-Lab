# Official R1.0.0 — repository vs storefront sell

**Synced:** 2026-07-17  
**Spec:** `docs/superpowers/specs/2026-07-17-storefront-binary-distribution-design.md`  
**Sell channel:** Lynk.id / Gumroad (full binary, no in-app license)  
**GitHub:** source + CI only — **no** installer Release assets

## Created in-repo

- `docs/DISTRIBUTION-STOREFRONT.md`
- `scripts/publish-storefront-release.sh`, `scripts/check-no-github-binaries.sh`
- Selling/commercial copy aligned (no DRM)
- Official docs: signing/Windows = optional hardening
- `STOREFRONT_SELL=1` gather/cut path

## Storefront sell still needs (operator machine)

1. Build full binaries into `dist/<ver>/`
2. Write `SHA256SUMS`
3. Upload to Lynk.id / Gumroad with freeze SHA
4. Optional source-only `v1.0.0` tag
5. Confirm no GitHub installer assets

## Optional (not sell blockers)

Authenticode, notarization, Indonesia O8, crypto O9, ThinkPad smoke.
