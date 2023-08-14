create sequence c_person_searches_id_seq;
alter table c_person_searches alter column "id" set default nextval('c_person_searches_id_seq');
alter sequence c_person_searches_id_seq owned by c_person_searches."id";
