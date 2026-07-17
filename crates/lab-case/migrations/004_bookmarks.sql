-- Bookmarks table (Day 33).
CREATE TABLE IF NOT EXISTS bookmark (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  bookmark_uuid TEXT NOT NULL UNIQUE,
  case_uuid TEXT NOT NULL,
  target_kind TEXT NOT NULL,
  target_ref TEXT NOT NULL,
  citation TEXT NOT NULL,
  author_role TEXT NOT NULL,
  created_at_utc TEXT NOT NULL,
  review_state TEXT NOT NULL,
  note TEXT
);
