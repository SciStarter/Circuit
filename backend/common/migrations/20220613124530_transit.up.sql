begin;

create table c_transit (
       "id" serial primary key,
       "created" timestamptz not null default NOW(),
       "prior" UUID not null,
       "postor" UUID not null,
       "actor" UUID
);

create index c_transit_by_prior on c_transit("prior");
create index c_transit_by_postor on c_transit("postor");
create index c_transit_by_actor on c_transit("actor");

commit;
