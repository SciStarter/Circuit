select R."person", R."rating", R."comment", R."when"
from c_opportunity_review R inner join c_opportunity O on R.opportunity_id = O.id
where (O.exterior ->> 'slug') = $1 order by r."when" desc;
