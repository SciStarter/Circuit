drop index if exists c_person_by_state_and_metro;
drop index if exists c_person_by_state;

alter table c_person drop column if exists "metro";
alter table c_person drop column if exists "state";

alter table c_person drop column "last_location";
alter table c_person drop column "home_location";

alter table c_person add column "home_location" geography(Point, 4326) generated always as (
  CASE
    WHEN jsonb_typeof(interior -> 'home_location'::text) <> 'null'::text THEN st_geomfromgeojson(interior -> 'home_location'::text)::geography
    ELSE NULL::geography                                                                                                     
  END) stored;

alter table c_person add column "last_location" geography(Point, 4326) generated always as (
  CASE                                                                                                                         
    WHEN jsonb_typeof(interior -> 'last_location'::text) <> 'null'::text THEN st_geomfromgeojson(interior -> 'last_location'::text)::geography
    ELSE NULL::geography                                                                                                     
  END) stored;
