begin;

alter table c_opportunity_review add column "id" serial;
alter table c_opportunity_review add column "reports" integer;

commit;
