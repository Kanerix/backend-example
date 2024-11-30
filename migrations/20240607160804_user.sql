CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE role AS ENUM ('user', 'moderator', 'admin');

CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    username VARCHAR(32) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    role role NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

CREATE TABLE passwords (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    user_id uuid NOT NULL,
    hash VARCHAR(255) NOT NULL,
    salt VARCHAR(255) DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
