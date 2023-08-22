CREATE TABLE IF NOT EXISTS apps (
  id bigserial,
  slug text NOT NULL,
  name text NOT NULL,
  description text NOT NULL,
  images text[] NOT NULL,
  url text NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS apps_slug_idx ON apps (slug);
