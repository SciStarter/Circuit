select exists(select 1 from c_person where (interior -> 'email') ? $1) as "exists";
