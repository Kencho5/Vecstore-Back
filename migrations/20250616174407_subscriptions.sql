CREATE TABLE IF NOT EXISTS subscriptions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    user_email VARCHAR(255) NOT NULL REFERENCES users(email) ON DELETE CASCADE,
    customer_id VARCHAR(255) NOT NULL,
    subscription_id VARCHAR(255) UNIQUE NOT NULL,
    plan_name VARCHAR(50) NOT NULL,
    plan_type VARCHAR(50) NOT NULL,
    db_type VARCHAR(50) NOT NULL,
    price INT NOT NULL,
    req_limit INT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active', 
    next_billing_date DATE,
    usage_reset_date DATE DEFAULT CURRENT_DATE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_subscriptions_user_id
    ON subscriptions(user_id);

CREATE INDEX IF NOT EXISTS idx_subscriptions_user_email
    ON subscriptions(user_email);

CREATE INDEX IF NOT EXISTS idx_subscriptions_status
    ON subscriptions(status);

CREATE INDEX IF NOT EXISTS idx_subscriptions_plan
    ON subscriptions(plan_name);

CREATE INDEX IF NOT EXISTS idx_subscriptions_next_billing_date
    ON subscriptions(next_billing_date);

CREATE INDEX IF NOT EXISTS idx_subscriptions_usage_reset_date
    ON subscriptions(usage_reset_date);
