CREATE TABLE IF NOT EXISTS databases (
  id SERIAL PRIMARY KEY,
  name TEXT UNIQUE NOT NULL,
  db_type TEXT NOT NULL,
  region TEXT NOT NULL,
  requests INT DEFAULT 0,
  owner_email TEXT NOT NULL REFERENCES users(email),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_databases_name ON databases(name);
CREATE INDEX idx_databases_owner_email ON databases(owner_email);
