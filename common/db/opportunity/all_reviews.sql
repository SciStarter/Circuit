select R."id", R."person", R."rating", R."comment", R."when",
  case when P."exterior" is not null then (P."exterior" ->> 'username') else '' end as "username",
  case when P."exterior" is not null then (P."exterior" ->> 'image_url') else '' end as "image_url"
from c_opportunity_review R
inner join c_opportunity O on R.opportunity_id = O.id
left outer join c_person P on R.person = (P.exterior ->> 'uid')::uuid
where (O.exterior ->> 'slug') = $1 order by r."when" desc;
