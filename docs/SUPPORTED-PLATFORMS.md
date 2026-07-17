# Supported Platforms

**ADR-009:** ACCEPTED  
**Date:** 2026-07-17

| OS | Editions / versions | Arch | Installer | Signing / notarization | FS access limits | Privilege | Live-host limits | Validation coverage |
|---|---|---|---|---|---|---|---|---|
| Windows | Windows 10 22H2; Windows 11 23H2/24H2 | x64 | `.msi` / `.exe` (Slint packager) | Authenticode required for Official Production | user-mode read; admin only for selected live ops | standard user default | live preview requires explicit authorization | CI + ThinkPad X270 physical |
| macOS | macOS 14, 15 | arm64 (Apple Silicon primary); x86_64 secondary P0-LATER | `.dmg` / `.pkg` | Developer ID + notarization for Official Production | TCC prompts for removable volumes | standard user | live host ops limited | CI + MacBook M4 Pro physical |
| Linux | Ubuntu 22.04/24.04 LTS; Debian 12; Kali rolling (lab) | x86_64 | `.deb` + AppImage | signed repo/appimage for Official Production | FUSE/udisks constraints documented | standard user | live ops limited | CI + Kali physical |

No umbrella “Windows/macOS/Linux” claim beyond this matrix.
