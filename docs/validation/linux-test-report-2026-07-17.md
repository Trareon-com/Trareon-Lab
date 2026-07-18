# Linux Test Validation Report

**Date:** 2026-07-17
**Host:** Kali Linux, x86_64
**Rust:** 1.96.0 (stable)
**Toolchain required:** 1.95.0 (pinned in rust-toolchain.toml) — 1.96 is forward-compatible
**Environment:** `/tmp` mounted on tmpfs (12 GB), `/dev/nvme0n1p1` root (453 GB, 414 GB used)
**Slint GUI deps:** cmake 3.31.6 (portable), pkg-config, libfontconfig, libfreetype, libxkbcommon, libwayland, libxcb-*, libgl1-mesa

---

## Trearon Lab — Full Test Results

| # | Test | Command | Result | Notes |
|---|------|---------|--------|-------|
| 1 | **Format check** | `cargo fmt --all --check` | ✅ PASS | — |
| 2 | **Clippy workspace** | `cargo clippy --workspace --all-targets --exclude lab-slint -- -D warnings` | ✅ PASS | — |
| 3 | **Clippy GUI** | `cargo clippy -p lab-slint --all-targets --features gui -- -D warnings` | ✅ PASS | — |
| 4 | **Workspace tests** | `cargo test --workspace --exclude lab-slint` | ✅ PASS | All crates pass |
| 5 | **GUI tests (no-features)** | `cargo test -p lab-slint --no-default-features --tests` | ✅ PASS | 3 UI model tests + 1 validation hooks test |
| 6 | **GUI tests (gui features)** | `cargo test -p lab-slint --features gui` | ✅ PASS | Same 4 tests + docs shell |
| 7 | **Artifact parsers** | `cargo test -p lab-artifacts --tests` | ✅ PASS | Prefetch, LNK, JumpList, macOS Unified Log, Linux auth/syslog parsers all OK |
| 8 | **Transfer sign/verify** | `cargo test -p lab-transfer --tests` | ✅ PASS | Export-import OK + tamper rejected |
| 9 | **Timeline merge** | `cargo test -p lab-timeline --lib` | ✅ PASS | Merge sort test |
| 10 | **Packaging smoke** | `./packaging/smoke.sh` | ✅ PASS | Smoke artifact generated |
| 11 | **E2E core path** | `./scripts/e2e-smoke.sh` (incl. packaging smoke) | ✅ PASS | workspace tests + artifacts + transfer + timeline + packaging all OK |
| 12 | **GUI build** | `cargo build -p lab-slint --features gui --locked` | ✅ PASS | Slint + rusqlite + all crates compile |
| 13 | **GUI build (no-features)** | `cargo build -p lab-slint --no-default-features` | ✅ PASS | Headless build |
| 14 | **Secret pattern scan** | grep for private keys, AWS keys | ✅ PASS | No secrets committed |

**Overall: 14/14 ✅ PASS**

---

## Trearon Acquire — Full Test Results

(Full report at `docs/platform/linux-test-report-2026-07-17.md` in Trareon-Acquire repo)

| # | Test | Result |
|---|------|--------|
| 1 | Format check | ✅ PASS |
| 2 | Clippy workspace | ✅ PASS |
| 3 | Clippy GUI | ✅ PASS |
| 4 | Workspace tests | ✅ PASS |
| 5 | GUI tests | ✅ PASS |
| 6 | Property/fuzz tests | ✅ PASS (9 tests) |
| 7 | Performance baseline | ✅ PASS (3/4, 1 ignored) |
| 8 | Golden fixture verify | ✅ PASS |
| 9 | Mutated/truncated/removed reject | ✅ PASS (3/3, exit code 2) |
| 10 | Foundation demo | ✅ PASS |
| 11 | Secret pattern scan | ✅ PASS |

---

## Environment Notes

1. **Disk space:** `/tmp` tmpfs has 12 GB capacity. Rust incremental builds + debug symbols can consume ~6 GB per repo. Parallel builds of both repos can exhaust space. Sequential builds or `rm -rf target/debug/incremental` between builds resolves this.

2. **Missing deps on stock Kali:** `cmake` is not pre-installed. Installed via portable binary from GitHub releases (cmake-3.31.6-linux-x86_64). Slint GUI requires: pkg-config, libfontconfig1-dev, libfreetype6-dev, libx11-dev, libxcursor-dev, libxkbcommon-dev, libwayland-dev, libgl1-mesa-dev.

3. **No sudo available:** All Slint system deps were pre-installed on this Kali system. cmake was installed as a portable binary in `/tmp/cmake-3.31.6-linux-x86_64/bin/`.

4. **Rust version:** System has rustc 1.96.0. The project pins 1.95.0 in `rust-toolchain.toml`. All tests pass on 1.96.0 with no compatibility issues.

5. **Headless GUI tests:** Tests pass without a display server (headless CI). The GUI tests use `--no-default-features` or headless test configurations.

---

## Signed-off

**Validation performed by:** Tohka Yatogami (Hermes Agent)
**Test environment:** `Linux kali 6.18.9-kali-amd64`
**Timestamp:** 2026-07-17T22:45:00Z
