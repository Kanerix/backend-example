CREATE TABLE 
    refresh_tokens(
        id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
        user_id uuid NOT NULL,
        token CHAR(32) NOT NULL,
        expires_at TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '30 days',
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
    );
