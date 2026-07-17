# Windows lab work queue (run at the end)

All Official steps that require a Windows host or Windows Authenticode hardware are collected here so macOS/Linux/repo work is not blocked.

## Queue (execute on ThinkPad / Windows reference machine)

| # | Task | Artifact | Script / doc |
|---|---|---|---|
| W1 | Buy/install Authenticode cert (Path C) | Procurement fields | `docs/operator/PATH-C-PROCUREMENT.md` |
| W2 | Build unsigned Windows binary | `dist/1.0.0/windows-x64/` | `packaging/WINDOWS-UNSIGNED.md` |
| W3 | Authenticode sign + verify | `release-evidence/OFFICIAL-1.0.0/windows-sig.txt` | `packaging/sign-windows.ps1` |
| W4 | Physical smoke O10 (ThinkPad) | `release-evidence/OFFICIAL-1.0.0/o10/windows-thinkpad.json` | template in `o10/` |
| W5 | Optional: MSI packaging if adopted | installer under `dist/1.0.0/` | operator choice |
| W6 | Attach Windows signed artifact to GitHub Release | Release assets | after O1–O12 |

## Rule

Do not mark O1 or O10 Windows rows `PASS` until W3/W4 evidence files exist. Repo scripts are ready; lab execution is deferred to this queue.
