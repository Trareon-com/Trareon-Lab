# Official signed packaging pipelines (scripts + env contracts)

Secrets stay in CI/operator environment variables.

| OS | Script | Evidence |
|---|---|---|
| macOS O2 | `packaging/sign-macos.sh` | `release-evidence/OFFICIAL-1.0.0/macos-notarization.json` |
| Linux O3 | `packaging/sign-linux.sh` | `linux-sig.txt` + `linux-signing-pubkey.asc` |
| Windows O1 | `packaging/sign-windows.ps1` (**Windows lab queue**) | `windows-sig.txt` |
| Verify | `packaging/verify-signatures.sh` | non-empty evidence files |
| Orchestrate | `packaging/rebuild-signed-from-freeze.sh` | then `gather.sh` |

Dry-run without certs: `bash packaging/signing-dry-run.sh`  
Windows execution order: `docs/WINDOWS-LAB-QUEUE.md`

## Status

Pipeline scripts: READY (fail closed without env).  
Signed artifact production: BLOCKED on Path C certificates/keys + Windows lab.
