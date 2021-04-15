select exists(select 1 from c_opportunity where exterior -> 'uid' = $1) as "exists";
