insert
into c_opportunity_like (opportunity_id, person)
values ($1, $2)
on conflict do nothing;
