begin;

create extension postgis;
create extension postgis_raster;

create function set_updated()
returns trigger as $body$
begin
  NEW.updated = NOW();
  return NEW;
end;
$body$ language plpgsql;

create table c_opportunity (
       id serial primary key,
       created timestamptz not null default NOW(),
       updated timestamptz not null default NOW(),
       location_point geography(POINT, 4326) generated always as (case when exterior ? 'location_point' then ST_GeomFromGeoJSON(interior -> 'location_point')::geography else null end) stored,
       location_polygon geography(MULTIPOLYGON, 4326) generated always as (case when interior ? 'location_polygon' then ST_GeomFromGeoJSON(interior -> 'location_polygon')::geography else null end) stored,
       exterior jsonb not null,
       interior jsonb not null
);

create trigger c_set_opportunity_updated before update on c_opportunity for each row execute procedure set_updated();

create index c_opportunity_uid on c_opportunity using GIN ((exterior -> 'uid'));
create index c_opportunity_partner on c_opportunity using GIN ((exterior -> 'partner'));
create index c_opportunity_tags on c_opportunity using GIN ((exterior -> 'tags'));
create index c_opportunity_min_age on c_opportunity using GIN ((exterior -> 'min_age'));
create index c_opportunity_max_age on c_opportunity using GIN ((exterior -> 'max_age'));
create index c_opportunity_min_domain on c_opportunity using GIN ((exterior -> 'pes_domain'));
create index c_opportunity_cost on c_opportunity using GIN ((exterior -> 'cost'));
create index c_opportunity_is_online on c_opportunity using GIN ((exterior -> 'is_online'));
create index c_opportunity_start_dates on c_opportunity using GIN ((exterior -> 'end_dates'));
create index c_opportunity_end_dates on c_opportunity using GIN ((exterior -> 'start_dates'));
create index c_opportunity_location_point on c_opportunity using SPGIST (location_point);
create index c_opportunity_location_polygon on c_opportunity using SPGIST (location_polygon);
create index c_opportunity_fulltext_english on c_opportunity using GIN (to_tsvector('english', (exterior -> 'title')::text || ' ' || (exterior -> 'description')::text));
create index c_opportunity_withdrawn on c_opportunity using GIN ((interior -> 'withdrawn'));

create table c_partner (
       id serial primary key,
       created timestamptz not null default NOW(),
       updated timestamptz not null default NOW(),
       exterior jsonb not null,
       interior jsonb not null
);

create trigger c_set_partner_updated before update on c_partner for each row execute procedure set_updated();

create index c_partner_uid on c_partner using GIN ((exterior -> 'uid'));
create index c_partner_prime on c_partner using GIN ((interior -> 'prime'));
create index c_partner_authorized on c_partner using GIN ((interior -> 'authorized'));

create table c_person (
       id serial primary key,
       created timestamptz not null default NOW(),
       updated timestamptz not null default NOW(),
       home_location geography(POINT, 4326) generated always as (case when interior ? 'home_location' then ST_GeomFromGeoJSON(interior -> 'home_location')::geography else null end) stored,
       last_location geography(POINT, 4326) generated always as (case when interior ? 'last_location' then ST_GeomFromGeoJSON(interior -> 'last_location')::geography else null end) stored,
       exterior jsonb not null,
       interior jsonb not null
);

create trigger c_set_person_updated before update on c_person for each row execute procedure set_updated();

create index c_person_uid on c_person using GIN ((exterior -> 'uid'));
create index c_person_home_location on c_person using SPGIST (home_location);
create index c_person_last_location on c_person using SPGIST (last_location);

commit;
