# Official R1.0.0 gates (O1–O12)

Unchecked until real evidence lands under this directory or the linked review path.

- [ ] O1 Windows Authenticode — `windows-sig.txt`
- [ ] O2 macOS Developer ID + notarization staple — `macos-notarization.json`
- [ ] O3 Linux signed package — `linux-sig.txt` (+ public key `linux-signing-pubkey.asc`)
- [ ] O4 CycloneDX SBOM + vuln/license review — `sbom.cdx.json`
- [ ] O5 SAST / secret scan / dependency review green on release tag — CI URL in `MANIFEST.txt`
- [ ] O6 Capability matrix freeze SHA — recorded in `MANIFEST.txt`
- [ ] O7 Every Validated method has dossier + offline docs — inventory in `MANIFEST.txt`
- [ ] O8 Indonesia legal/quality sign-off — `docs/reviews/INDONESIA-OFFICIAL-SIGNOFF.md`
- [ ] O9 External crypto review — `docs/reviews/CRYPTO-EXTERNAL-REVIEW.md`
- [ ] O10 Physical Win/macOS/Linux smoke logs — files under this directory referenced by `MANIFEST.txt`
- [ ] O11 Bookmark + signed transfer verification — tests/UI evidence paths in `MANIFEST.txt`
- [ ] O12 `v1.0.0` tag + GitHub Release + known issues + support period — release URL in `MANIFEST.txt`
