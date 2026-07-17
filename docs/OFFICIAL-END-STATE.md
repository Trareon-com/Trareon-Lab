# Official end-state — what “selesai” means

## Repo creatable track — COMPLETE when

- All plan rows except the human-blocked 8 are `[x]`
- Go-live scripts exist and fail closed without evidence
- Windows lab work is listed only in `docs/WINDOWS-LAB-QUEUE.md`
- No Official gate is marked PASS without real artifacts

**Current:** COMPLETE as of tip after go-live + transfer-security gap fill.

## Human + lab track — COMPLETE only when

1. Path C certs (Apple + Authenticode + Linux key)
2. Path D: crypto review received + accepted; Indonesia wet/digital sign-off
3. `docs/MACOS-LINUX-SIGNING-QUEUE.md` M1–L4 done
4. `docs/WINDOWS-LAB-QUEUE.md` W1–W6 done **last**
5. `gather.sh` exits 0
6. `cut-official-v1.sh` → `publish-official-release.sh` → `close-official-program.sh`
7. PRD status = Official Production 1.0.0 released

## Honesty

Engineering Alpha / `v1.0.0-rc1-unsigned` ≠ Official Production. Do not invent signatures, Team IDs, or reviewer names.
