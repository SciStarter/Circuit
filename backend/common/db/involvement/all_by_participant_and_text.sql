select I.id, I.opportunity, I.first, I.latest, I.mode, I.participant, I."location"
from c_involvement as I
left join c_opportunity as O
on I.opportunity = O.uid
left join c_opportunity_search as S
on O.uid = S.uid
where
  I.participant = $1
and
  S.fulltext_english @@ websearch_to_tsquery($4)
and
  case
    when $2::integer is null then I.mode >= 1
    else I.mode >= greatest($2::integer, 1)
  end
and
  case
    when $3::integer is null then true
    else I.mode <= $3::integer
  end
order by I.updated desc
limit $5 offset $6;
