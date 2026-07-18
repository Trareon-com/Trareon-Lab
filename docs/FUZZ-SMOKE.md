# Fuzz smoke

No `fuzz/fuzz_targets/*.rs` targets are currently checked in. This is an
explicit deferred state, not a fuzz PASS. Target ownership and crash triage are
defined in [FUZZ-OFFICIAL-PREP.md](FUZZ-OFFICIAL-PREP.md).

Until cargo-fuzz targets land, run the bounded hostile-input smoke:

```bash
python3 - <<'PY'
import subprocess

for command in (
    ["cargo", "test", "-p", "lab-fsnap", "--test", "hostile_fsnap"],
    ["cargo", "test", "-p", "lab-transfer"],
    ["cargo", "test", "-p", "lab-core", "--test", "schema_fixtures"],
):
    subprocess.run(command, check=True, timeout=120)
PY
```

Each command has a 120-second ceiling. A timeout or non-zero exit is a failed
smoke, and no fuzz-complete claim may be made from this substitute.
