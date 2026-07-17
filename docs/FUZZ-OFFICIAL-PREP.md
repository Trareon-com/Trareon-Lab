# cargo-fuzz nightlies (Official prep)

Fuzz targets are not checked in yet. Until Path C certs are less urgent, run adversarial unit tests already in-tree:

```bash
cargo test -p lab-fsnap
cargo test -p lab-transfer
cargo test -p lab-core --test schema_fixtures
```

When enabling cargo-fuzz:

1. `cargo install cargo-fuzz`
2. Add targets under `fuzz/fuzz_targets/` for `.fsnap` verify and transfer envelope parse
3. Triage crashes into `docs/reviews/OFFICIAL-DISCREPANCY-REGISTER.md`

Status: **DEFERRED** (tooling note only; not a silent PASS).
