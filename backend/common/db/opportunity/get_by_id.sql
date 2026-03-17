select id, uid, slug, partner_name, partner_website, partner_logo_url,
  partner_created, partner_updated, partner_opp_url,
  organization_name, organization_type, organization_website, organization_logo_url,
  entity_type, opp_venue, opp_descriptor, min_age, max_age, pes_domain,
  tags, opp_topics, ticket_required,
  title, description, short_desc, image_url, image_credit,
  start_datetimes, has_end, end_datetimes, recurrence, end_recurrence, timezone,
  attraction_hours, cost, languages, is_online,
  location_type, location_name, location_point_geojson, location_polygon_geojson,
  address_street, address_city, address_state, address_country, address_zip,
  opp_hashtags, opp_social_handles, opp_partner,
  accepted, withdrawn, submitted_by, review_status,
  contact_name, contact_email, contact_phone, extra_data
from c_opportunity where id = $1 limit 1;
