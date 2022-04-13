select id, exterior, interior
from c_involvement
where
  ($1::jsonb) @> (interior -> 'participant')
and
  case
    when $2::integer is null then (exterior ->> 'mode')::integer >= 1
    else (exterior ->> 'mode')::integer >= greatest($2::integer, 1)
  end
and
  case
    when $3::integer is null then true
    else (exterior ->> 'mode')::integer <= $3::integer
  end
order by updated desc
limit $4 offset $5;
