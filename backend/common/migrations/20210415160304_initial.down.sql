drop index if exists c_person_last_location;
drop index if exists c_person_home_location;
drop index if exists c_person_email_hashes;
drop index if exists c_person_email;
drop index if exists c_person_uid;

drop trigger if exists c_set_person_updated on c_person cascade;

drop table if exists c_person;

drop index if exists c_partner_authorized;
drop index if exists c_partner_prime;
drop index if exists c_partner_uid;

drop trigger if exists c_set_partner_updated on c_partner cascade;

drop table if exists c_partner;

drop index if exists c_opportunity_withdrawn;
drop index if exists c_opportunity_fulltext_english;
drop index if exists c_opportunity_location_polygon;
drop index if exists c_opportunity_location_point;
drop index if exists c_opportunity_end_dates;
drop index if exists c_opportunity_start_dates;
drop index if exists c_opportunity_is_online;
drop index if exists c_opportunity_cost;
drop index if exists c_opportunity_min_domain;
drop index if exists c_opportunity_max_age;
drop index if exists c_opportunity_min_age;
drop index if exists c_opportunity_tags;
drop index if exists c_opportunity_partner;
drop index if exists c_opportunity_uid;

drop trigger if exists c_set_opportunity_updated on c_opportunity cascade;

drop table if exists c_opportunity;

drop function if exists set_updated;

drop extension if exists postgis_raster;
drop extension if exists postgis;
