CREATE TABLE IF NOT EXISTS bookmarks (
  id bigserial,
  slug text NOT NULL,
  url text NOT NULL,
  title text NOT NULL,
  description text NOT NULL,
  favicon text,
  image text,
  posted_at timestamp with time zone NOT NULL,
  created_at timestamp with time zone NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS bookmarks_slug_idx ON bookmarks (slug);
