insert into c_partner (
  uid, "name", organization_type, pes_domain, url, image_url,
  description, background_color, primary_color, secondary_color, tertiary_color,
  under, open_submission, default_query,
  manager, contact, prime, authorized, pending, secret
) values (
  $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20
) returning id;
