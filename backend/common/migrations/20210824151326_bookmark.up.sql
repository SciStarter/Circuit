begin;

create table c_person_bookmark (person UUID, opportunity UUID, saved timestamptz);
create index c_person_bookmark_by_person on c_person_bookmark (person);
create index c_person_bookmark_by_opportunity on c_person_bookmark (opportunity);
create unique index c_person_bookmark_unique on c_person_bookmark (person, opportunity);

commit;
