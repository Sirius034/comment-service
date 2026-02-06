-- Your SQL goes here
CREATE TABLE comments (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  data_created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  type INTEGER NOT NULL DEFAULT 2,
  comment TEXT,
  page_id TEXT,
  user_name TEXT,
  user_id TEXT,
  client_id TEXT,
  pinned BOOLEAN NOT NULL DEFAULT FALSE
)
