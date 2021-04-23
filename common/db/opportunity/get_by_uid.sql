select id, exterior, interior from c_opportunity where ($1::jsonb) @> (exterior -> 'uid') limit 1;
