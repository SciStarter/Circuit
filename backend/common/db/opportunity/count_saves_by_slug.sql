select count(*) saves
from c_involvement I inner join c_opportunity O
on (I.exterior -> 'opportunity') @> (O.exterior -> 'uid')
where (I.exterior -> 'mode')::integer = 20 and (O.exterior ->> 'slug') = $1;
