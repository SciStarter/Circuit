update c_participation
set opportunity = $2, partner = $3, "when" = $4, mode = $5, keywords = $6,
    participant = $7, snml = $8, "location" = $9
where id = $1;
