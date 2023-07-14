CREATE TABLE IF NOT EXISTS likes (
  id bigserial,
  user_id bigint,
  url text
);

CREATE UNIQUE INDEX IF NOT EXISTS likes_user_id_url_idx ON likes (user_id, url);

