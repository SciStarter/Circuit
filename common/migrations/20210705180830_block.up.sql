begin;

create table c_block (
       id serial primary key,
       created timestamptz not null default NOW(),
       updated timestamptz not null default NOW(),
       "language" varchar(8) not null default 'en',
       "group" text not null,
       item text not null,
       content text not null
);

create trigger c_set_block_updated before update on c_block for each row execute procedure set_updated();

create index c_block_language on c_block ("language");
create index c_block_group on c_block ("group");
create index c_block_item on c_block (item);

create unique index c_block_identity on c_block ("language", "group", item);

commit;
