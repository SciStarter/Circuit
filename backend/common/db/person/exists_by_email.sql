select exists(select 1 from c_person where ($1::jsonb) @> (interior -> 'email')) as "exists";
