begin;

create or replace function c_valid_geojson(jsonb) returns boolean as
$func$
begin
    return ST_IsValid(ST_GeomFromGeoJSON($1));
exception when others then
    return false;
end
$func$ language plpgsql immutable;

create or replace function c_make_valid_geography(geometry) returns geography as
$func$
begin
    return ST_MakeValid($1)::geography;
exception when others then
    return null;
end
$func$ language plpgsql immutable;

alter table c_opportunity drop column if exists location_point;
alter table c_opportunity drop column if exists location_polygon;

alter table c_opportunity add column location_point geography(POINT, 4326)
generated always as (
  case
    when exterior #>> '{location_point,type}' = 'Point' and c_valid_geojson(exterior -> 'location_point')
    then c_make_valid_geography(ST_GeomFromGeoJSON(exterior -> 'location_point'))
    else null
  end
) stored;

alter table c_opportunity add column location_polygon geography(MULTIPOLYGON, 4326)
generated always as (
  case
    when exterior #>> '{location_polygon,type}' in ('Polygon', 'MultiPolygon') and c_valid_geojson(exterior -> 'location_polygon')
    then c_make_valid_geography(ST_GeomFromGeoJSON(exterior -> 'location_polygon'))
    else null
  end
) stored;

commit;
