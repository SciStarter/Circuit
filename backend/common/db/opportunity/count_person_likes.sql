select count(*) likes
from c_opportunity_like L inner join c_opportunity O on L.opportunity_id = O.id
where (O.exterior ->> 'slug') = $1 and L.person = $2;
