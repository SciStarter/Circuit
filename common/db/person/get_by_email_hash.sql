select id, exterior, interior from c_person where (interior -> 'email_hashes') ? $1 order by id limit 1;
