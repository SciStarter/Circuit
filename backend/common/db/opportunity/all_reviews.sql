select R."id", R."person", R."rating", R."comment", R."when",
  case when P.uid is not null then coalesce(P.username, '') else '' end as "username",
  case when P.uid is not null then coalesce(P.person_image_url, '') else '' end as "image_url"
from c_opportunity_review R
inner join c_opportunity O on R.opportunity_id = O.id
left outer join c_person P on R.person = P.uid
where O.slug = $1 order by r."when" desc;
