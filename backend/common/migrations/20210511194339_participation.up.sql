begin;

create table c_participation (
       id serial primary key,
       created timestamptz not null default NOW(),
       updated timestamptz not null default NOW(),
       exterior jsonb not null,
       interior jsonb not null
);

create index c_participation_by_opportunity on c_participation using GIN ((exterior -> 'opportunity'));
create index c_participation_by_participant on c_participation using GIN ((interior -> 'participant'));
create index c_participation_by_when on c_participation using GIN ((exterior -> 'when'));

commit;
