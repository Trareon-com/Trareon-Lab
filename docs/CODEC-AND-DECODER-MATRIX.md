# Codec and Decoder Matrix

**ADR-010:** ACCEPTED

| Media type | R1 decoder posture | License/patent notes | Sandbox | Metadata | Corruption handling | Validation | Fallback |
|---|---|---|---|---|---|---|---|
| JPEG/PNG | bundled audited decoder | permissive; patent low for baseline | pack | EXIF subset | fail soft + coverage | CORPUS_VALIDATED | unsupported |
| MP4/H.264 container parse | container parse only in R1; decode optional pack | patent risk on decode → pack gated | pack | container tags | fail closed on bomb | UNIT then CORPUS | hash-only |
| WAV/PCM | bundled | permissive | pack | basic | fail soft | CORPUS | unsupported |
| PDF | metadata + text extract subset | license review per library | pack | doc info | limit pages/time | CORPUS | unsupported |
| Deepfake/PRNU/speaker ID | out of R1 | N/A | N/A | N/A | N/A | N/A | P2 |

Legal/license review is required before promoting any decoder from pack-gated to bundled Official Production.
