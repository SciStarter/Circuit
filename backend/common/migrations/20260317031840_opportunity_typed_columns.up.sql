begin;

-- Phase 1: Drop dependent objects

-- Drop generated geography columns (they depend on exterior/interior)
alter table c_opportunity drop column if exists location_point;
alter table c_opportunity drop column if exists location_polygon;

-- Drop the generated fulltext column (depends on exterior)
alter table c_opportunity drop column if exists fulltext_english;

-- Drop the generated "current" column if it exists
alter table c_opportunity drop column if exists "current";

-- Drop the view that depends on opportunity exterior/interior
drop view if exists c_partner_stats;

-- Drop triggers that depend on the JSONB columns
drop trigger if exists c_opportunity_search_on_opportunity_insert_or_update on c_opportunity;

-- Phase 2: Add typed columns

alter table c_opportunity
  add column uid uuid,
  add column slug text not null default '',
  add column partner_name text not null default '',
  add column partner_website text,
  add column partner_logo_url text,
  add column partner_created timestamptz,
  add column partner_updated timestamptz,
  add column partner_opp_url text,
  add column organization_name text not null default '',
  add column organization_type text not null default 'unspecified',
  add column organization_website text,
  add column organization_logo_url text,
  add column entity_type text not null default 'opportunity',
  add column opp_venue text[] not null default '{}',
  add column opp_descriptor text[] not null default '{}',
  add column min_age smallint not null default 0,
  add column max_age smallint not null default 999,
  add column pes_domain text not null default 'unspecified',
  add column tags text[] not null default '{}',
  add column opp_topics text[] not null default '{}',
  add column ticket_required boolean not null default false,
  add column title text not null default '',
  add column description text not null default '',
  add column short_desc text not null default '',
  add column image_url text not null default '',
  add column image_credit text not null default '',
  add column start_datetimes timestamptz[] not null default '{}',
  add column has_end boolean not null default false,
  add column end_datetimes timestamptz[] not null default '{}',
  add column recurrence text not null default 'once',
  add column end_recurrence timestamptz,
  add column timezone text,
  add column attraction_hours jsonb,
  add column cost text not null default 'free',
  add column languages text[] not null default '{en-US}',
  add column is_online boolean not null default false,
  add column location_type text not null default 'any',
  add column location_name text not null default '',
  add column location_point_geojson jsonb,
  add column location_polygon_geojson jsonb,
  add column address_street text not null default '',
  add column address_city text not null default '',
  add column address_state text not null default '',
  add column address_country text not null default '',
  add column address_zip text not null default '',
  add column opp_hashtags text[] not null default '{}',
  add column opp_social_handles jsonb not null default '{}',
  add column opp_partner uuid,
  -- interior fields
  add column accepted boolean,
  add column withdrawn boolean not null default false,
  add column submitted_by uuid,
  add column review_status text not null default 'not_required',
  add column contact_name text not null default '',
  add column contact_email text not null default '',
  add column contact_phone text not null default '',
  add column extra_data jsonb not null default '{}';

-- Phase 3: Backfill from JSONB

update c_opportunity set
  uid = (exterior->>'uid')::uuid,
  slug = coalesce(exterior->>'slug', ''),
  partner_name = coalesce(exterior->>'partner_name', ''),
  partner_website = exterior->>'partner_website',
  partner_logo_url = exterior->>'partner_logo_url',
  partner_created = (exterior->>'partner_created')::timestamptz,
  partner_updated = (exterior->>'partner_updated')::timestamptz,
  partner_opp_url = exterior->>'partner_opp_url',
  organization_name = coalesce(exterior->>'organization_name', ''),
  organization_type = coalesce(exterior->>'organization_type', 'unspecified'),
  organization_website = exterior->>'organization_website',
  organization_logo_url = exterior->>'organization_logo_url',
  entity_type = coalesce(exterior->>'entity_type', 'opportunity'),
  opp_venue = (select coalesce(array_agg(v::text), '{}') from jsonb_array_elements_text(coalesce(exterior->'opp_venue', '[]'::jsonb)) v),
  opp_descriptor = (select coalesce(array_agg(v::text), '{}') from jsonb_array_elements_text(coalesce(exterior->'opp_descriptor', '[]'::jsonb)) v),
  min_age = coalesce((exterior->'min_age')::smallint, 0),
  max_age = coalesce((exterior->'max_age')::smallint, 999),
  pes_domain = coalesce(exterior->>'pes_domain', 'unspecified'),
  tags = (select coalesce(array_agg(v::text), '{}') from jsonb_array_elements_text(coalesce(exterior->'tags', '[]'::jsonb)) v),
  opp_topics = (select coalesce(array_agg(v::text), '{}') from jsonb_array_elements_text(coalesce(exterior->'opp_topics', '[]'::jsonb)) v),
  ticket_required = coalesce((exterior->>'ticket_required')::boolean, false),
  title = coalesce(exterior->>'title', ''),
  description = coalesce(exterior->>'description', ''),
  short_desc = coalesce(exterior->>'short_desc', ''),
  image_url = coalesce(exterior->>'image_url', ''),
  image_credit = coalesce(exterior->>'image_credit', ''),
  start_datetimes = (select coalesce(array_agg(v::timestamptz), '{}'::timestamptz[]) from jsonb_array_elements_text(coalesce(exterior->'start_datetimes', '[]'::jsonb)) v),
  has_end = coalesce((exterior->>'has_end')::boolean, false),
  end_datetimes = (select coalesce(array_agg(v::timestamptz), '{}'::timestamptz[]) from jsonb_array_elements_text(coalesce(exterior->'end_datetimes', '[]'::jsonb)) v),
  recurrence = coalesce(exterior->>'recurrence', 'once'),
  end_recurrence = (exterior->>'end_recurrence')::timestamptz,
  timezone = exterior->>'timezone',
  attraction_hours = exterior->'attraction_hours',
  cost = coalesce(exterior->>'cost', 'free'),
  languages = (select coalesce(array_agg(v::text), '{en-US}') from jsonb_array_elements_text(coalesce(exterior->'languages', '["en-US"]'::jsonb)) v),
  is_online = coalesce((exterior->>'is_online')::boolean, false),
  location_type = coalesce(exterior->>'location_type', 'any'),
  location_name = coalesce(exterior->>'location_name', ''),
  location_point_geojson = case
    when exterior->'location_point' is not null and jsonb_typeof(exterior->'location_point') = 'object'
    then exterior->'location_point'
    else null
  end,
  location_polygon_geojson = case
    when exterior->'location_polygon' is not null and jsonb_typeof(exterior->'location_polygon') = 'object'
    then exterior->'location_polygon'
    else null
  end,
  address_street = coalesce(exterior->>'address_street', ''),
  address_city = coalesce(exterior->>'address_city', ''),
  address_state = coalesce(exterior->>'address_state', ''),
  address_country = coalesce(exterior->>'address_country', ''),
  address_zip = coalesce(exterior->>'address_zip', ''),
  opp_hashtags = (select coalesce(array_agg(v::text), '{}') from jsonb_array_elements_text(coalesce(exterior->'opp_hashtags', '[]'::jsonb)) v),
  opp_social_handles = coalesce(exterior->'opp_social_handles', '{}'::jsonb),
  opp_partner = (exterior->>'partner')::uuid,
  accepted = (interior->>'accepted')::boolean,
  withdrawn = coalesce((interior->>'withdrawn')::boolean, false),
  submitted_by = (interior->>'submitted_by')::uuid,
  review_status = coalesce(nullif(interior->>'review_status', ''), 'not_required'),
  contact_name = coalesce(interior->>'contact_name', ''),
  contact_email = coalesce(interior->>'contact_email', ''),
  contact_phone = coalesce(interior->>'contact_phone', ''),
  extra_data = coalesce(interior->'extra_data', '{}'::jsonb);

-- Add NOT NULL constraint on uid
alter table c_opportunity
  alter column uid set not null,
  alter column opp_partner set not null;

-- Phase 4: Drop old JSONB columns and indexes

drop index if exists c_opportunity_uid;
drop index if exists c_opportunity_partner;
drop index if exists c_opportunity_tags;
drop index if exists c_opportunity_min_age;
drop index if exists c_opportunity_max_age;
drop index if exists c_opportunity_min_domain;
drop index if exists c_opportunity_cost;
drop index if exists c_opportunity_is_online;
drop index if exists c_opportunity_start_dates;
drop index if exists c_opportunity_end_dates;
drop index if exists c_opportunity_location_point;
drop index if exists c_opportunity_location_polygon;
drop index if exists c_opportunity_fulltext_english;
drop index if exists c_opportunity_withdrawn;

alter table c_opportunity
  drop column exterior,
  drop column interior;

-- Phase 5: Create new indexes

create unique index c_opportunity_uid on c_opportunity (uid);
create index c_opportunity_slug on c_opportunity (lower(slug));
create index c_opportunity_partner on c_opportunity (opp_partner);
create index c_opportunity_tags on c_opportunity using GIN (tags);

-- Phase 6: Recreate geography columns as generated from GeoJSON
alter table c_opportunity add column location_point geography(POINT, 4326) generated always as (
  case
    when location_point_geojson is not null
      and jsonb_typeof(location_point_geojson) = 'object'
      and (location_point_geojson->>'type') = 'Point'
      and c_valid_geojson(location_point_geojson)
    then c_make_valid_geography(st_geomfromgeojson(location_point_geojson))
    else null
  end
) stored;

alter table c_opportunity add column location_polygon geography(MULTIPOLYGON, 4326) generated always as (
  case
    when location_polygon_geojson is not null
      and jsonb_typeof(location_polygon_geojson) = 'object'
      and (location_polygon_geojson->>'type') = any(array['Polygon', 'MultiPolygon'])
      and c_valid_geojson(location_polygon_geojson)
    then c_make_valid_geography(st_geomfromgeojson(location_polygon_geojson))
    else null
  end
) stored;

create index c_opportunity_location_point on c_opportunity using SPGIST (location_point);
create index c_opportunity_location_polygon on c_opportunity using SPGIST (location_polygon);

-- Phase 7: Recreate PL/pgSQL functions with typed parameters

-- Drop old JSONB-parameter functions
drop function if exists c_opportunity_is_current(jsonb, jsonb);
drop function if exists c_opportunity_is_current_as_of(jsonb, jsonb, timestamptz);
drop function if exists c_opportunity_is_scheduled(jsonb, jsonb);
drop function if exists c_opportunity_is_ondemand(jsonb, jsonb);
drop function if exists c_opportunity_by_uid_is_current(uuid);
drop function if exists c_opportunity_by_uid_is_current_as_of(uuid, timestamptz);

-- New function using record type
create or replace function c_opportunity_is_current_as_of(opp c_opportunity, stamp timestamptz) returns boolean as
$func$
BEGIN
 RETURN (
   coalesce(opp.review_status, 'not_required') IN ('publish', 'not_required')
   AND
   opp.accepted = true
   AND
   opp.withdrawn = false
   AND
   (
       (
          coalesce(array_length(opp.start_datetimes, 1), 0) <= 1
          AND
          coalesce(array_length(opp.end_datetimes, 1), 0) = 0
       )
       OR
       EXISTS (SELECT value FROM unnest(opp.start_datetimes) t(value) WHERE value > stamp)
       OR
       EXISTS (SELECT value FROM unnest(opp.end_datetimes) t(value) WHERE value > stamp)
       OR
       (
          (opp.recurrence = 'daily' OR opp.recurrence = 'weekly')
          AND
          (opp.end_recurrence IS null OR opp.end_recurrence > stamp)
       )
   )
 );
END
$func$ language plpgsql stable;

create or replace function c_opportunity_is_current(opp c_opportunity) returns boolean as
$func$
BEGIN
 RETURN c_opportunity_is_current_as_of(opp, CURRENT_TIMESTAMP);
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_is_current_as_of(p_uid uuid, stamp timestamptz) returns boolean as
$func$
DECLARE
 opp c_opportunity;
BEGIN
 SELECT * INTO opp FROM c_opportunity WHERE uid = p_uid;
 IF NOT FOUND THEN RETURN false; END IF;
 RETURN c_opportunity_is_current_as_of(opp, stamp);
END
$func$ language plpgsql stable;

create or replace function c_opportunity_by_uid_is_current(p_uid uuid) returns boolean as
$func$
BEGIN
 RETURN c_opportunity_by_uid_is_current_as_of(p_uid, CURRENT_TIMESTAMP);
END
$func$ language plpgsql stable;

-- Phase 8: Recreate the search trigger to use typed columns

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
    new.uid,
    new.slug,
    new.created,
    new.updated,
    coalesce(new.accepted, false),
    new.withdrawn,
    new.review_status::t_review_status,
    new.start_datetimes,
    new.end_datetimes,
    new.recurrence::t_recurrence,
    new.end_recurrence,
    case
      when starts_with(new.entity_type, '{"page":') then 'page'::t_entity_type
      when starts_with(new.entity_type, 'page_') then 'page'::t_entity_type
      else new.entity_type::t_entity_type
    end,
    new.title,
    new.tags,
    new.opp_topics,
    new.opp_descriptor,
    new.opp_partner,
    new.min_age,
    new.max_age,
    new.cost::t_cost,
    (select coalesce(array_agg(v::t_venue_type), '{}'::t_venue_type[]) from unnest(new.opp_venue) v),
    new.organization_name,
    new.is_online,
    new.location_type::t_location_type,
    new.short_desc,
    new.description,
    new.image_url,
    new.image_credit,
    (
      to_tsvector('english', new.title) ||
      to_tsvector('english', new.description) ||
      to_tsvector('english', new.partner_name) ||
      to_tsvector('english', new.organization_name) ||
      to_tsvector('english', new.location_name) ||
      to_tsvector('english', new.address_city) ||
      to_tsvector('english', new.address_state) ||
      to_tsvector('english', new.address_country) ||
      to_tsvector('english', array_to_string(new.tags, ' ')) ||
      to_tsvector('english', array_to_string(new.opp_topics, ' ')) ||
      to_tsvector('english', array_to_string(new.opp_descriptor, ' ')) ||
      to_tsvector('english', array_to_string(new.opp_hashtags, ' '))
    ),
    (
      case
        when new.location_point_geojson is not null
          and (new.location_point_geojson->>'type') = 'Point'
          and c_valid_geojson(new.location_point_geojson)
        then c_make_valid_geography(st_geomfromgeojson(new.location_point_geojson))
        else null
      end
    ),
    (
      case
        when new.location_polygon_geojson is not null
          and (new.location_polygon_geojson->>'type') = any(array['Polygon', 'MultiPolygon'])
          and c_valid_geojson(new.location_polygon_geojson)
        then c_make_valid_geography(st_geomfromgeojson(new.location_polygon_geojson))
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

-- Recreate the trigger
create trigger c_opportunity_search_on_opportunity_insert_or_update
  after insert or update on c_opportunity
  for each row execute procedure c_opportunity_search_update();

-- Phase 9: Recreate the c_partner_stats view

create view c_partner_stats
as select
  count(c_opportunity.*) as "total",
  min(c_partner.uid::text) as "uid",
  min(c_partner."name") as "name"
  from c_opportunity inner join c_partner
  on c_opportunity.opp_partner = c_partner.uid
  where (EXISTS
         (SELECT value
          FROM unnest(c_opportunity.start_datetimes) t(value)
          WHERE value > CURRENT_TIMESTAMP
         )
         OR
         EXISTS
          (SELECT value
           FROM unnest(c_opportunity.end_datetimes) t(value)
           WHERE value > CURRENT_TIMESTAMP
          )
         OR
          (
           coalesce(array_length(c_opportunity.start_datetimes, 1), 0) = 0
           AND
           coalesce(array_length(c_opportunity.end_datetimes, 1), 0) = 0
          )
        )
        AND
        c_opportunity.accepted = true
        AND
        c_opportunity.withdrawn = false
        GROUP BY c_opportunity.opp_partner;

commit;
