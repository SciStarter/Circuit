begin;

alter table c_block add column tags text not null default '';
alter table c_block add column label text not null default '';

commit;
