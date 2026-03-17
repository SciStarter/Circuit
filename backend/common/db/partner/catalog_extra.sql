select
  c_partner.id as "id",
  c_partner.uid as "uid!",
  c_partner."name" as "name!",
  (c_partner.manager ->> 'name') as "manager_name",
  (c_partner.manager ->> 'email') as "manager_email",
  c_partner."created" as "joined",
  count(distinct c_opportunity.id) as "published"
from c_partner left join c_opportunity
  on c_partner.uid = c_opportunity.opp_partner
where c_opportunity.accepted != false
  and c_opportunity.withdrawn != true
group by
  c_partner.id,
  c_partner.uid,
  c_partner."name",
  c_partner.manager ->> 'name',
  c_partner.manager ->> 'email',
  c_partner."created"
order by c_partner."name" asc;
