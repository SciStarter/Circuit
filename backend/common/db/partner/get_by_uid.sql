select id, exterior, interior from c_partner where ($1::jsonb) @> (exterior -> 'uid') limit 1;
