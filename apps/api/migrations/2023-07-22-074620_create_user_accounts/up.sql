CREATE TABLE IF NOT EXISTS user_accounts (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMPTZ,
    username VARCHAR NOT NULL,
    password TEXT NOT NULL,
    role VARCHAR NOT NULL,
    person_id uuid,

    FOREIGN KEY (person_id) REFERENCES people(id)
);