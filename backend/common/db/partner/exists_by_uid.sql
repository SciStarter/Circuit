select exists(select 1 from c_partner where uid = $1) as "exists";
