select exists(select 1 from c_person where email = $1) as "exists";
