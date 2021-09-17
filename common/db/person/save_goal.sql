update c_person_goals
set
    "category" = $3,
    "target" = $4,
    "begin" = $5,
    "end" = $6,
    "status" = $7::c_person_goals_status
where
    "id" = $1
and
    "person_id" = $2
;

