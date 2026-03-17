update c_partner set
  uid = $2, "name" = $3, organization_type = $4, pes_domain = $5, url = $6, image_url = $7,
  description = $8, background_color = $9, primary_color = $10, secondary_color = $11, tertiary_color = $12,
  under = $13, open_submission = $14, default_query = $15,
  manager = $16, contact = $17, prime = $18, authorized = $19, pending = $20, secret = $21
where id = $1;
