select id, exterior, interior from c_partner where exterior -> 'uid' = $1 limit 1;
