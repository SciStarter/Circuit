create table c_analytics_cache (
       "temporary" boolean not null,
       "begin" timestamptz not null,
       "end" timestamptz not null,
       "about" uuid not null,

       "date" timestamptz not null,
       "city" text not null,
       "device_category" text not null,
       "first_session_date" timestamptz not null,
       "page_path" text not null,
       "region" text not null,
       "views" bigint not null,
       "events" bigint not null,
       "total_users" bigint not null,
       "new_users" bigint not null,
       "engagement_duration" double precision not null
);

create index c_analytics_cache_by_dates on c_analytics_cache ("begin", "end");

create index c_analytics_cache_by_temporary on c_analytics_cache ("temporary") where "temporary" = true;

create index c_analytics_cache_by_about on c_analytics_cache ("about");

create unique index c_analytics_cache_no_duplicates on c_analytics_cache ("begin", "end", "about");

create index c_log_by_action_external on c_log ("action") where "action" = 'external';

create function c_refresh_log_by_when_this_year () returns void as $$
begin
       execute 'drop index if exists c_log_by_when_this_year';
       execute format('create index c_log_by_when_this_year on c_log ("when") where "when" > %L', NOW() - interval '1 year');
end;
$$ language 'plpgsql' volatile;

select c_refresh_log_by_when_this_year();
