# Synthetic FS content store (fs-content-synth-v1)

| Field | Value |
|---|---|
| Class | synthetic |
| Format | `TRFSCT01` (`*.trfsct`) |
| Purpose | Day 18 file-byte read → CAS ingest without a full volume image |
| Redistribution | allowed |
| Sensitivity | none |

## Recipe

1. Write payloads with `lab_fs::write_fs_content_synthetic`.
2. Keys are the same record/cluster numbers used by NTFS/FAT synth enumeration.
3. Ingest with `lab_fs::ingest_synth_file_to_cas` into `lab_store::CasStore`.

## Non-claims

Not a claim of NTFS/FAT cluster run decoding from a raw disk image.
