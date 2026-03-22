CREATE TABLE login_table (
    id              BIGSERIAL       PRIMARY KEY,
    email           VARCHAR(255)    UNIQUE NOT NULL,
    password_hash   VARCHAR(255)    NOT NULL,
    created_at      TIMESTAMPTZ     DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMPTZ     DEFAULT CURRENT_TIMESTAMP,
    is_active       BOOLEAN         DEFAULT TRUE
);

-- Optional: better naming clarity
COMMENT ON TABLE login_table IS 'Stores user authentication credentials';