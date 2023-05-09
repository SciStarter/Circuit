select
  id,
  (c_opportunity.exterior || coalesce(c_opportunity_overlay.exterior, '{}'::jsonb)) as "exterior!",
  (c_opportunity.interior || coalesce(c_opportunity_overlay.interior, '{}'::jsonb)) as "interior!"
from
  c_opportunity left join c_opportunity_overlay
  on c_opportunity.id = c_opportunity_overlay.opportunity_id
where ($1::jsonb) @> (c_opportunity.exterior -> 'uid') limit 1;
