drop function if exists c_opportunity_by_uid_calendar_adds_during;
drop function if exists c_opportunity_by_uid_shares_during;
drop function if exists c_opportunity_by_uid_likes_during;
drop function if exists c_opportunity_by_uid_saves_during;
drop function if exists c_opportunity_by_uid_didits_during;
drop function if exists c_opportunity_by_uid_clicks_during;
drop function if exists c_opportunity_by_uid_domain;
drop function if exists c_opportunity_by_uid_is_status;
drop function if exists c_opportunity_by_uid_is_current;
drop function if exists c_opportunity_by_uid_is_current_as_of;
drop function if exists c_opportunity_is_current_as_of;

create or replace function c_opportunity_is_current(interior jsonb, exterior jsonb) returns boolean as
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
       EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'start_datetimes') WHERE value::timestamptz < CURRENT_TIMESTAMP)
       AND
       (
         EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'end_datetimes') WHERE value::timestamptz > CURRENT_TIMESTAMP)
         OR
         jsonb_array_length(exterior -> 'end_datetimes') = 0
       )
     )
     OR
     EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'start_datetimes') WHERE value::timestamptz > CURRENT_TIMESTAMP)
     OR
     EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'end_datetimes') WHERE value::timestamptz > CURRENT_TIMESTAMP)
     OR
     (
       jsonb_array_length(exterior -> 'start_datetimes') = 0
       AND
       jsonb_array_length(exterior -> 'end_datetimes') = 0
     )
   )
 );
END
$func$ language plpgsql immutable;

drop aggregate if exists public.first (anyelement);
drop function if exists public.first_agg;
drop aggregate if exists public.last (anyelement);
drop function if exists public.last_agg;
drop index if exists c_log_by_when_this_year;
drop function if exists c_refresh_log_by_when_this_year;
drop view if exists x_analytics_cache;
drop index if exists c_log_by_action_external;
drop index if exists c_analytics_cache_by_temporary;
drop index if exists c_analytics_cache_by_dates_and_partner;
drop index if exists c_analytics_cache_by_dates_and_opportunity;
drop table if exists c_misc_uuid;
drop table if exists c_misc_json;
drop table if exists c_misc_int;
drop table if exists c_misc_str;
drop table if exists c_demographics;
drop table if exists c_analytics_compiled;
drop table if exists c_analytics_search_term_cache;
drop table if exists c_analytics_overview_cache;
drop table if exists c_analytics_cache;
