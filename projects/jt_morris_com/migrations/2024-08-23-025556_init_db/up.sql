-- Your SQL goes here
--
-- PostgreSQL database dump
--

-- Dumped from database version 16.4 (Ubuntu 16.4-1.pgdg24.04+1)
-- Dumped by pg_dump version 16.3 (Ubuntu 16.3-1.pgdg22.04+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', 'public', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;


-- the users table used to be called user
ALTER TABLE IF EXISTS "user" RENAME TO "users";
-- users table
CREATE TABLE if not exists public.users (
    id  uuid primary key DEFAULT gen_random_uuid() NOT NULL,
    full_name text NOT NULL,
    auth0_user_id text NOT NULL,
    email text,
    phone_number text,
    picture text,
    profile text
);



-- users.full_name
ALTER TABLE users ADD COLUMN IF NOT EXISTS full_name text NOT NULL;
ALTER TABLE users ALTER COLUMN "full_name" TYPE text;
update users set full_name = 'NULL' where full_name is null;
-- LINE
ALTER TABLE users ALTER COLUMN full_name SET NOT NULL;
-- users.auth0_user_id
-- some old colums used to be named this
-- this syntax is not supported by diesel cli but is supported by psql
-- ALTER TABLE "users" RENAME COLUMN IF EXISTS external_id TO auth0_user_id;
ALTER TABLE "users" ADD COLUMN IF NOT EXISTS auth0_user_id TEXT NOT NULL;
ALTER TABLE "users" ALTER COLUMN "auth0_user_id" TYPE TEXT;
UPDATE "users" SET auth0_user_id = 'NULL_ID' where "auth0_user_id" IS NULL;
ALTER TABLE "users" ALTER COLUMN "auth0_user_id" SET NOT NULL;
-- users.email
ALTER TABLE "users" ADD COLUMN IF NOT EXISTS "email" TEXT NULL;
ALTER TABLE "users" ALTER COLUMN "email" TYPE TEXT;
ALTER TABLE "users" ALTER COLUMN "email" DROP NOT NULL;
-- users.picture
ALTER TABLE "users" ADD COLUMN IF NOT exists picture TEXT NULL;
ALTER TABLE "users" ALTER COLUMN "picture" TYPE TEXT;
ALTER TABLE "users" ALTER COLUMN "picture" DROP NOT NULL;

ALTER TABLE "users" ADD COLUMN IF NOT exists "phone_number" TEXT NULL;
ALTER TABLE "users" ALTER COLUMN "phone_number" TYPE TEXT;
ALTER TABLE "users" ALTER COLUMN "phone_number" DROP NOT NULL;



ALTER TABLE "users" ADD COLUMN IF NOT exists profile TEXT NULL;
ALTER TABLE "users" ALTER COLUMN "profile" TYPE TEXT;
ALTER TABLE "users" ALTER COLUMN "profile" DROP NOT NULL;

ALTER TABLE "users" OWNER TO "jt-morris.com";












--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: jt-morris.com
--

CREATE OR REPLACE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO "jt-morris.com";

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: jt-morris.com
--

CREATE OR REPLACE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO "jt-morris.com";

--
-- Name: update_transaction_total_cost_on_insert_or_update(); Type: FUNCTION; Schema: public; Owner: jt-morris.com
--

CREATE OR REPLACE FUNCTION public.update_transaction_total_cost_on_insert_or_update() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
    BEGIN
    UPDATE transactions SET total_cost = (SELECT SUM(price_usd_cents) where transaction_id = NEW.transaction_id);
    RETURN NULL;
    END
$$;


ALTER FUNCTION public.update_transaction_total_cost_on_insert_or_update() OWNER TO "jt-morris.com";

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: jt-morris.com
--

CREATE TABLE IF NOT EXISTS public.__diesel_schema_migrations (
    version character varying(50) primary key  NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO "jt-morris.com";


-- Name: users; Type: TABLE; Schema: public; Owner: jt-morris.com
--





--
-- Name: auth_state; Type: TABLE; Schema: public; Owner: jt-morris.com
--

CREATE TABLE IF NOT EXISTS public.auth_state (
    id uuid PRIMARY KEY NOT NULL,
    started timestamp with time zone NOT NULL,
    scope text NOT NULL,
    redirect_url text NOT NULL,
    return_url text
);

-- auth_state.started
ALTER TABLE "auth_state" ADD COLUMN IF NOT EXISTS "started" timestamp with time zone NOT NULL;
ALTER TABLE "auth_state" ALTER COLUMN "started" TYPE timestamp with time zone;
UPDATE "auth_state" SET started = NOW() where "started" IS NULL;
ALTER TABLE "auth_state" ALTER COLUMN "started" SET NOT NULL;
-- auth_state.scope
ALTER TABLE "auth_state" ADD COLUMN IF NOT EXISTS scope TEXT NOT NULL;
ALTER TABLE "auth_state" ALTER COLUMN scope TYPE TEXT;
UPDATE "auth_state" SET scope = '' where scope IS NULL;
ALTER TABLE "auth_state" ALTER COLUMN scope SET NOT NULL;


ALTER TABLE public.auth_state OWNER TO "jt-morris.com";

--
-- Name: sessions; Type: TABLE; Schema: public; Owner: jt-morris.com
--

CREATE TABLE IF NOT EXISTS public.sessions (
    id bigint PRIMARY KEY NOT NULL,
    expires timestamp without time zone NOT NULL,
    data jsonb NOT NULL
);


ALTER TABLE public.sessions OWNER TO "jt-morris.com";

--
-- Name: sessions_id_seq; Type: SEQUENCE; Schema: public; Owner: jt-morris.com
--

CREATE SEQUENCE IF NOT EXISTS public.sessions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.sessions_id_seq OWNER TO "jt-morris.com";

--
-- Name: sessions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: jt-morris.com
--

ALTER SEQUENCE public.sessions_id_seq OWNED BY public.sessions.id;


--
-- Name: transactions; Type: TABLE; Schema: public; Owner: jt-morris.com
--

CREATE TABLE IF NOT EXISTS public.transactions (
    id uuid PRIMARY KEY NOT NULL,
    total_cost numeric NOT NULL,
    vendor text NOT NULL,
    date date NOT NULL
);


ALTER TABLE public.transactions OWNER TO "jt-morris.com";








--
-- Name: transactions_items; Type: TABLE; Schema: public; Owner: jt-morris.com
--

CREATE TABLE IF NOT EXISTS public.transactions_items (
    id uuid PRIMARY KEY NOT NULL,
    transaction_id uuid NOT NULL,
    price_usd_cents numeric NOT NULL,
    quantity numeric NOT NULL,
    item_name text NOT NULL,
    CONSTRAINT transactions_items_quantity_check CHECK ((quantity > (0)::numeric)),
    user_id uuid REFERENCES users(id) not null
);


ALTER TABLE public.transactions_items OWNER TO "jt-morris.com";

--
-- Name: transactions_items_categories; Type: TABLE; Schema: public; Owner: jt-morris.com
--

CREATE TABLE IF NOT EXISTS public.transactions_items_categories (
    id  uuid PRIMARY KEY NOT NULL,
    transaction_item_id uuid NOT NULL,
    category text
);


ALTER TABLE public.transactions_items_categories OWNER TO "jt-morris.com";




--
-- Name: vehicles; Type: TABLE; Schema: public; Owner: jt-morris.com
--

CREATE TABLE IF NOT EXISTS public.vehicles (
    vin  character varying(20) PRIMARY KEY UNIQUE NOT NULL,
    make text NOT NULL,
    model text NOT NULL,
    color text NOT NULL,
    year text NOT NULL
);


ALTER TABLE public.vehicles OWNER TO "jt-morris.com";

--
-- Name: sessions id; Type: DEFAULT; Schema: public; Owner: jt-morris.com
--

ALTER TABLE ONLY public.sessions ALTER COLUMN id SET DEFAULT nextval('public.sessions_id_seq'::regclass);

--
-- Name: transactions_items transaction_items_update_transactions_on_insert_or_update; Type: TRIGGER; Schema: public; Owner: jt-morris.com
--

CREATE OR REPLACE TRIGGER transaction_items_update_transactions_on_insert_or_update AFTER INSERT OR UPDATE ON public.transactions_items FOR EACH ROW EXECUTE FUNCTION public.update_transaction_total_cost_on_insert_or_update();

--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: pg_database_owner
--

GRANT ALL ON SCHEMA public TO "jt-morris.com";


--
-- PostgreSQL database dump complete
--

