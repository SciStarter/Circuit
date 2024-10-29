create table c_visits (
       "when" timestamptz not null default now(),
       "user" uuid,
       "times" integer not null default 0
);

insert into c_visits ("when", "user", "times") values (now(), null, 604277); -- Prior to table creation, according to Google Analytics count of page events

create table c_visits_cumulative (
       "as_of" timestamptz not null,
       "total" integer not null
);

insert into c_visits_cumulative ("as_of", "total") values (now(), 604277);
