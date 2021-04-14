drop table if exists c_partner;

create table c_partner (
       id serial primary key,
       uid UUID unique not null generated always as ((exterior -> 'uid')::text::UUID) stored,
       exterior jsonb not null,
       interior jsonb not null
);

