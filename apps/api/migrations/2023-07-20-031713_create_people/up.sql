CREATE TABLE IF NOT EXISTS people (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMPTZ,
    name TEXT,
    nip TEXT,
    country TEXT,
    city TEXT,
    address TEXT,
    zip_code TEXT,
    email TEXT,
    phone TEXT,
    birthday DATE,
    organization TEXT,
    role TEXT,
    department TEXT,
    joining_date DATE
);
