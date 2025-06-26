CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    user_email VARCHAR(255) NOT NULL REFERENCES users(email) ON DELETE CASCADE,
    transaction_id VARCHAR(255) UNIQUE NOT NULL,
    customer_id VARCHAR(255) NOT NULL,
    price_id VARCHAR(255) NOT NULL,
    plan_name VARCHAR(100) NOT NULL,
    plan_description VARCHAR(100) NOT NULL,
    credits_purchased INTEGER NOT NULL,
    amount_paid INTEGER NOT NULL, -- amount in cents
    status VARCHAR(50) NOT NULL,
    billed_at TIMESTAMP,
    invoice_id VARCHAR(255),
    invoice_number VARCHAR(255),
    payment_method VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_transactions_user_id
    ON transactions(user_id);

CREATE INDEX IF NOT EXISTS idx_transactions_user_email
    ON transactions(user_email);

CREATE INDEX IF NOT EXISTS idx_transactions_status
    ON transactions(status);

CREATE INDEX IF NOT EXISTS idx_transactions_customer_id
    ON transactions(customer_id);

CREATE INDEX IF NOT EXISTS idx_transactions_billed_at
    ON transactions(billed_at);

