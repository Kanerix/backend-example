CREATE TABLE posts (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    user_id uuid NOT NULL,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE comments (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    user_id uuid NOT NULL,
    post_id uuid NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_post_id FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);
