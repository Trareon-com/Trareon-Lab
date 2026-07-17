-- Evidence object registry (counts shown on case open).
CREATE TABLE IF NOT EXISTS evidence_object (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  evidence_uuid TEXT NOT NULL UNIQUE,
  case_uuid TEXT NOT NULL,
  created_at_utc TEXT NOT NULL,
  display_name TEXT NOT NULL,
  evidence_class TEXT NOT NULL,
  validation_state TEXT NOT NULL
);
