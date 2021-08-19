select id, exterior, interior from c_opportunity where lower($1::text) = lower(exterior ->> 'slug') limit 1;
