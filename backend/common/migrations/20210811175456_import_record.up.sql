begin;

create table c_opportunity_import_record (
       "id" serial primary key,
       "when" timestamptz not null default NOW(),
       "partner" UUID not null,
       "opportunity" UUID not null,
       "created" boolean not null,
       "ignored" boolean not null
);

create index c_opportunity_import_record_by_partner
on c_opportunity_import_record (partner);

commit;
