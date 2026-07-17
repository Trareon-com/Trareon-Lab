# Windows lab work queue (run at the end)

**Role:** optional post-sell hardening (Authenticode / ThinkPad smoke). **Not required** to sell full binaries on Lynk.id / Gumroad.

All Official **signing** steps that require a Windows host or Windows Authenticode hardware are collected here so macOS/Linux/repo work is not blocked.

## Queue (execute on ThinkPad / Windows reference machine)

| # | Task | Artifact | Script / doc |
|---|---|---|---|
| W1 | Buy/install Authenticode cert (Path C) | Procurement fields | `docs/operator/PATH-C-PROCUREMENT.md` |
| W2 | Build unsigned Windows binary | `dist/1.0.0/windows-x64/` | `packaging/WINDOWS-UNSIGNED.md` |
| W3 | Authenticode sign + verify | `release-evidence/OFFICIAL-1.0.0/windows-sig.txt` | `packaging/sign-windows.ps1` |
| W4 | Physical smoke O10 (ThinkPad) | `release-evidence/OFFICIAL-1.0.0/o10/windows-thinkpad.json` | template in `o10/` |
| W5 | Optional: MSI packaging if adopted | installer under `dist/1.0.0/` | operator choice |
| W6 | Attach Windows signed artifact to GitHub Release | Release assets | after O1–O12 |

## After Mac/Linux signing (optional order)

Non-Windows Path C work: `docs/MACOS-LINUX-SIGNING-QUEUE.md`.

## After Windows lab (W1–W6)

Then run go-live (macOS/Linux host is fine once `windows-sig.txt` exists):

```bash
bash scripts/generate-official-sbom.sh   # O4 if missing
# fill docs/reviews/SBOM-VULN-LICENSE-REVIEW.md acceptance
# ensure o11-bookmark-transfer.log exists (from templates/)
bash release-evidence/OFFICIAL-1.0.0/gather.sh
bash scripts/cut-official-v1.sh v1.0.0
bash scripts/publish-official-release.sh v1.0.0
bash scripts/close-official-program.sh v1.0.0
```

See `docs/operator/FINAL-8-GATES.md` and `docs/OFFICIAL-END-STATE.md`.

## Rule

Do not mark O1 or O10 Windows rows `PASS` until W3/W4 evidence files exist. Repo scripts are ready; lab execution is deferred to this queue.
