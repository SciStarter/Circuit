select id, (exterior -> 'uid') as "uid", (exterior -> 'name') as "name" from c_partner;
