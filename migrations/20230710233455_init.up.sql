-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    "users" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        first_name VARCHAR(100) NOT NULL,
        last_name VARCHAR(100) NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        resume VARCHAR NOT NULL DEFAULT '',
        is_verified BOOLEAN NOT NULL DEFAULT TRUE,
        password VARCHAR(100) NOT NULL,
        role VARCHAR(50) NOT NULL DEFAULT 'user',
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );

CREATE TABLE
    "jobs" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        title VARCHAR(100) NOT NULL,
        company_name VARCHAR(100) NOT NULL,
        city VARCHAR(255) NOT NULL,
        country VARCHAR(100) NOT NULL,
        salary VARCHAR(255) NOT NULL,
        description VARCHAR(255) NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );

    CREATE TABLE
    "applications" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        job_id UUID NOT NULL,
        user_id UUID NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );


CREATE INDEX users_email_idx ON users (email);