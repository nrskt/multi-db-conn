-- Your SQL goes here
CREATE TABLE IF NOT EXISTS sample_user (
  id INTEGER PRIMARY KEY,
  tenant_id INTEGER NOT NULL,
  value TEXT NOT NULL
);
