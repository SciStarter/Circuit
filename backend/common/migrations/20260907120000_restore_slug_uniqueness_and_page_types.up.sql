-- EntityType::Page has been written to the database in two forms: a legacy
-- page_<layout> name, and the JSON that serialize_enum() produces for a variant
-- carrying data. Normalize the legacy form onto the current one, so there is a
-- single representation for deserialize_enum() to read. This must accompany the
-- deserializer fix: an unparseable entity_type is now an error rather than
-- being silently reinterpreted as a plain opportunity.
--
-- The rendering matters. The search trigger added by
-- 20260317031840_opportunity_typed_columns recognizes a page by
-- starts_with(entity_type, '{"page":') and casts anything else to
-- t_entity_type, which fails for JSON. json_build_object() renders a space
-- before the colon, so it is cast through jsonb, whose text output matches both
-- the trigger and the rows already stored in this form.

update c_opportunity
set entity_type = json_build_object(
      'page', json_build_object('layout', substring(entity_type from 6))
    )::jsonb::text
where entity_type like 'page\_%';

-- The add-opportunities page was flattened to a plain opportunity by a store()
-- that round-tripped it through the broken deserializer, and unaccepted rows
-- are served as 404, so /add-opportunities stopped resolving.

update c_opportunity
set entity_type = '{"page": {"layout": "add_opportunities"}}'::jsonb::text,
    accepted = true
where slug = 'add-opportunities';

-- Slug uniqueness was enforced by a partial unique index on the JSONB exterior
-- until 20260317031840_opportunity_typed_columns dropped that column with
-- cascade, taking the index with it, and recreated c_opportunity_slug as a
-- plain index. Nothing has prevented duplicate slugs since, and
-- get_by_slug_with_overlay.sql resolves a slug with `limit 1` and no ordering,
-- so a duplicate would shadow the original arbitrarily. Blank slugs no longer
-- need excluding: set_slug_if_necessary() falls back to the uid.

drop index if exists c_opportunity_slug;
create unique index c_opportunity_slug on c_opportunity (lower(slug));
