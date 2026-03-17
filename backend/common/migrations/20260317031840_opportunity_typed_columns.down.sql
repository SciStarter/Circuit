begin;

-- Phase 1: Drop dependent objects

drop trigger if exists c_opportunity_search_on_opportunity_insert_or_update on c_opportunity;
drop view if exists c_partner_stats;
alter table c_opportunity drop column if exists location_point;
alter table c_opportunity drop column if exists location_polygon;

-- Drop new indexes
drop index if exists c_opportunity_uid;
drop index if exists c_opportunity_slug;
drop index if exists c_opportunity_partner;
drop index if exists c_opportunity_tags;

-- Drop new-signature functions
drop function if exists c_opportunity_is_current(c_opportunity);
drop function if exists c_opportunity_is_current_as_of(c_opportunity, timestamptz);
drop function if exists c_opportunity_by_uid_is_current(uuid);
drop function if exists c_opportunity_by_uid_is_current_as_of(uuid, timestamptz);

-- Phase 2: Add back JSONB columns

alter table c_opportunity
  add column exterior jsonb not null default '{}'::jsonb,
  add column interior jsonb not null default '{}'::jsonb;

-- Phase 3: Backfill JSONB from typed columns

update c_opportunity set
  exterior = jsonb_build_object(
    'uid', uid,
    'slug', slug,
    'partner_name', partner_name,
    'partner_website', partner_website,
    'partner_logo_url', partner_logo_url,
    'partner_created', partner_created,
    'partner_updated', partner_updated,
    'partner_opp_url', partner_opp_url,
    'organization_name', organization_name,
    'organization_type', organization_type,
    'organization_website', organization_website,
    'organization_logo_url', organization_logo_url,
    'entity_type', entity_type,
    'opp_venue', to_jsonb(opp_venue),
    'opp_descriptor', to_jsonb(opp_descriptor),
    'min_age', min_age,
    'max_age', max_age,
    'pes_domain', pes_domain,
    'tags', to_jsonb(tags),
    'opp_topics', to_jsonb(opp_topics),
    'ticket_required', ticket_required,
    'title', title,
    'description', description,
    'short_desc', short_desc,
    'image_url', image_url,
    'image_credit', image_credit,
    'start_datetimes', to_jsonb(start_datetimes),
    'has_end', has_end,
    'end_datetimes', to_jsonb(end_datetimes),
    'recurrence', recurrence,
    'end_recurrence', end_recurrence,
    'timezone', timezone,
    'attraction_hours', attraction_hours,
    'cost', cost,
    'languages', to_jsonb(languages),
    'is_online', is_online,
    'location_type', location_type,
    'location_name', location_name,
    'location_point', location_point_geojson,
    'location_polygon', location_polygon_geojson,
    'address_street', address_street,
    'address_city', address_city,
    'address_state', address_state,
    'address_country', address_country,
    'address_zip', address_zip,
    'opp_hashtags', to_jsonb(opp_hashtags),
    'opp_social_handles', opp_social_handles,
    'partner', opp_partner
  ),
  interior = jsonb_build_object(
    'accepted', accepted,
    'withdrawn', withdrawn,
    'submitted_by', submitted_by,
    'review_status', review_status,
    'contact_name', contact_name,
    'contact_email', contact_email,
    'contact_phone', contact_phone,
    'extra_data', extra_data
  );

-- Phase 4: Drop typed columns

alter table c_opportunity
  drop column uid,
  drop column slug,
  drop column partner_name,
  drop column partner_website,
  drop column partner_logo_url,
  drop column partner_created,
  drop column partner_updated,
  drop column partner_opp_url,
  drop column organization_name,
  drop column organization_type,
  drop column organization_website,
  drop column organization_logo_url,
  drop column entity_type,
  drop column opp_venue,
  drop column opp_descriptor,
  drop column min_age,
  drop column max_age,
  drop column pes_domain,
  drop column tags,
  drop column opp_topics,
  drop column ticket_required,
  drop column title,
  drop column description,
  drop column short_desc,
  drop column image_url,
  drop column image_credit,
  drop column start_datetimes,
  drop column has_end,
  drop column end_datetimes,
  drop column recurrence,
  drop column end_recurrence,
  drop column timezone,
  drop column attraction_hours,
  drop column cost,
  drop column languages,
  drop column is_online,
  drop column location_type,
  drop column location_name,
  drop column location_point_geojson,
  drop column location_polygon_geojson,
  drop column address_street,
  drop column address_city,
  drop column address_state,
  drop column address_country,
  drop column address_zip,
  drop column opp_hashtags,
  drop column opp_social_handles,
  drop column opp_partner,
  drop column accepted,
  drop column withdrawn,
  drop column submitted_by,
  drop column review_status,
  drop column contact_name,
  drop column contact_email,
  drop column contact_phone,
  drop column extra_data;

-- Phase 5: Recreate old GIN indexes on JSONB

create index c_opportunity_uid on c_opportunity using GIN ((exterior -> 'uid'));
create index c_opportunity_partner on c_opportunity using GIN ((exterior -> 'partner'));
create index c_opportunity_tags on c_opportunity using GIN ((exterior -> 'tags'));
create index c_opportunity_min_age on c_opportunity using GIN ((exterior -> 'min_age'));
create index c_opportunity_max_age on c_opportunity using GIN ((exterior -> 'max_age'));
create index c_opportunity_min_domain on c_opportunity using GIN ((exterior -> 'pes_domain'));
create index c_opportunity_cost on c_opportunity using GIN ((exterior -> 'cost'));
create index c_opportunity_is_online on c_opportunity using GIN ((exterior -> 'is_online'));
create index c_opportunity_start_dates on c_opportunity using GIN ((exterior -> 'end_dates'));
create index c_opportunity_end_dates on c_opportunity using GIN ((exterior -> 'start_dates'));
create index c_opportunity_fulltext_english on c_opportunity using GIN (to_tsvector('english', (exterior -> 'title')::text || ' ' || (exterior -> 'description')::text));
create index c_opportunity_withdrawn on c_opportunity using GIN ((interior -> 'withdrawn'));

-- Phase 6: Recreate geography columns from JSONB

alter table c_opportunity add column location_point geography(POINT, 4326)
generated always as (
  case
    when exterior #>> '{location_point,type}' = 'Point' and c_valid_geojson(exterior -> 'location_point')
    then c_make_valid_geography(ST_GeomFromGeoJSON(exterior -> 'location_point'))
    else null
  end
) stored;

alter table c_opportunity add column location_polygon geography(MULTIPOLYGON, 4326)
generated always as (
  case
    when exterior #>> '{location_polygon,type}' in ('Polygon', 'MultiPolygon') and c_valid_geojson(exterior -> 'location_polygon')
    then c_make_valid_geography(ST_GeomFromGeoJSON(exterior -> 'location_polygon'))
    else null
  end
) stored;

create index c_opportunity_location_point on c_opportunity using SPGIST (location_point);
create index c_opportunity_location_polygon on c_opportunity using SPGIST (location_polygon);

-- Recreate fulltext_english generated column
alter table c_opportunity add column fulltext_english tsvector generated always as (
  to_tsvector('english', (exterior ->> 'title')) ||
  to_tsvector('english', (exterior ->> 'description')) ||
  to_tsvector('english', (exterior ->> 'partner_name')) ||
  to_tsvector('english', (exterior ->> 'organization_name')) ||
  to_tsvector('english', (exterior ->> 'location_name')) ||
  to_tsvector('english', (exterior ->> 'address_city')) ||
  to_tsvector('english', (exterior ->> 'address_state')) ||
  to_tsvector('english', (exterior ->> 'address_country')) ||
  to_tsvector('english', (exterior -> 'tags')) ||
  to_tsvector('english', (exterior -> 'opp_topics')) ||
  to_tsvector('english', (exterior -> 'opp_descriptor')) ||
  to_tsvector('english', (exterior -> 'opp_hashtags')) ||
  to_tsvector('english', (exterior -> 'opp_social_handles'))
) stored;

-- Phase 7: Recreate PL/pgSQL functions with old (jsonb, jsonb) signatures

create or replace function c_opportunity_is_current_as_of(interior jsonb, exterior jsonb, stamp timestamptz) returns boolean as
$func$
BEGIN
 RETURN (
   coalesce(nullif(interior ->> 'review_status', ''), 'not_required') IN ('publish', 'not_required')
   AND
   interior ->> 'accepted' = 'true'
   AND
   interior ->> 'withdrawn' = 'false'
   AND
   (
       (
          jsonb_array_length(exterior -> 'start_datetimes') <= 1
          AND
          jsonb_array_length(exterior -> 'end_datetimes') = 0
       )
       OR
       EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'start_datetimes') WHERE value::timestamptz > stamp)
       OR
       EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'end_datetimes') WHERE value::timestamptz > stamp)
       OR
       (
          (exterior->>'recurrence' = 'daily' OR exterior->>'recurrence' = 'weekly')
          AND
          (exterior->>'end_recurrence' IS null OR (exterior->>'end_recurrence')::timestamptz > stamp)
       )
   )
 );
END
$func$ language plpgsql stable;

create or replace function c_opportunity_is_current(interior jsonb, exterior jsonb) returns boolean as
$func$
BEGIN
 RETURN c_opportunity_is_current_as_of(interior, exterior, CURRENT_TIMESTAMP);
END
$func$ language plpgsql stable;

create or replace function c_opportunity_is_scheduled(interior jsonb, exterior jsonb) returns boolean as
$func$
BEGIN
 RETURN (
   jsonb_array_length(exterior -> 'start_datetimes') > 1
   OR
   jsonb_array_length(exterior -> 'end_datetimes') > 1
   OR (
     jsonb_array_length(exterior -> 'start_datetimes') = 1
     AND
     jsonb_array_length(exterior -> 'end_datetimes') = 1
     AND
     (exterior #> '{start_datetimes,0}')::text::date = (exterior #> '{end_datetimes,0}')::text::date
   )
 );
END
$func$ language plpgsql stable;

create or replace function c_opportunity_is_ondemand(interior jsonb, exterior jsonb) returns boolean as
$func$
BEGIN
 RETURN (
   jsonb_array_length(exterior -> 'start_datetimes') <= 1
   AND
   jsonb_array_length(exterior -> 'end_datetimes') <= 1
   AND (
     jsonb_array_length(exterior -> 'start_datetimes') != 1
     OR
     jsonb_array_length(exterior -> 'end_datetimes') != 1
     OR
     (exterior #> '{start_datetimes,0}')::text::date != (exterior #> '{end_datetimes,0}')::text::date
   )
 );
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_is_current_as_of(p_uid uuid, stamp timestamptz) returns boolean as
$func$
DECLARE
 opp_int jsonb;
 opp_ext jsonb;
BEGIN
 SELECT interior, exterior INTO opp_int, opp_ext FROM c_opportunity WHERE (exterior->>'uid')::uuid = p_uid;
 IF NOT FOUND THEN RETURN false; END IF;
 RETURN c_opportunity_is_current_as_of(opp_int, opp_ext, stamp);
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_is_current(p_uid uuid) returns boolean as
$func$
BEGIN
 RETURN c_opportunity_by_uid_is_current_as_of(p_uid, CURRENT_TIMESTAMP);
END
$func$ language plpgsql stable;

-- Phase 8: Recreate the search trigger with old JSONB-based logic

create or replace function c_opportunity_search_update() returns trigger as
$body$
begin
  insert into c_opportunity_search (
    uid, slug, created, updated,
    accepted, withdrawn, review_status,
    start_datetimes, end_datetimes, recurrence, end_recurrence,
    entity_type, title, tags, topics, descriptors,
    partner, min_age, max_age, "cost", venue_type,
    organization_name, is_online, location_type,
    short_desc, description, image_url, image_credit,
    fulltext_english, location_point, location_polygon,
    opp_id
  )
  values (
    (new.exterior->>'uid')::UUID,
    (new.exterior->>'slug'),
    new.created,
    new.updated,
    (new.interior->>'accepted') = 'true',
    (new.interior->>'withdrawn') = 'true',
    coalesce(nullif(new.interior->>'review_status', ''), 'not_required')::t_review_status,
    (select coalesce(array_agg(i_starts)::timestamptz[], '{}'::timestamptz[]) from jsonb_array_elements_text(new.exterior->'start_datetimes') i_starts),
    (select coalesce(array_agg(i_ends)::timestamptz[], '{}'::timestamptz[]) from jsonb_array_elements_text(new.exterior->'end_datetimes') i_ends),
    (new.exterior->>'recurrence')::t_recurrence,
    (new.exterior->>'end_recurrence')::timestamptz,
    case
      when starts_with(new.exterior->>'entity_type', '{"page":') then 'page'::t_entity_type
      when starts_with(new.exterior->>'entity_type', 'page_') then 'page'::t_entity_type
      else (new.exterior->>'entity_type')::t_entity_type
    end,
    (new.exterior->>'title'),
    (select coalesce(array_agg(i_tags), '{}'::text[]) from jsonb_array_elements_text(new.exterior->'tags') i_tags),
    (select coalesce(array_agg(i_topics), '{}'::text[]) from jsonb_array_elements_text(new.exterior->'opp_topics') i_topics),
    (select coalesce(array_agg(i_descriptors), '{}'::text[]) from jsonb_array_elements_text(new.exterior->'opp_descriptor') i_descriptors),
    (new.exterior->>'partner')::UUID,
    (new.exterior->'min_age')::smallint,
    (new.exterior->'max_age')::smallint,
    (new.exterior->>'cost')::t_cost,
    (select coalesce(array_agg(i_venue_type)::t_venue_type[], '{}'::t_venue_type[]) from jsonb_array_elements_text(new.exterior->'opp_venue') i_venue_type),
    (new.exterior->>'organization_name'),
    ((new.exterior->>'is_online') = 'true'),
    (new.exterior->>'location_type')::t_location_type,
    (new.exterior->>'short_desc'),
    (new.exterior->>'description'),
    (new.exterior->>'image_url'),
    (new.exterior->>'image_credit'),
    (
      to_tsvector('english', coalesce(new.exterior->>'title', '')) ||
      to_tsvector('english', coalesce(new.exterior->>'description', '')) ||
      to_tsvector('english', coalesce(new.exterior->>'partner_name', '')) ||
      to_tsvector('english', coalesce(new.exterior->>'organization_name', '')) ||
      to_tsvector('english', coalesce(new.exterior->>'location_name', '')) ||
      to_tsvector('english', coalesce(new.exterior->>'address_city', '')) ||
      to_tsvector('english', coalesce(new.exterior->>'address_state', '')) ||
      to_tsvector('english', coalesce(new.exterior->>'address_country', '')) ||
      to_tsvector('english', coalesce(new.exterior->'tags', '[]'::jsonb)::text) ||
      to_tsvector('english', coalesce(new.exterior->'opp_topics', '[]'::jsonb)::text) ||
      to_tsvector('english', coalesce(new.exterior->'opp_descriptor', '[]'::jsonb)::text) ||
      to_tsvector('english', coalesce(new.exterior->'opp_hashtags', '[]'::jsonb)::text)
    ),
    (
      case
        when (new.exterior #>> '{location_point,type}') = 'Point'
          and c_valid_geojson(new.exterior -> 'location_point')
        then c_make_valid_geography(st_geomfromgeojson(new.exterior->'location_point'))
        else null
      end
    ),
    (
      case
        when (new.exterior #>> '{location_polygon,type}') = any(array['Polygon', 'MultiPolygon'])
          and c_valid_geojson(new.exterior -> 'location_polygon')
        then c_make_valid_geography(st_geomfromgeojson(new.exterior->'location_polygon'))
        else null
      end
    ),
    new.id
  )
  on conflict (opp_id)
  do update set
    uid = excluded.uid,
    slug = excluded.slug,
    created = excluded.created,
    updated = excluded.updated,
    accepted = excluded.accepted,
    withdrawn = excluded.withdrawn,
    review_status = excluded.review_status,
    start_datetimes = excluded.start_datetimes,
    end_datetimes = excluded.end_datetimes,
    recurrence = excluded.recurrence,
    end_recurrence = excluded.end_recurrence,
    entity_type = excluded.entity_type,
    title = excluded.title,
    tags = excluded.tags,
    topics = excluded.topics,
    descriptors = excluded.descriptors,
    partner = excluded.partner,
    min_age = excluded.min_age,
    max_age = excluded.max_age,
    "cost" = excluded."cost",
    venue_type = excluded.venue_type,
    organization_name = excluded.organization_name,
    is_online = excluded.is_online,
    location_type = excluded.location_type,
    short_desc = excluded.short_desc,
    description = excluded.description,
    image_url = excluded.image_url,
    image_credit = excluded.image_credit,
    fulltext_english = excluded.fulltext_english,
    location_point = excluded.location_point,
    location_polygon = excluded.location_polygon;
    return new;
end;
$body$
language plpgsql;

create trigger c_opportunity_search_on_opportunity_insert_or_update
  after insert or update on c_opportunity
  for each row execute procedure c_opportunity_search_update();

-- Phase 9: Recreate "current" generated column

alter table c_opportunity
add column "current" boolean
generated always as (c_opportunity_is_current(interior, exterior)) stored;

-- Phase 10: Recreate the c_partner_stats view

create view c_partner_stats
as select
  count(c_opportunity.*) as "total",
  min(c_partner.uid::text) as "uid",
  min(c_partner."name") as "name"
  from c_opportunity inner join c_partner
  on c_opportunity.exterior -> 'partner' = to_jsonb(c_partner.uid)
  where (EXISTS
         (SELECT value
          FROM jsonb_array_elements_text(c_opportunity.exterior -> 'start_datetimes')
          WHERE value::timestamptz > CURRENT_TIMESTAMP
         )
         OR
         EXISTS
          (SELECT value
           FROM jsonb_array_elements_text(c_opportunity.exterior -> 'end_datetimes')
           WHERE value::timestamptz > CURRENT_TIMESTAMP
          )
         OR
          (
           jsonb_array_length(c_opportunity.exterior -> 'start_datetimes') = 0
           AND
           jsonb_array_length(c_opportunity.exterior -> 'end_datetimes') = 0
          )
        )
        AND
        c_opportunity.interior ->> 'accepted' = 'true'
        AND
        c_opportunity.interior ->> 'withdrawn' = 'false'
        GROUP BY c_opportunity.exterior -> 'partner';

commit;
