-- alter table c_opportunity add column if not exists "current" boolean generated always as (
--   c_opportunity_is_current(interior, exterior)
-- ) stored;
