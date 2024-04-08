begin;

create type t_entity_type as enum (
  'unspecified',
  'attraction',
  'page',
  'opportunity'
);

create type t_cost as enum (
  'free',
  'cost',
  'unknown'
);

create type t_venue_type as enum (
  'indoors',
  'outdoors',
  -- Below are deprecated but still present in some records
  'museum_or_science_center',
  'library',
  'pk12school',
  'community_organization',
  'bar',
  'college_university',
  'unspecified'
);

create type t_location_type as enum (
  'any',
  'at',
  'near',
  'unknown'
);

create table c_opportunity_search (
  id serial primary key,
  uid UUID not null unique,
  slug text not null unique, 
  created timestamptz not null,
  updated timestamptz not null,
  accepted boolean not null,
  withdrawn boolean not null,
  start_datetimes timestamptz[] not null,
  end_datetimes timestamptz[] not null,
  entity_type t_entity_type not null,
  title text not null,
  tags text[] not null,
  topics text[] not null,
  descriptors text[] not null,
  partner UUID not null,
  min_age smallint not null default 0,
  max_age smallint not null default 999,
  "cost" t_cost not null,
  venue_type t_venue_type[] not null,
  organization_name text not null,
  is_online boolean not null,
  location_type t_location_type not null,
  short_desc text not null,
  description text not null,
  image_url text not null,
  image_credit text not null,
  
  fulltext_english tsvector,
  location_point geography(Point,4326),
  location_polygon geography(MultiPolygon,4326),

  opp_id integer unique not null references c_opportunity (id)
);

insert into c_opportunity_search (
  uid,
  slug,
  created,
  updated,
  accepted,
  withdrawn,
  start_datetimes,
  end_datetimes,
  entity_type,
  title,
  tags,
  topics,
  descriptors,
  partner,
  min_age,
  max_age,
  "cost",
  venue_type,
  organization_name,
  is_online,
  location_type,
  short_desc,
  description,
  image_url,
  image_credit,
  fulltext_english,
  location_point,
  location_polygon,
  opp_id
)
select
  (exterior->>'uid')::UUID,
  (exterior->>'slug'),
  created,
  updated,
  (interior->>'accepted') = 'true',
  (interior->>'withdrawn') = 'true',
  (select coalesce(array_agg(i_starts)::timestamptz[], '{}'::timestamptz[]) from jsonb_array_elements_text(exterior->'start_datetimes') i_starts),
  (select coalesce(array_agg(i_ends)::timestamptz[], '{}'::timestamptz[]) from jsonb_array_elements_text(exterior->'end_datetimes') i_ends),
  (exterior->>'entity_type')::t_entity_type,
  (exterior->>'title'),
  (select coalesce(array_agg(i_tags), '{}'::text[]) from jsonb_array_elements_text(exterior->'tags') i_tags),
  (select coalesce(array_agg(i_topics), '{}'::text[]) from jsonb_array_elements_text(exterior->'topics') i_topics),
  (select coalesce(array_agg(i_descriptors), '{}'::text[]) from jsonb_array_elements_text(exterior->'opp_descriptor') i_descriptors),
  (exterior->>'partner')::UUID,
  (exterior->'min_age')::smallint,
  (exterior->'max_age')::smallint,
  (exterior->>'cost')::t_cost,
  (select coalesce(array_agg(i_venue_type)::t_venue_type[], '{}'::t_venue_type[]) from jsonb_array_elements_text(exterior->'venue_type') i_venue_type),
  (exterior->>'organization_name'),
  ((exterior->>'is_online') = 'true'),
  (exterior->>'location_type')::t_location_type,
  (exterior->>'short_desc'),
  (exterior->>'description'),
  (exterior->>'image_url'),
  (exterior->>'image_credit'),
  (
    to_tsvector('english', exterior->>'title') ||
    to_tsvector('english', exterior->>'description') ||
    to_tsvector('english', exterior->>'partner_name') ||
    to_tsvector('english', exterior->>'organization_name') ||
    to_tsvector('english', exterior->>'location_name') ||
    to_tsvector('english', exterior->>'address_city') ||
    to_tsvector('english', exterior->>'address_state') ||
    to_tsvector('english', exterior->>'address_country') ||
    to_tsvector('english', exterior->'tags') ||
    to_tsvector('english', exterior->'opp_topics') ||
    to_tsvector('english', exterior->'opp_descriptor') ||
    to_tsvector('english', exterior->'opp_hashtags') ||
    to_tsvector('english', exterior->'opp_social_handles')
  ),
  (
    case
      when (exterior #>> '{location_point,type}') = 'Point'
        and c_valid_geojson(exterior->'location_point')
      then c_make_valid_geography(st_geomfromgeojson(exterior->'location_point'))
      else null
    end
  ),
  (
    case
      when (exterior #>> '{location_polygon,type}') = any(array['Polygon', 'MultiPolygon'])
        and c_valid_geojson(exterior->'location_polygon')
      then c_make_valid_geography(st_geomfromgeojson(exterior->'location_polygon'))
      else null
    end
  ),
  id
from c_opportunity
;

create index c_opportunity_search_via_fulltext_english on c_opportunity_search using gin(fulltext_english);
create index c_opportunity_search_via_location_point on c_opportunity_search using gist (location_point);
create index c_opportunity_search_via_location_polygon on c_opportunity_search using gist (location_polygon);

create function c_opportunity_search_update() returns trigger as
$body$
begin
  insert into c_opportunity_search (
    uid,
    slug,
    created,
    updated,
    accepted,
    withdrawn,
    start_datetimes,
    end_datetimes,
    entity_type,
    title,
    tags,
    topics,
    descriptors,
    partner,
    min_age,
    max_age,
    "cost",
    venue_type,
    organization_name,
    is_online,
    location_type,
    short_desc,
    description,
    image_url,
    image_credit,
    fulltext_english,
    location_point,
    location_polygon,
    opp_id
  )
  values (
    (new.exterior->>'uid')::UUID,
    (new.exterior->>'slug'),
    new.created,
    new.updated,
    (new.interior->>'accepted') = 'true',
    (new.interior->>'withdrawn') = 'true',
    (select coalesce(array_agg(i_starts)::timestamptz[], '{}'::timestamptz[]) from jsonb_array_elements_text(new.exterior->'start_datetimes') i_starts),
    (select coalesce(array_agg(i_ends)::timestamptz[], '{}'::timestamptz[]) from jsonb_array_elements_text(new.exterior->'end_datetimes') i_ends),
    (new.exterior->>'entity_type')::t_entity_type,
    (new.exterior->>'title'),
    (select coalesce(array_agg(i_tags), '{}'::text[]) from jsonb_array_elements_text(new.exterior->'tags') i_tags),
    (select coalesce(array_agg(i_topics), '{}'::text[]) from jsonb_array_elements_text(new.exterior->'topics') i_topics),
    (select coalesce(array_agg(i_descriptors), '{}'::text[]) from jsonb_array_elements_text(new.exterior->'opp_descriptor') i_descriptors),
    (new.exterior->>'partner')::UUID,
    (new.exterior->'min_age')::smallint,
    (new.exterior->'max_age')::smallint,
    (new.exterior->>'cost')::t_cost,
    (select coalesce(array_agg(i_venue_type)::t_venue_type[], '{}'::t_venue_type[]) from jsonb_array_elements_text(new.exterior->'venue_type') i_venue_type),
    (new.exterior->>'organization_name'),
    ((new.exterior->>'is_online') = 'true'),
    (new.exterior->>'location_type')::t_location_type,
    (new.exterior->>'short_desc'),
    (new.exterior->>'description'),
    (new.exterior->>'image_url'),
    (new.exterior->>'image_credit'),
    (
      to_tsvector('english', new.exterior->>'title') ||
      to_tsvector('english', new.exterior->>'description') ||
      to_tsvector('english', new.exterior->>'partner_name') ||
      to_tsvector('english', new.exterior->>'organization_name') ||
      to_tsvector('english', new.exterior->>'location_name') ||
      to_tsvector('english', new.exterior->>'address_city') ||
      to_tsvector('english', new.exterior->>'address_state') ||
      to_tsvector('english', new.exterior->>'address_country') ||
      to_tsvector('english', new.exterior->'tags') ||
      to_tsvector('english', new.exterior->'opp_topics') ||
      to_tsvector('english', new.exterior->'opp_descriptor') ||
      to_tsvector('english', new.exterior->'opp_hashtags') ||
      to_tsvector('english', new.exterior->'opp_social_handles')
    ),
    (
      case
        when (
          new.exterior #>> '{location_point,type}') = 'Point' and
          c_valid_geojson(new.exterior -> 'location_point'
        )
        then c_make_valid_geography(st_geomfromgeojson(new.exterior->'location_point'))
        else null
      end
    ),
    (
      case
        when (
          (new.exterior #>> '{location_polygon,type}') = any(array['Polygon', 'MultiPolygon']) and
          c_valid_geojson(new.exterior -> 'location_polygon')
        )
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
    start_datetimes = excluded.start_datetimes,
    end_datetimes = excluded.end_datetimes,
    entity_type = excluded.entity_type,
    title = excluded.title,
    tags = excluded.tags,
    topics = excluded.topics,
    descriptors = excluded.descriptors,
    partner = excluded.partner,
    min_age = excluded.max_age,
    max_age = excluded.min_age,
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

create function c_opportunity_search_delete() returns trigger as
$body$
begin
  delete from c_opportunity_search where opp_id = old.id;
  return new;
end;
$body$
language plpgsql;

create trigger c_opportunity_search_on_opportunity_delete before delete on c_opportunity
for each row execute function c_opportunity_search_delete();

create trigger c_opportunity_search_on_opportunity_insert_or_update after insert or update on c_opportunity
for each row execute function c_opportunity_search_update();

commit;
