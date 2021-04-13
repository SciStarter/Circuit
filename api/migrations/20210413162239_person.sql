drop table if exists c_person;

create table c_person (
       id serial primary key,
       home_location geography(POINT, 4326) generated always as (case when interior ? 'home_location' then ST_GeomFromGeoJSON(interior -> 'home_location')::geography else null end) stored,
       last_location geography(POINT, 4326) generated always as (case when interior ? 'last_location' then ST_GeomFromGeoJSON(interior -> 'last_location')::geography else null end) stored,
       exterior jsonb not null,
       interior jsonb not null
);
