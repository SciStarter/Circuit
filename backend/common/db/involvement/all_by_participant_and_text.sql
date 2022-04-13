select I.id, I.exterior, I.interior
from c_involvement as I
left join c_opportunity as O
on (I.exterior -> 'opportunity') = (O.exterior -> 'uid')
where
  ($1::jsonb) @> (I.interior -> 'participant')
and
  O.fulltext_english @@ websearch_to_tsquery($4)
and
  case
    when $2::integer is null then (I.exterior ->> 'mode')::integer >= 1
    else (I.exterior ->> 'mode')::integer >= greatest($2::integer, 1)
  end
and
  case
    when $3::integer is null then true
    else (I.exterior ->> 'mode')::integer <= $3::integer
  end
order by I.updated desc
limit $5 offset $6;
