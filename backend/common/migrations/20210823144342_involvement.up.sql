begin;

create table c_involvement (
       id serial primary key,
       created timestamptz not null default NOW(),
       updated timestamptz not null default NOW(),
       exterior jsonb not null,
       interior jsonb not null
);

create index c_involvement_by_opportunity on c_involvement using GIN ((exterior -> 'opportunity'));
create index c_involvement_by_participant on c_involvement using GIN ((interior -> 'participant'));
create unique index c_involvement_unique_opporunity_and_participant on c_involvement ((exterior -> 'opportunity'), (interior -> 'participant'));

commit;

