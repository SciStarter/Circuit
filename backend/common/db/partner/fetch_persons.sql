select id, exterior, interior from c_person where ($1::jsonb) @> (exterior -> 'uid');
