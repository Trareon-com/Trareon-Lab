-- Append-only examination ledgers (Day 7).
CREATE TABLE IF NOT EXISTS audit_event (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  event_uuid TEXT NOT NULL UNIQUE,
  case_uuid TEXT NOT NULL,
  created_at_utc TEXT NOT NULL,
  actor_role TEXT NOT NULL,
  action TEXT NOT NULL,
  detail_json TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS provenance_event (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  event_uuid TEXT NOT NULL UNIQUE,
  case_uuid TEXT NOT NULL,
  created_at_utc TEXT NOT NULL,
  evidence_uuid TEXT NOT NULL,
  activity TEXT NOT NULL,
  detail_json TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS coverage_record (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  coverage_uuid TEXT NOT NULL UNIQUE,
  case_uuid TEXT NOT NULL,
  created_at_utc TEXT NOT NULL,
  scope TEXT NOT NULL,
  status TEXT NOT NULL,
  detail_json TEXT NOT NULL
);
