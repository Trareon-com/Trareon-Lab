# Synthetic exFAT corpus (exfat-synth-v1)

| Field | Value |
|---|---|
| Class | synthetic |
| Format | `TRFSEXF1` (`*.trfsex`) |
| Purpose | Day 17 exFAT enumeration semantics |
| Redistribution | allowed |
| Sensitivity | none |

## Recipe

Same row layout as FAT32 synth; distinct magic only. Root cluster `0`; paths use `/`.

Demo: `validation/synthetic/exfat/exfat-synth-demo.trfsex`.

## Non-claims

Not a full exFAT boot/allocation bitmap parser.
