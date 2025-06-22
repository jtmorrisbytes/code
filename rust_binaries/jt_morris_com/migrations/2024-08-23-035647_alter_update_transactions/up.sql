-- Your SQL goes here

alter table transactions add column IF NOT EXISTS "user_id" uuid not null REFERENCES users(id);
ALTER TABLE transactions alter column "user_id" TYPE uuid;