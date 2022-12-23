delete from c_transit
where exists(
  select 1 from c_opportunity
  where
    ("exterior"->>'uid')::uuid in (c_transit."prior", c_transit."postor") and
    "exterior"->>'entity_type' != 'opportunity'
);

create table c_analytics_cache (
       "temporary" boolean not null,
       "begin" timestamptz not null,
       "end" timestamptz not null,
       "opportunity" uuid not null,
       "partner" uuid not null,
       "date" timestamptz not null,
       "city" text not null,
       "device_category" text not null,
       "first_session_date" timestamptz not null,
       "session_channel_group" varchar(16) not null,
       "page_path" text not null,
       "page_referrer" text not null,
       "region" text not null,
       "views" bigint not null,
       "sessions" bigint not null,
       "events" bigint not null,
       "total_users" bigint not null,
       "new_users" bigint not null,
       "engagement_duration" double precision not null
);

create table c_analytics_overview_cache (
       "temporary" boolean not null,
       "begin" timestamptz not null,
       "end" timestamptz not null,
       "unique_visitors" bigint not null,
       "accounts" bigint not null,
       "opportunity_views" bigint not null,
       "opportunity_unique" bigint not null,
       "opportunity_exits" bigint not null,
       "didits" bigint not null,
       "saves" bigint not null,
       "likes" bigint not null,
       "shares" bigint not null,
       "calendar_adds" bigint not null
);

create table c_analytics_search_term_cache (
       "temporary" boolean not null,
       "begin" timestamptz not null,
       "end" timestamptz not null,
       "term" text not null,
       "times" bigint not null
);

create table c_analytics_compiled (
       "about" uuid not null,
       "kind" integer not null,
       "period" integer not null,
       "status" integer not null,
       "data" jsonb not null,
       PRIMARY KEY ("about", "kind", "period", "status")
);

create table c_demographics (
       "about" uuid primary key,
       "data" jsonb not null
);

create table c_misc_str (
       "key" varchar(32) primary key,
       "value" text not null
);

create table c_misc_int (
       "key" varchar(32) primary key,
       "value" bigint not null
);

create table c_misc_json (
       "key" varchar(32) primary key,
       "value" jsonb not null
);

create table c_misc_uuid (
       "key" varchar(32) primary key,
       "value" uuid not null
);

create index c_analytics_cache_by_dates_and_opportunity on c_analytics_cache ("begin", "end", "opportunity");

create index c_analytics_cache_by_dates_and_partner on c_analytics_cache ("begin", "end", "partner");

create index c_analytics_cache_by_temporary on c_analytics_cache ("temporary") where "temporary" = true;

create index c_log_by_action_external on c_log ("action") where "action" = 'external';

create or replace function c_refresh_log_by_when_this_year () returns void as $$
begin
       execute 'drop index if exists c_log_by_when_this_year';
       execute format('create index c_log_by_when_this_year on c_log ("when") where "when" > %L', NOW() - interval '1 year');
end;
$$ language 'plpgsql' volatile;

select c_refresh_log_by_when_this_year();

create or replace function c_opportunity_is_current_as_of(interior jsonb, exterior jsonb, stamp timestamptz) returns boolean as
$func$
BEGIN
 RETURN (
   coalesce(nullif(interior ->> 'review_status', ''), 'not_required') IN ('publish', 'not_required')
   AND
   interior ->> 'accepted' = 'true'
   AND
   interior ->> 'withdrawn' = 'false'
   AND
   (
     (
       EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'start_datetimes') WHERE value::timestamptz < stamp)
       AND
       (
         EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'end_datetimes') WHERE value::timestamptz > stamp)
         OR
         jsonb_array_length(exterior -> 'end_datetimes') = 0
       )
     )
     OR
     EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'start_datetimes') WHERE value::timestamptz > stamp)
     OR
     EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'end_datetimes') WHERE value::timestamptz > stamp)
     OR
     (
       jsonb_array_length(exterior -> 'start_datetimes') = 0
       AND
       jsonb_array_length(exterior -> 'end_datetimes') = 0
     )
   )
 );
END
$func$ language plpgsql stable;

create or replace function c_opportunity_is_current(interior jsonb, exterior jsonb) returns boolean as
$func$
BEGIN
 RETURN c_opportunity_is_current_as_of(interior, exterior, CURRENT_TIMESTAMP);
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_is_current_as_of(uid uuid, stamp timestamptz) returns boolean as
$func$
DECLARE
 opp c_opportunity%ROWTYPE;
BEGIN
 SELECT * INTO opp FROM c_opportunity WHERE ("exterior"->>'uid')::uuid = uid LIMIT 1;
 RETURN c_opportunity_is_current_as_of(opp.interior, opp.exterior, stamp);
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_is_current(uid uuid) returns boolean as
$func$
DECLARE
 opp c_opportunity%ROWTYPE;
BEGIN
 SELECT * INTO opp FROM c_opportunity WHERE ("exterior"->>'uid')::uuid = uid LIMIT 1;
 RETURN c_opportunity_is_current(opp.interior, opp.exterior);
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_is_status(uid uuid, status integer) returns boolean as
$func$
DECLARE
 opp c_opportunity%ROWTYPE;
BEGIN
 SELECT * INTO opp FROM c_opportunity WHERE ("exterior"->>'uid')::uuid = uid LIMIT 1;
 CASE status
  WHEN 2 THEN RETURN c_opportunity_is_current(opp.interior, opp.exterior) = false;
  WHEN 1 THEN RETURN c_opportunity_is_current(opp.interior, opp.exterior) = true;
  ELSE RETURN true;
 END CASE;
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_domain(uid uuid) returns text as
$func$
DECLARE
 opp c_opportunity%ROWTYPE;
 partner c_partner%ROWTYPE;
BEGIN
 SELECT * INTO opp FROM c_opportunity WHERE ("exterior"->>'uid')::uuid = uid LIMIT 1;
 CASE opp."exterior"->>'pes_domain'
  WHEN 'unspecified' THEN
   SELECT * INTO partner FROM c_partner WHERE ("exterior"->'uid') = (opp."exterior"->'partner') LIMIT 1;
   RETURN partner."exterior"->>'pes_domain';
  ELSE RETURN opp."exterior"->>'pes_domain';
 END CASE;
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_clicks_during("uid" uuid, "begin" timestamptz, "end" timestamptz) returns bigint as
$func$
DECLARE
 val bigint;
BEGIN
 SELECT COALESCE(COUNT(*), 0) INTO val FROM c_log WHERE "action" = 'external' AND "object" = "uid" AND "when" >= "begin" AND "when" < "end";
 RETURN val;
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_didits_during("uid" uuid, "begin" timestamptz, "end" timestamptz) returns bigint as
$func$
DECLARE
 val bigint;
BEGIN
 SELECT COALESCE(COUNT(*), 0) INTO val FROM c_involvement WHERE ("exterior"->>'opportunity')::uuid = "uid" AND ("exterior"->'mode')::integer >= 30 AND "updated" >= "begin" AND "updated" < "end";
 RETURN val;
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_saves_during("uid" uuid, "begin" timestamptz, "end" timestamptz) returns bigint as
$func$
DECLARE
 val bigint;
BEGIN
 SELECT COALESCE(COUNT(*), 0) INTO val FROM c_involvement WHERE ("exterior"->>'opportunity')::uuid = "uid" AND ("exterior"->'mode')::integer = 20 AND "updated" >= "begin" AND "updated" < "end";
 RETURN val;
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_likes_during("uid" uuid, "begin" timestamptz, "end" timestamptz) returns bigint as
$func$
DECLARE
 opp c_opportunity%ROWTYPE;
 val bigint;
BEGIN
 SELECT * INTO opp FROM c_opportunity WHERE ("exterior"->>'uid')::uuid = "uid" LIMIT 1;
 SELECT COALESCE(COUNT(*), 0) INTO val FROM c_opportunity_like WHERE "opportunity_id" = opp."id" AND "when" >= "begin" AND "when" < "end";
 RETURN val;
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_shares_during("uid" uuid, "begin" timestamptz, "end" timestamptz) returns bigint as
$func$
DECLARE
 val bigint;
BEGIN
 SELECT COALESCE(COUNT(*), 0) INTO val FROM c_log WHERE c_log."object" = "uid" AND "action" LIKE 'shared:%' AND "when" >= "begin" AND "when" < "end";
 RETURN val;
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_calendar_adds_during("uid" uuid, "begin" timestamptz, "end" timestamptz) returns bigint as
$func$
DECLARE
 val bigint;
BEGIN
 SELECT COALESCE(COUNT(*), 0) INTO val FROM c_log WHERE c_log."object" = "uid" AND "action" LIKE 'calendar:%' AND "when" >= "begin" AND "when" < "end";
 RETURN val;
END
$func$ language plpgsql stable;



-- first and last aggregates from https://wiki.postgresql.org/wiki/First/last_%28aggregate%29

-- Create a function that always returns the first non-NULL value:
CREATE OR REPLACE FUNCTION public.first_agg (anyelement, anyelement)
  RETURNS anyelement
  LANGUAGE sql IMMUTABLE STRICT PARALLEL SAFE AS
'SELECT $1';

-- Then wrap an aggregate around it:
CREATE AGGREGATE public.first (anyelement) (
  SFUNC    = public.first_agg
, STYPE    = anyelement
, PARALLEL = safe
);

-- Create a function that always returns the last non-NULL value:
CREATE OR REPLACE FUNCTION public.last_agg (anyelement, anyelement)
  RETURNS anyelement
  LANGUAGE sql IMMUTABLE STRICT PARALLEL SAFE AS
'SELECT $2';

-- Then wrap an aggregate around it:
CREATE AGGREGATE public.last (anyelement) (
  SFUNC    = public.last_agg
, STYPE    = anyelement
, PARALLEL = safe
);
