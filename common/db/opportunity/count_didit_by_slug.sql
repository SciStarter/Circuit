select count(*) didit
from c_involvement I inner join c_opportunity O
on (I.exterior -> 'opportunity') @> (O.exterior -> 'uid')
where (I.exterior -> 'mode')::integer >= 2 and (O.exterior ->> 'slug') = $1;
