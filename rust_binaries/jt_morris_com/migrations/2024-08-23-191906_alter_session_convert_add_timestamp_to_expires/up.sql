-- Your SQL goes here
ALTER TABLE "sessions" ALTER COLUMN "expires" SET DATA TYPE  TIMESTAMP  WITH  TIME ZONE USING ("expires" AT TIME ZONE 'UTC');