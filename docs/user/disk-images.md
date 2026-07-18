# Disk images (user guide draft)

**Status:** Engineering / lab use. Unsigned builds — see `docs/SELLING-UNSIGNED.md`.

## Supported in current slice

| Format | Status |
|---|---|
| Raw / dd / `.img` | Supported — open, SHA-256, import to case registry |
| E01 | **Validated subset** — single-segment EWF v1; see `E01-DOSSIER.md` |
| Ex01 / EWF2 | Unsupported — fail closed |
| Other extensions | Unsupported — fail closed |

## Import flow

1. Open a case (exclusive lock).
2. Choose **Import Evidence** (UI) or call `lab_storage::import_raw_image`.
3. Lab streams SHA-256; cancel mid-hash yields **no** stored digest.
4. Evidence + provenance rows are append-only on the case DB.

## Hostile input

Missing, empty, truncated, or unsupported containers must fail closed. Lab never silently repairs image bytes.

## What this is not

- Not a claim of complete E01 producer compatibility or multi-segment support.
- Not court-ready packaging or ISO accreditation.
