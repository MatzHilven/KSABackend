CREATE TYPE Role AS ENUM (
    'User',
    'Admin',
    'WebShop',
    'Event',
    'Activity',
    'Forms'
    );

ALTER TABLE users ADD COLUMN roles Role[] NOT NULL DEFAULT '{User}';