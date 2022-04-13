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
    "id" = $2
;
