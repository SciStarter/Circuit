select id, exterior, interior from c_opportunity where exterior -> 'uid' = $1;
