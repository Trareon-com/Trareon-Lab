# Validation Strategy

## Validation levels (exact labels only)

- `UNIT_VERIFIED` — implementation logic tests
- `CORPUS_VALIDATED` — expected results on identified corpus
- `CROSS_TOOL_CORROBORATED` — compared with named independent tools/methods
- `LAB_METHOD_APPROVED` — accepted under the deploying laboratory’s controlled method

UI and reports must never collapse these into a generic “verified” badge.

## Regression gates

No parser or analysis-pack release passes if it increases critical false positives, loses previously supported ground-truth artifacts, weakens determinism, or changes a report conclusion without a reviewed method-version change.
