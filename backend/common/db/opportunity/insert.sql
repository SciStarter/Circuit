insert into c_opportunity (
  uid, slug, partner_name, partner_website, partner_logo_url,
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
) values (
  $1, $2, $3, $4, $5,
  $6, $7, $8,
  $9, $10, $11, $12,
  $13, $14, $15, $16, $17, $18,
  $19, $20, $21,
  $22, $23, $24, $25, $26,
  $27, $28, $29, $30, $31, $32,
  $33, $34, $35, $36,
  $37, $38, $39, $40,
  $41, $42, $43, $44, $45,
  $46, $47, $48,
  coalesce($49, false), $50, $51, $52,
  $53, $54, $55, $56
) returning id;
