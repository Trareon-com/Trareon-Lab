-- v5: second-method verification + blind PT participant hooks (FR-VAL-009/010)

CREATE TABLE IF NOT EXISTS second_method_verification (
  verification_uuid TEXT PRIMARY KEY NOT NULL,
  case_uuid TEXT NOT NULL,
  method_a_id TEXT NOT NULL,
  method_b_id TEXT NOT NULL,
  claim_ref TEXT NOT NULL,
  output_a_digest TEXT,
  output_b_digest TEXT,
  discrepancy TEXT NOT NULL DEFAULT 'none',
  disposition TEXT NOT NULL DEFAULT 'open',
  residual_risk TEXT,
  created_at_utc TEXT NOT NULL,
  CHECK (discrepancy IN ('none', 'present', 'inconclusive')),
  CHECK (disposition IN ('open', 'accepted_a', 'accepted_b', 'both_disclosed', 'rejected'))
);

CREATE TABLE IF NOT EXISTS blind_pt_participant (
  package_uuid TEXT PRIMARY KEY NOT NULL,
  case_uuid TEXT NOT NULL,
  scheme_id TEXT NOT NULL,
  round_id TEXT NOT NULL,
  import_digest TEXT NOT NULL,
  expected_results_embargoed INTEGER NOT NULL DEFAULT 1,
  submission_locked INTEGER NOT NULL DEFAULT 0,
  result_export_digest TEXT,
  created_at_utc TEXT NOT NULL,
  CHECK (expected_results_embargoed IN (0, 1)),
  CHECK (submission_locked IN (0, 1))
);
