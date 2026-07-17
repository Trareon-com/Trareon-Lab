-- Forensic index v1 (searchable rows; not case admin metadata).
CREATE TABLE IF NOT EXISTS index_entry (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  case_uuid TEXT NOT NULL,
  entry_kind TEXT NOT NULL,
  target_ref TEXT NOT NULL,
  display_text TEXT NOT NULL,
  sort_key TEXT NOT NULL,
  created_at_utc TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_index_entry_case ON index_entry(case_uuid);
CREATE INDEX IF NOT EXISTS idx_index_entry_sort ON index_entry(case_uuid, sort_key);
