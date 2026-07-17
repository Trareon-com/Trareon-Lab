-- Case metadata v1 (admin only; forensic index remains lab-index adapter).
CREATE TABLE IF NOT EXISTS case_record (
  case_uuid TEXT PRIMARY KEY NOT NULL,
  state TEXT NOT NULL,
  created_at_utc TEXT NOT NULL,
  updated_at_utc TEXT NOT NULL,
  display_name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS migration_audit (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  from_version INTEGER NOT NULL,
  to_version INTEGER NOT NULL,
  applied_at_utc TEXT NOT NULL
);
