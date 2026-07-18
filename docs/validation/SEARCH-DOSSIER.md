# Search validation dossier

## Supported query operators

`lab-index` parses `SearchQuery` into OR clauses containing AND terms. AND binds
more tightly than OR; adjacent operands are also ANDed.

- Phrase: `"foo bar"` performs a contiguous UTF-8 text match.
- Boolean: `foo AND bar OR baz`.
- Wildcard: `foo*` matches a field prefix and `*foo` a field suffix. `*foo*`
  performs a contains match.
- Bytes: `hex:` requires non-empty, even-length hexadecimal digits, for example
  `hex:deadbeef`. Index schema v1 has no content column, so this matches ASCII
  hex in `target_ref` or `display_text`.

Paths and names are SQLite `TEXT` and are searched as UTF-8. SQLite's built-in
case folding applies to ASCII; non-ASCII text is preserved and matched without
lossy decoding. `%`, `_`, and `\` supplied by users are escaped before `LIKE`.

## Coverage and limits

Search fetches one row beyond the requested limit. `SearchResult.truncated` is
set when more rows exist, and `truncation_reason` states that coverage is
partial and tells the caller to raise the limit or narrow the query. A limited
page therefore cannot be presented as complete coverage.

## Validation

`crates/lab-index/tests/search_operators.rs` covers phrases, AND/OR precedence,
prefix/suffix wildcards, hex validation, truncation disclosure, and JSON
round-tripping of `SearchPlan`.

`crates/lab-index/tests/persist_10k.rs` asserts a 200-row page from 10,000 rows
stays below a generous 5-second CI budget. The existing
`crates/lab-index/tests/windowed_1m.rs` contract remains unchanged: the
one-million-row page budget is 5 seconds and its legacy substring search still
passes.
