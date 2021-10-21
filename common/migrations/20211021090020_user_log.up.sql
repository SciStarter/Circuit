begin;

create table c_person_log (
       "id" serial primary key,
       "person_id" integer not null references c_person on delete cascade,
       "when" timestamptz not null default CURRENT_TIMESTAMP,
       "event" jsonb not null
);

create index c_person_log_by_person on c_person_log ("person_id");

commit;
