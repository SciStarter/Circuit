select exists(select 1 from c_partner where exterior -> 'uid' = $1) as "exists";
