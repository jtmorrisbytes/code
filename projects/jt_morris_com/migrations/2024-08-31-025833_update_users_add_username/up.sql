-- adds a username column to users
-- I REALLY, REALLY!! hope that we dont need usernames longer than 256 characters. 
alter table users add column IF NOT EXISTS username  varchar(256) check (LENGTH(username) > 0 AND LENGTH(username) <> 256);
alter table users alter column username TYPE text;
-- workaround until usernames can be chosen
update users set username=gen_random_uuid()::text where username is null;
alter table users alter column username SET NOT NULL;
alter table users ADD CONSTRAINT unique_username UNIQUE(username);