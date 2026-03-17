update c_involvement
set opportunity = $2, first = $3, latest = $4, mode = $5, participant = $6, "location" = $7
where id = $1;
