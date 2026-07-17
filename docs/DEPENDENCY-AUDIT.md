# Dependency audit notes

CI runs `cargo audit` with project config `.cargo/audit.toml`.

## Active ignores

| ID | Crate | Why ignored | Exit condition |
|---|---|---|---|
| RUSTSEC-2026-0194 | quick-xml 0.39.x | Transitive via `wayland-scanner` → Slint Linux GUI only; not used to parse forensic evidence XML. Cargo `^0.39` cannot select `0.41.x` under 0.x semver. | `wayland-scanner` (or Slint stack) depends on quick-xml `>=0.41` |
| RUSTSEC-2026-0195 | quick-xml 0.39.x | Same as above | Same |

Unmaintained warnings (bincode, paste, rustybuzz, ttf-parser) currently do not fail CI; track for future Slint upgrades.
