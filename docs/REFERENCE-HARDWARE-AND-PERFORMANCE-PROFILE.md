# Reference Hardware and Performance Profile

**ADR-009 companion**

## Tiers

| Tier | CPU | RAM | System SSD | Evidence storage | Sustained R/W | GPU | Display |
|---|---|---|---|---|---|---|---|
| Minimum field/lab | 4c/8t x86_64 or Apple M1 | 16 GiB | 512 GiB | USB3 HDD/SSD 1 TB | ≥100 MB/s read | none required | 1920×1080 |
| Recommended examiner | 8c/16t or Apple M2/M3 | 32 GiB | 1 TB NVMe | dedicated NVMe 2 TB | ≥500 MB/s | optional | 2560×1440 |
| High-volume | 16c+ or Apple M4 Pro/Max | 64 GiB+ | 2 TB NVMe | RAID/NVMe shelf | ≥1 GB/s | optional decode assist | 3840×2160 |

Gate A reference machines: MacBook M4 Pro (Recommended/High-volume class), ThinkPad X270 (Minimum/older), Kali laptop (Minimum/Recommended mix).
