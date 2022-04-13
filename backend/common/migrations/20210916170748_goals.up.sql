begin;

create type c_person_goals_status as enum ('canceled', 'failed', 'working', 'succeeded');

create table if not exists c_person_goals (
    "id" serial primary key,
    "person_id" integer not null references c_person on delete cascade,
    "category" varchar(32) not null,
    "target" integer not null,
    "begin" timestamptz not null,
    "end" timestamptz not null,
    "status" c_person_goals_status not null default 'working'
);

create index c_person_goals_by_person_id on c_person_goals ("person_id");
create index c_person_goals_by_category on c_person_goals ("category");
create index c_person_goals_by_status on c_person_goals ("status");

create index c_person_goals_working on c_person_goals ("person_id") where "status" = 'working';

commit;
