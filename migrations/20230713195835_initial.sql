CREATE TYPE role AS ENUM ('User', 'Admin');

CREATE TABLE IF NOT EXISTS users (
  id bigserial,
  password_hash bytea,
  role role,
  name text
);

INSERT INTO users
(password_hash, role, name)
VALUES ('psw', 'Admin', 'Antonio');
