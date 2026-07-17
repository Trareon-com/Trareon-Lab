# Final 8 Official gates — creatable vs human-blocked

Repo artifacts for go-live are complete. These eight plan rows stay open until real evidence exists.

| # | Gate | Creatable artifact (done) | Blocker |
|---|---|---|---|
| 1 | Crypto review received | `CRYPTO-EXTERNAL-REVIEW-RECEIPT.md` + stub | Named reviewer document |
| 2 | Indonesia wet/digital sign-off O8 | `OBTAIN-SIGNOFFS.md` + sign-off template | Human signature |
| 3 | Crypto review accepted O9 | Human signoffs metadata | Release Manager acceptance |
| 4 | Final gather PASS | `gather.sh` fail-closed | O1–O12 files |
| 5 | Annotated `v1.0.0` | `scripts/cut-official-v1.sh` | gather PASS |
| 6 | GitHub Release signed assets | `scripts/publish-official-release.sh` | signed binaries + evidence |
| 7 | Push tag + release | included in publish script | same |
| 8 | Close program / PRD released | `scripts/close-official-program.sh` | after publish |

## Execution order (after Path C/D + Windows lab)

```bash
# After O1–O11 evidence lands (Windows via docs/WINDOWS-LAB-QUEUE.md):
bash scripts/generate-official-sbom.sh                   # O4 if missing
bash release-evidence/OFFICIAL-1.0.0/gather.sh          # must exit 0
bash scripts/cut-official-v1.sh v1.0.0
bash scripts/publish-official-release.sh v1.0.0
bash scripts/close-official-program.sh v1.0.0
# then commit PRD/program-closed docs and follow POST-RELEASE-24H.md
```
