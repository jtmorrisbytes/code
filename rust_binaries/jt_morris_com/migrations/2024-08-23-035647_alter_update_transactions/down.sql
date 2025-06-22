-- This file should undo anything in `up.sql`
alter table transactions alter add column user_id uuid not null;

-- destructively reverts!
alter table transactions alter column user_id DROP NOT NULL;
update transactions set user_id = null;
alter table transactions drop column user id;