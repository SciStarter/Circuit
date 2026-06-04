-- Single source of truth for the "published" definition of an opportunity:
-- it has been accepted and has not been withdrawn. This mirrors the inline
-- "accepted != false and withdrawn != true" predicate previously duplicated in
-- db/partner/catalog_extra.sql and is the all-time analogue of
-- c_opportunity_is_current (which additionally requires the opportunity to be
-- live right now).

create or replace function c_opportunity_is_published(opp c_opportunity) returns boolean as
$func$
BEGIN
 RETURN coalesce(opp.accepted, false) AND NOT opp.withdrawn;
END
$func$ language plpgsql stable;
