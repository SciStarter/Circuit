begin;

create extension postgis;
create extension postgis_raster;

create table c_opportunity (
       id serial primary key,
       location_point geography(POINT, 4326) generated always as (case when exterior ? 'location_point' then ST_GeomFromGeoJSON(interior -> 'location_point')::geography else null end) stored,
       location_polygon geography(MULTIPOLYGON, 4326) generated always as (case when interior ? 'location_polygon' then ST_GeomFromGeoJSON(interior -> 'location_polygon')::geography else null end) stored,
       exterior jsonb not null,
       interior jsonb not null
);

create index c_opportunity_uid on c_opportunity using GIN ((exterior -> 'uid') jsonb_path_ops);
create index c_opportunity_partner on c_opportunity using GIN ((exterior -> 'partner') jsonb_path_ops);
create index c_opportunity_tags on c_opportunity using GIN ((exterior -> 'tags') jsonb_path_ops);
create index c_opportunity_min_age on c_opportunity using GIN ((exterior -> 'min_age'));
create index c_opportunity_max_age on c_opportunity using GIN ((exterior -> 'max_age'));
create index c_opportunity_min_domain on c_opportunity using GIN ((exterior -> 'pes_domain') jsonb_path_ops);
create index c_opportunity_cost on c_opportunity using GIN ((exterior -> 'cost') jsonb_path_ops);
create index c_opportunity_is_online on c_opportunity using GIN ((exterior -> 'is_online') jsonb_path_ops);
create index c_opportunity_start_dates on c_opportunity using GIN ((exterior -> 'end_dates'));
create index c_opportunity_end_dates on c_opportunity using GIN ((exterior -> 'start_dates'));
create index c_opportunity_location_point on c_opportunity using SPGIST (location_point);
create index c_opportunity_location_polygon on c_opportunity using SPGIST (location_polygon);
create index c_opportunity_fulltext_english on c_opportunity using GIN (to_tsvector('english', (exterior -> 'title')::text || ' ' || (exterior -> 'description')::text));
create index c_opportunity_withdrawn on c_opportunity using GIN ((interior -> 'withdrawn') jsonb_path_ops);

create table c_partner (
       id serial primary key,
       exterior jsonb not null,
       interior jsonb not null
);

create index c_partner_uid on c_partner using GIN ((exterior -> 'uid') jsonb_path_ops);
create index c_partner_prime on c_partner using GIN ((interior -> 'prime') jsonb_path_ops);
create index c_partner_authorized on c_partner using GIN ((interior -> 'authorized') jsonb_path_ops);

create table c_person (
       id serial primary key,
       home_location geography(POINT, 4326) generated always as (case when interior ? 'home_location' then ST_GeomFromGeoJSON(interior -> 'home_location')::geography else null end) stored,
       last_location geography(POINT, 4326) generated always as (case when interior ? 'last_location' then ST_GeomFromGeoJSON(interior -> 'last_location')::geography else null end) stored,
       exterior jsonb not null,
       interior jsonb not null
);

create index c_person_uid on c_person using GIN ((exterior -> 'uid') jsonb_path_ops);
create index c_person_home_location on c_person using SPGIST (home_location);
create index c_person_last_location on c_person using SPGIST (last_location);

commit;
