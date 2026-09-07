drop index if exists c_opportunity_slug;
create index c_opportunity_slug on c_opportunity (lower(slug));

-- The entity_type normalization and the add-opportunities repair are not
-- reversed: both move rows from a broken state to a correct one, and restoring
-- the broken state would serve the pages as plain opportunities again.
