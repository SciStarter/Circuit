select exists(select 1 from c_opportunity where ($1::jsonb) @> (exterior -> 'uid')) as "exists";
