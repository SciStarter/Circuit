select count(*) as total
from c_partner
where ((interior -> 'authorized') @> ($1::jsonb)) or ((interior -> 'prime') @> ($1::jsonb));
