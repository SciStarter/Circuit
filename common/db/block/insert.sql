INSERT INTO c_block ("language", "group", item, content) VALUES ($1, $2, $3, $4) RETURNING id;
