select
  "id",
  "person_id",
  "category",
  "target",
  "begin",
  "end",
  "status" as "status: GoalStatus"
from
  c_person_goals
where
    "person_id" = $1
  and
    "status" = $2::c_person_goals_status
order by "end" asc
;
