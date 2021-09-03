begin;

create table c_opportunity_like (
       "opportunity_id" integer not null references c_opportunity,
       "person" uuid,
       "when" timestamptz not null default now(),

       /* identified persons can only like an opportunity once, but
          anonymous likes (person is null) do not trigger this
          constraint */

       unique ("opportunity_id", "person")
);

create index c_opportunity_like_by_opportunity on c_opportunity_like (opportunity_id);
create index c_opportunity_like_by_person on c_opportunity_like (person);

create table c_opportunity_review (
       "opportunity_id" integer not null references c_opportunity,
       "person" uuid not null,
       "rating" smallint not null check ("rating" > 0 and "rating" < 6),
       "comment" text not null default '',
       "when" timestamptz not null default now(),
       "flags" smallint not null default 0,
       -- person is not null in this table, so the uniqueness is complete
       unique ("opportunity_id", "person")
);

create index c_opportunity_review_by_opportunity on c_opportunity_review (opportunity_id);
create index c_opportunity_review_by_person on c_opportunity_review (person);

commit;
