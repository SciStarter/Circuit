insert into c_involvement (opportunity, first, latest, mode, participant, "location")
values ($1, $2, $3, $4, $5, $6)
returning id;
