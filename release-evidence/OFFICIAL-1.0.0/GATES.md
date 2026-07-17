# Official R1.0.0 gates (O1–O12)

Storefront sell (DIST-001): use `STOREFRONT_SELL=1` gather — O12 storefront + checksums.  
O1–O3 / O8–O10 = **optional hardening/compliance**, not Lynk.id/Gumroad sell blockers.

- [ ] O1 Windows Authenticode — `windows-sig.txt` *(OPTIONAL hardening)*
- [ ] O2 macOS Developer ID + notarization — `macos-notarization.json` *(OPTIONAL)*
- [ ] O3 Linux signed package — `linux-sig.txt` + `linux-signing-pubkey.asc` *(OPTIONAL)*
- [ ] O4 CycloneDX SBOM + vuln/license review — `sbom.cdx.json`
- [ ] O5 SAST / secret scan / dependency review — CI URL in `MANIFEST.txt`
- [ ] O6 Capability matrix freeze SHA — `MANIFEST.txt`
- [ ] O7 Validated method dossiers — inventory in `MANIFEST.txt`
- [ ] O8 Indonesia legal/quality sign-off *(OPTIONAL compliance)*
- [ ] O9 External crypto review *(OPTIONAL compliance)*
- [ ] O10 Physical Win/macOS/Linux smoke *(OPTIONAL for unsigned sell)*
- [ ] O11 Bookmark + signed transfer — tests / `o11-bookmark-transfer.log`
- [ ] O12 Storefront publish + source-only tag; **no** GitHub installer assets — `dist/.../SHA256SUMS` + storefront URL
