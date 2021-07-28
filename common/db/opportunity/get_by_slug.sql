select id, exterior, interior from c_opportunity where ($1::text) = (exterior ->> 'slug') limit 1;
