# Memory Engine Strategy

**ADR-011:** ACCEPTED

## Decision

R1 does **not** ship a full memory-profile library. Isolation/API strategy is locked now:

1. Memory parsers run only as out-of-process analysis packs under the ADR-005 sandbox.
2. Import of Volatility-compatible structured output is permitted as interchange (`P0-LATER` profile expansion).
3. Native Trareon memory profiles for selected Windows/macOS/Linux kernels are a named **P0-LATER** release gate, not an implicit Volatility install dependency.

## Comparison summary

| Option | License | Security boundary | Determinism | R1 posture |
|---|---|---|---|---|
| Native Trareon profiles | MIT Lab-owned | pack sandbox | high | P0-LATER |
| Protocol-isolated bundled engine | depends | pack sandbox | medium | P0-LATER candidate |
| Silent external Volatility install | N/A | fails offline rule | variable | REJECTED |
