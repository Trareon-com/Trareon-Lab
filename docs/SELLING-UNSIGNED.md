# Menjual / memasang Trareon Lab tanpa code signing berbayar

**Binary delivery:** Lynk.id / Gumroad only — see `docs/DISTRIBUTION-STOREFRONT.md`. Do not publish installers on GitHub.

Build distribusi Trareon Lab **unsigned** (tidak memakai Apple Developer ID, Authenticode, atau notarization). Ini legal untuk distribusi sendiri; OS akan memperingatkan. Ikuti langkah di bawah.

## macOS (Gatekeeper)

1. Unduh `.app.zip` atau `.dmg` dari rilis.
2. Jika muncul “cannot be opened because the developer cannot be verified”:
   - **System Settings → Privacy & Security** → buka saja / Open Anyway, **atau**
   - Terminal: `xattr -dr com.apple.quarantine /path/to/Trareon\ Lab.app`
3. Jangan bagikan instruksi yang menonaktifkan Gatekeeper global.

## Windows (SmartScreen)

1. Unduh `.zip` portable atau installer unsigned.
2. Jika SmartScreen muncul: **More info → Run anyway**.
3. Opsional (admin): `Unblock-File` di PowerShell pada file unduhan.
4. Antivirus pihak ketiga mungkin memindai binary Rust baru — whitelist folder aplikasi bila false positive.

## Linux

1. Unduh `.tar.gz` atau AppImage.
2. `chmod +x` pada binary / AppImage.
3. AppImage: jalankan langsung; `.tar.gz`: ekstrak lalu jalankan binary di folder release.
4. Beberapa distro membatasi FUSE untuk AppImage — sediakan `.tar.gz` sebagai fallback.

## Verifikasi integritas (pengganti “trust dari OS signature”)

Setiap rilis harus menyertakan `SHA256SUMS.txt`:

```bash
# macOS / Linux
shasum -a 256 -c SHA256SUMS.txt
# atau
sha256sum -c SHA256SUMS.txt
```

Windows (PowerShell): bandingkan hash file dengan nilai di `SHA256SUMS.txt`.

## Kalimat jujur untuk pelanggan

Trareon Lab didistribusikan **tanpa tanda tangan vendor OS** agar harga tetap rendah. Keaslian paket diverifikasi lewat **SHA-256** yang dipublikasikan. Cocok untuk laboratorium / pelatihan; bukan klaim akreditasi atau “official notarized”.
