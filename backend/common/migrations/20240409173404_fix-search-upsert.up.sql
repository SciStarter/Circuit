begin;

create or replace function c_opportunity_search_update() returns trigger as
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
    case
      when starts_with(new.exterior->>'entity_type', '{"page":') then 'page'::t_entity_type
      when starts_with(new.exterior->>'entity_type', 'page_') then 'page'::t_entity_type
      else (new.exterior->>'entity_type')::t_entity_type
    end,
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

commit;
