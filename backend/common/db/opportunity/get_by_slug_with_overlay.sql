select o.id, o.uid, o.slug, o.partner_name, o.partner_website, o.partner_logo_url,
  o.partner_created, o.partner_updated, o.partner_opp_url,
  o.organization_name, o.organization_type, o.organization_website, o.organization_logo_url,
  o.entity_type, o.opp_venue, o.opp_descriptor, o.min_age, o.max_age, o.pes_domain,
  o.tags, o.opp_topics, o.ticket_required,
  o.title, o.description, o.short_desc, o.image_url, o.image_credit,
  o.start_datetimes, o.has_end, o.end_datetimes, o.recurrence, o.end_recurrence, o.timezone,
  o.attraction_hours, o.cost, o.languages, o.is_online,
  o.location_type, o.location_name, o.location_point_geojson, o.location_polygon_geojson,
  o.address_street, o.address_city, o.address_state, o.address_country, o.address_zip,
  o.opp_hashtags, o.opp_social_handles, o.opp_partner,
  o.accepted, o.withdrawn, o.submitted_by, o.review_status,
  o.contact_name, o.contact_email, o.contact_phone, o.extra_data,
  ov.exterior as "overlay_exterior?",
  ov.interior as "overlay_interior?"
from c_opportunity o
left join c_opportunity_overlay ov on o.id = ov.opportunity_id
where lower(o.slug) = lower($1::text) limit 1;
