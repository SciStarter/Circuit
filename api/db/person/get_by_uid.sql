select id, exterior, interior from c_person where exterior -> 'uid' = $1 limit 1;
