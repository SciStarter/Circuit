select exists(select 1 from c_opportunity where lower(slug) = lower($1::text)) as "exists";
