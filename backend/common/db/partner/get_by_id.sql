select id, uid, "name", organization_type, pes_domain, url, image_url,
  description, background_color, primary_color, secondary_color, tertiary_color,
  under, open_submission, default_query,
  manager, contact, prime, authorized, pending, secret
from c_partner where id = $1 limit 1;
