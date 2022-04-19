#!/bin/bash
shp2pgsql -D -I -s 94326 census/cb_2020_us_zcta520_500k.shp zip_code_tabulation_area >zcta.sql
sed -i "/^BEGIN;/a INSERT INTO spatial_ref_sys (srid, auth_name, auth_srid, proj4text, srtext) values ( 94326, 'epsg', 4326, '+proj=longlat +ellps=WGS84 +datum=WGS84 +no_defs ', 'GEOGCS[\"WGS 84\",DATUM[\"WGS_1984\",SPHEROID[\"WGS 84\",6378137,298.257223563,AUTHORITY[\"EPSG\",\"7030\"]],AUTHORITY[\"EPSG\",\"6326\"]],PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],UNIT[\"degree\",0.01745329251994328,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4326\"]]') ON CONFLICT DO NOTHING;" zcta.sql
sed -i '/^COMMIT;/i CREATE UNIQUE INDEX ON "zip_code_tabulation_area" ("zcta5ce20");' zcta.sql
