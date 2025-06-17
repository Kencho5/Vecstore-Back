CREATE TABLE IF NOT EXISTS subscriptions (
    id SERIAL PRIMARY KEY,
    user_email VARCHAR(255) NOT NULL REFERENCES users(email) ON DELETE CASCADE,
    subscription_id VARCHAR(255) UNIQUE NOT NULL,
    plan_name VARCHAR(255) NOT NULL,
    price VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active', 
    next_billing_date DATE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_subscriptions_user_email
    ON subscriptions(user_email);

CREATE INDEX IF NOT EXISTS idx_subscriptions_status
    ON subscriptions(status);

CREATE INDEX IF NOT EXISTS idx_subscriptions_next_billing_date
    ON subscriptions(next_billing_date);
