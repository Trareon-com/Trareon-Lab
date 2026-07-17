# Day 10 — Index 100k measurement notes

Recorded by `cargo test -p lab-index --test measure_100k -- --nocapture`.

| Metric | Soft gate | Sample (dev macOS arm64, 2026-07-17) |
|---|---|---|
| Insert 100k rows (batched) | < 120s | **595 ms** |
| Reopen + count + last page | < 30s | **21 ms** |

CI runners may be slower; gates stay loose to avoid flakes while still catching pathological regressions.

## Day 6–9 smoke

```bash
cargo test -p lab-index --tests
cargo test -p lab-case --tests
cargo test -p lab-slint --no-default-features --tests
./packaging/smoke.sh
```
