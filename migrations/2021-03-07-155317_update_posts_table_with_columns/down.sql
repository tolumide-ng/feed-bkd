-- This file should undo anything in `up.sql`

ALTER TABLE posts DROP COLUMN description;
ALTER TABLE posts DROP COLUMN created_at;
ALTER TABLE posts DROP COLUMN updated_at;
ALTER TABLE posts DROP COLUMN slug;