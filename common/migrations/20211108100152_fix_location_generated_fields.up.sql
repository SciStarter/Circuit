begin;

alter table c_opportunity drop column if exists location_point;
alter table c_opportunity drop column if exists location_polygon;

alter table c_opportunity add column location_point geography(POINT, 4326)
generated always as (
  case
    when exterior #>> '{location_point,type}' = 'Point'
    then ST_GeomFromGeoJSON(exterior -> 'location_point')::geography
    else null
  end
) stored;

alter table c_opportunity add column location_polygon geography(MULTIPOLYGON, 4326)
generated always as (
  case
    when exterior #>> '{location_polygon,type}' in ('Polygon', 'MultiPolygon')
    then ST_GeomFromGeoJSON(exterior -> 'location_polygon')::geography
    else null
  end
) stored;

commit;
