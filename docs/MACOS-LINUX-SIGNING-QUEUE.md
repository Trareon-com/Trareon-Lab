# macOS / Linux signing queue (Path C — not Windows)

Windows Authenticode + ThinkPad smoke stay in `docs/WINDOWS-LAB-QUEUE.md` and run **last**.
These rows can proceed on Mac/Linux hosts whenever Apple Team ID / Linux signing key exist.

| # | Task | Artifact | Script / doc |
|---|---|---|---|
| M1 | Apple Developer Team ID + notarization credentials | operator record (no secrets in git) | `docs/operator/PATH-C-PROCUREMENT.md` |
| M2 | Build unsigned macOS package | `dist/1.0.0/macos-arm64/` | `packaging/build-macos-unsigned.sh` |
| M3 | Developer ID sign + notarize + staple | `release-evidence/OFFICIAL-1.0.0/macos-notarization.json` | `packaging/sign-macos.sh` |
| M4 | O10 MacBook smoke | `release-evidence/OFFICIAL-1.0.0/o10/macos-macbook.json` | fill template (leave `NOT_RUN` until done) |
| L1 | Generate Linux signing key offline | public key only | `docs/LINUX-PACKAGE-SIGNING-KEY-PLAN.md` |
| L2 | Build unsigned Linux tarball | `dist/1.0.0/linux-x64/` | `packaging/build-linux-unsigned.sh` |
| L3 | Detached sign + verify | `linux-sig.txt` + `linux-signing-pubkey.asc` | `packaging/sign-linux.sh` |
| L4 | O10 Kali/Linux smoke | `release-evidence/OFFICIAL-1.0.0/o10/linux-kali.json` | fill template |

## After Mac/Linux (and Windows W1–W6)

See `docs/WINDOWS-LAB-QUEUE.md` then `docs/operator/FINAL-8-GATES.md`.
