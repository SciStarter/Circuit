select count(*) saves
from c_involvement I inner join c_opportunity O
on I.opportunity = O.uid
where I.mode = 20 and O.slug = $1;
