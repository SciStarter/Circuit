insert into c_person_bookmark (person, opportunity, saved)
values ($1, $2, now())
on conflict do nothing;
