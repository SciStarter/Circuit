select id, exterior, interior from c_involvement where ($1::jsonb) @> (interior -> 'participant');
