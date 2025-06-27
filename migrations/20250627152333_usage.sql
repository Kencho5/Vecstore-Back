CREATE TABLE IF NOT EXISTS usage_logs (
   id SERIAL PRIMARY KEY,
   user_id INTEGER NOT NULL REFERENCES users(id),
   usage_date DATE NOT NULL,
   credits_used INTEGER DEFAULT 0,
   UNIQUE(user_id, usage_date)
);

CREATE INDEX IF NOT EXISTS idx_usage_logs_user_id ON usage_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_usage_logs_usage_date ON usage_logs(usage_date);
CREATE INDEX IF NOT EXISTS idx_usage_logs_user_date ON usage_logs(user_id, usage_date);
