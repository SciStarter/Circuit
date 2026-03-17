select exists(select 1 from c_person where $1 = ANY(email_hashes)) as "exists";
