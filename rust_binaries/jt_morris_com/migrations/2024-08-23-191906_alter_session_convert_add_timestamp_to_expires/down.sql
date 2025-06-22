-- This file should undo anything in `up.sql`
ALTER TABLE "sessions" ALTER COLUMN "expires" SET DATA TYPE  TIMESTAMP WITHOUT TIME ZONE;