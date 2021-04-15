select exists(select 1 from c_person where exterior -> 'uid' = $1) as "exists";
