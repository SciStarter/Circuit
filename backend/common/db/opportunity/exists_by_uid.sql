select exists(select 1 from c_opportunity where uid = $1) as "exists";
