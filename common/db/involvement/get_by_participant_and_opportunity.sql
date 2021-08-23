select id, exterior, interior from c_involvement where ($1::jsonb) @> (interior -> 'participant') and ($2::jsonb) @> (exterior -> 'opportunity') limit 1;
