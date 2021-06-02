select id, exterior, interior from c_person where (interior -> 'email_hashes') ? $1;
