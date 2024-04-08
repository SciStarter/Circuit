drop trigger if exists c_opportunity_search_on_opportunity_insert_or_update on c_opportunity;
drop trigger if exists c_opportunity_search_on_opportunity_delete on c_opportunity;

drop function if exists c_opportunity_search_delete;
drop function if exists c_opportunity_search_update;

drop index if exists c_opportunity_search_via_location_polygon;
drop index if exists c_opportunity_search_via_location_point;
drop index if exists c_opportunity_search_via_fulltext_english;

drop table if exists c_opportunity_search;

drop type if exists t_location_type;
drop type if exists t_venue_type;
drop type if exists t_cost;
drop type if exists t_entity_type;
