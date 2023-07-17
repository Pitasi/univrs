CREATE TYPE role AS ENUM ('User', 'Admin');

CREATE TABLE IF NOT EXISTS users (
  id bigserial,
  role role,
  email TEXT NOT NULL,
  username TEXT,
  picture TEXT
);

