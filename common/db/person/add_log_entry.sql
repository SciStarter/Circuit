insert
into c_person_log ("person_id", "event")
values ($1, $2::jsonb)
returning "id";
