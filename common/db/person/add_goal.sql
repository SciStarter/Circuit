insert
into c_person_goals (
    "person_id",
    "category",
    "target",
    "begin",
    "end",
    "status"
  )
values ($1, $2, $3, $4, $5, $6::c_person_goals_status)
returning "id"
;
