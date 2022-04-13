select
  (opp.exterior ->> 'uid')::uuid as uid,
  (opp.exterior ->> 'slug') as slug,
  (opp.exterior ->> 'title') as title,
  (opp.exterior ->> 'image_url') as image_url,
  (opp.exterior ->> 'short_desc') as short_desc
from c_opportunity as opp inner join c_person_bookmark as mark on
  (opp.exterior -> 'uid') @> to_jsonb(mark.opportunity)
where
  mark.person = $1::uuid and opp.exterior ? 'uid' and opp.exterior ? 'slug';
