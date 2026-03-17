insert into c_participation (opportunity, partner, "when", mode, keywords, participant, snml, "location")
values ($1, $2, $3, $4, $5, $6, $7, $8)
returning id;
