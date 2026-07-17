# Official signed packaging pipelines (scripts + env contracts)

Secrets stay in CI/operator environment variables. This document is the repository-side pipeline contract for O1–O3.

## macOS (O2)

```bash
# Required env (never commit values):
# APPLE_TEAM_ID, APPLE_ID, APPLE_APP_SPECIFIC_PASSWORD, DEVELOPER_ID_APPLICATION
# Build unsigned first, then:
# codesign --sign "$DEVELOPER_ID_APPLICATION" ...
# xcrun notarytool submit ... --wait
# xcrun stapler staple ...
# Write ticket JSON to release-evidence/OFFICIAL-1.0.0/macos-notarization.json
```

Dry-run without certs: `bash packaging/signing-dry-run.sh`

## Windows (O1)

```powershell
# Required env: WINDOWS_CERT_PATH / WINDOWS_CERT_PASSWORD (or hardware token)
# signtool sign /fd SHA256 ...
# Record verification output to release-evidence/OFFICIAL-1.0.0/windows-sig.txt
```

## Linux (O3)

Follow `docs/LINUX-PACKAGE-SIGNING-KEY-PLAN.md`. Sign `.deb`/AppImage; write `linux-sig.txt` and commit only `linux-signing-pubkey.asc`.

## Status

Pipeline contracts: READY.  
Signed artifact production: BLOCKED on Path C certificates/keys.
