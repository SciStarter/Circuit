select
  opp.uid as uid,
  opp.slug as slug,
  opp.title as title,
  opp.image_url as image_url,
  opp.short_desc as short_desc
from c_opportunity as opp inner join c_person_bookmark as mark on
  opp.uid = mark.opportunity
where
  mark.person = $1::uuid;
