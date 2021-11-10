drop view if exists c_partner_stats;
alter table c_opportunity drop column if exists "current";
drop function if exists c_opportunity_is_current;
