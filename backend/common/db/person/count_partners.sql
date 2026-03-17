select count(*) as total
from c_partner
where (uid != $2) and (($1 = ANY(authorized)) or (prime = $1));
