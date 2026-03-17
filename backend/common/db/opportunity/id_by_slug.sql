select id from c_opportunity where lower(slug) = lower($1::text) limit 1;
