# Performance Benchmark Protocol

Each metric records dataset identity, warm/cold state, concurrency, percentile, hardware tier, pass threshold, and measurement tool.

| Metric | Dataset | State | Percentile | Tier | Pass threshold (R1 target) | Tool |
|---|---|---|---|---|---|---|
| Cold start | empty | cold | n/a | Recommended | ≤3 s UI ready | OS timer / spike harness |
| Case open | synthetic 1M metadata | cold | n/a | Recommended | ≤5 s to first page | lab harness |
| `.fsnap` manifest verify | 1 GiB package fixture | cold | n/a | Recommended | ≥200 MB/s equivalent | importer |
| Hashing throughput | 10 GiB sequential | warm | n/a | Recommended | ≥300 MB/s SHA-256 | core hasher |
| Indexing throughput | 1M metadata rows | cold | n/a | Recommended | ≤60 s | D-RUST-INDEX |
| Search | 1M rows | warm | p50/p95 | Recommended | p95 ≤50 ms filter | UI measure |
| Timeline first render | R1 artifact set | cold | n/a | Recommended | ≤5 s | UI timer |
| 1M-row navigation | synthetic table | warm | p95 | Recommended | ≤50 ms page | UI measure |
| Memory overhead / indexed item | 1M | steady | n/a | Recommended | documented MiB/item | RSS probe |
| Cancellation latency | hash job | running | n/a | Recommended | ≤500 ms | spike cancel_ms |
| Crash recovery | mid-write | n/a | n/a | all | PASS reopen | lock tests |
| Report generation | sealed case draft | cold | n/a | Recommended | ≤10 s HTML/JSON | exporter |
| Temp storage amplification | 10 GiB image | n/a | n/a | Recommended | ≤1.5× working set | disk meter |

## Degraded operation

Low-disk, low-memory, slow removable media, network share, encrypted source, corrupted source, and thermal throttling must surface degraded confidence/coverage and must not silently skip work to meet a performance target.
