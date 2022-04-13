select exists(select 1 from c_partner where ($1::jsonb) @> (exterior -> 'uid')) as "exists";
