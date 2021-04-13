drop table if exists c_opportunity;

create table c_opportunity (
       id serial primary key,
       partner_uid UUID unique not null generated always as ((exterior -> 'partner_uid')::text::UUID) stored,
       exterior jsonb not null,
       interior jsonb not null
);

drop index if exists c_opportunity_fulltext_english;

create index c_opportunity_fulltext_english on c_opportunity using GIN (to_tsvector('english', exterior -> 'description'));
