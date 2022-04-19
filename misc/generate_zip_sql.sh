#!/bin/bash
shp2pgsql -D -I -s 94326 census/cb_2020_us_zcta520_500k.shp zip_code_tabulation_area >zcta.sql
sed -i '/^COMMIT;/i CREATE UNIQUE INDEX ON "zip_code_tabulation_area" ("zcta5ce20");' zcta.sql
