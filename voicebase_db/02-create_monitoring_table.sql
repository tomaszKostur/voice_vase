CREATE TYPE action_enum AS ENUM ('login', 'logout', 'posted');

CREATE TABLE monitoring_table (
    id          BIGSERIAL       PRIMARY KEY,
    email       VARCHAR(255)    NOT NULL,
    action_date TIMESTAMPTZ     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    action_type action_enum      NOT NULL,
    
    -- Optional useful columns
    ip_address  INET,
    user_agent  TEXT,
    created_at  TIMESTAMPTZ     DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for common queries
CREATE INDEX idx_monitoring_email_action 
    ON monitoring_table (email, action_type, action_date DESC);

CREATE INDEX idx_monitoring_date 
    ON monitoring_table (action_date DESC);