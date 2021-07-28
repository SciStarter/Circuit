select exists(select 1 from c_opportunity where ($1::text) = (exterior ->> 'slug')) as "exists";
