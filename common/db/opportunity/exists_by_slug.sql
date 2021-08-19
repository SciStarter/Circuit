select exists(select 1 from c_opportunity where lower($1::text) = lower(exterior ->> 'slug')) as "exists";
