CREATE TABLE IF NOT EXISTS databases (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  db_type TEXT NOT NULL,
  region TEXT NOT NULL,
  requests INT DEFAULT 0,
  owner_id INT NOT NULL REFERENCES users(id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (owner_id, name)
);
CREATE INDEX IF NOT EXISTS idx_databases_name ON databases(name);
CREATE INDEX IF NOT EXISTS idx_databases_owner_id ON databases(owner_id);
