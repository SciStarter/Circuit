INSERT INTO c_block ("language", "group", item, tags, label, content) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id;
