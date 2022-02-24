select id, exterior, interior
from c_partner
where (exterior -> 'uid' != $2::jsonb) and (((interior -> 'authorized') @> ($1::jsonb)) or ((interior -> 'prime') @> ($1::jsonb)));
