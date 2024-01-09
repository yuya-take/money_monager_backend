-- This file should undo anything in `up.sql`
DROP FUNCTION IF EXISTS set_updated_at_column() cascade;
DROP TABLE IF EXISTS users cascade;
DROP TABLE IF EXISTS user_activation cascade;
