select
  c_partner.id as "id",
  (c_partner.exterior -> 'uid') as "uid",
  c_partner.exterior->>'name' as "name",
  (c_partner.interior -> 'manager' ->> 'name') as "manager_name",
  (c_partner.interior -> 'manager' ->> 'email') as "manager_email",
  c_partner."created" as "joined",
  count(distinct c_opportunity.id) as "accepted"
from c_partner join c_opportunity
  on c_partner.exterior->'uid' = c_opportunity.exterior->'partner'
 and c_opportunity.interior->>'accepted' != 'false'
group by
  c_partner.id,
  c_partner.exterior -> 'uid',
  c_partner.exterior->>'name',
  c_partner.interior -> 'manager' ->> 'name',
  c_partner.interior -> 'manager' ->> 'email',
  c_partner."created"
order by c_partner.exterior->>'name' asc;
