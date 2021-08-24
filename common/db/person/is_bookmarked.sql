select exists (select 1 from c_person_bookmark where person = $1 and opportunity = $2) as bookmarked;
