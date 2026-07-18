# Golden corpus inventory

| Fixture path | Validation use |
|---|---|
| `fixtures/contracts/case.valid.json` | Valid CASE record |
| `fixtures/contracts/case.invalid-*.json` | Missing fields, enum, and integrity-link rejection |
| `fixtures/contracts/evidence.valid.json` | Valid evidence record |
| `fixtures/contracts/evidence.invalid-*.json` | Evidence contract rejection |
| `fixtures/contracts/bookmark.valid.json` | Valid bookmark record |
| `fixtures/contracts/bookmark.invalid-*.json` | Bookmark contract rejection |
| `fixtures/contracts/transfer.valid.json` | Valid transfer record |
| `fixtures/contracts/transfer.invalid-*.json` | Transfer contract rejection |
| `crates/lab-fs/tests/ntfs_real.rs` | Embedded/generated NTFS golden |
| `crates/lab-fs/tests/fat_ext4_real.rs` | Embedded/generated FAT and ext4 goldens |
| `crates/lab-storage/tests/e01_roundtrip.rs` | Generated E01 round-trip golden |
| `crates/lab-storage/tests/e01_hostile.rs` | E01 oversized/truncated fail-closed corpus |
| `crates/lab-transfer/tests/transfer_sign.rs` | Signed transfer and tamper rejection |
| `crates/lab-index/tests/search_operators.rs` | Search operator and coverage golden |
| `crates/lab-yara/tests/fixtures/production.yar` | Frozen YARA-X rule corpus |
| `crates/lab-hashset/tests/lookup.rs` | Hash-set lookup and database-pin vectors |

Restricted evidence belongs under the ignored `fixtures/restricted/` path and
must not be committed. Generated temporary corpora are recreated by their test
and are not release evidence by themselves.
