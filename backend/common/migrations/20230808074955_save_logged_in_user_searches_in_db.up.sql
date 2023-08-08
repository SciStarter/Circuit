create table c_person_searches (
       "id" integer primary key,
       "person_id" integer not null references c_person on delete cascade,
       "when" timestamptz not null default now(),
       "text" text not null
);

create index c_person_searches_by_person on c_person_searches(person_id);
