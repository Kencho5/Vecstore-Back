CREATE TABLE IF NOT EXISTS email_codes (
  email VARCHAR(255) NOT NULL,
  code SMALLINT NOT NULL,
  expiry TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_email_code_email ON email_codes (email, code);
