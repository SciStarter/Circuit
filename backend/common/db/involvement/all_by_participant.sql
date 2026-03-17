select id, opportunity, first, latest, mode, participant, "location"
from c_involvement
where
  participant = $1
and
  case
    when $2::integer is null then mode >= 1
    else mode >= greatest($2::integer, 1)
  end
and
  case
    when $3::integer is null then true
    else mode <= $3::integer
  end
order by updated desc
limit $4 offset $5;
