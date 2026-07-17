# Storefront distribution (source public, binary private)

**Spec:** `docs/superpowers/specs/2026-07-17-storefront-binary-distribution-design.md`

## Rule

- GitHub = source + docs + CI tests only.
- Product binaries live on the operator machine and on Lynk.id / Gumroad only.
- Never attach installers to GitHub Releases.

## Build (full binary, no license gate)

```bash
# macOS / Linux examples — see also docs/PACKAGING-UNSIGNED-BUILDS.md
./packaging/build-macos-unsigned.sh   # or build-linux-unsigned.sh
# Windows: follow packaging/WINDOWS-UNSIGNED.md on a Windows host
```

Artifacts under `dist/` (gitignored).

## Checksums

```bash
mkdir -p dist/1.0.0
# after copying platform artifacts into dist/1.0.0/<platform>/
(cd dist/1.0.0 && find . -type f ! -name SHA256SUMS -print0 | sort -z | xargs -0 shasum -a 256) > dist/1.0.0/SHA256SUMS
```

## Upload

1. Upload zip/tarball + `SHA256SUMS` to Lynk.id and/or Gumroad.
2. On the product page list: freeze git SHA, OS matrix, link to public GitHub source, GPL-3.0-only, unsigned/SmartScreen/Gatekeeper warnings (`docs/SELLING-UNSIGNED.md`).

## Optional source-only tag

```bash
git tag -a v1.0.0 -m "source freeze for storefront v1.0.0"
# Do NOT gh release upload binaries
```

## Optional signing

Signing scripts under `packaging/sign-*.sh` remain available for later hardening. They are not required to sell.
