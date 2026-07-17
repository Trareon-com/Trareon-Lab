# Final gates — storefront sell vs optional compliance

Repo artifacts for storefront sell are creatable without Path C certs or wet signatures.

## Sell gates (required for Lynk.id / Gumroad v1)

| # | Gate | Artifact / script | Blocker |
|---|---|---|---|
| 1 | Full local binaries | `dist/<ver>/` | Operator build |
| 2 | Checksums | `dist/<ver>/SHA256SUMS` | Operator |
| 3 | Storefront upload | Lynk.id / Gumroad product page | Operator |
| 4 | Freeze SHA disclosed | product page + optional source tag | Operator |
| 5 | No GitHub installer assets | `scripts/check-no-github-binaries.sh` | Must stay clean |
| 6 | Selling honesty | `docs/SELLING-PAGE.md` + `SELLING-UNSIGNED.md` | Copy review |

```bash
bash scripts/publish-storefront-release.sh 1.0.0
STOREFRONT_SELL=1 bash scripts/cut-official-v1.sh v1.0.0   # optional source-only tag
bash scripts/check-no-github-binaries.sh v1.0.0
```

## Optional compliance (not sell blockers)

| Gate | Notes |
|---|---|
| Authenticode / notarization / Linux sig | `docs/MACOS-LINUX-SIGNING-QUEUE.md`, `docs/WINDOWS-LAB-QUEUE.md` |
| Indonesia wet/digital sign-off O8 | `docs/operator/OBTAIN-SIGNOFFS.md` |
| External crypto review O9 | receipt + acceptance |
| Strict `gather.sh` (unset STOREFRONT_SELL) | signed evidence path |

Do **not** use `gh release create` to attach product binaries.
