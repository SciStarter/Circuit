update c_opportunity set
  uid = $2, slug = $3, partner_name = $4, partner_website = $5, partner_logo_url = $6,
  partner_created = $7, partner_updated = $8, partner_opp_url = $9,
  organization_name = $10, organization_type = $11, organization_website = $12, organization_logo_url = $13,
  entity_type = $14, opp_venue = $15, opp_descriptor = $16, min_age = $17, max_age = $18, pes_domain = $19,
  tags = $20, opp_topics = $21, ticket_required = $22,
  title = $23, description = $24, short_desc = $25, image_url = $26, image_credit = $27,
  start_datetimes = $28, has_end = $29, end_datetimes = $30, recurrence = $31, end_recurrence = $32, timezone = $33,
  attraction_hours = $34, cost = $35, languages = $36, is_online = $37,
  location_type = $38, location_name = $39, location_point_geojson = $40, location_polygon_geojson = $41,
  address_street = $42, address_city = $43, address_state = $44, address_country = $45, address_zip = $46,
  opp_hashtags = $47, opp_social_handles = $48, opp_partner = $49,
  accepted = coalesce($50, c_opportunity.accepted), withdrawn = $51, submitted_by = $52, review_status = $53,
  contact_name = $54, contact_email = $55, contact_phone = $56, extra_data = $57
where id = $1;
