alter table c_opportunity_review drop constraint if exists c_opportunity_review_opportunity_id_fkey;

alter table c_opportunity_review add constraint c_opportunity_review_opportunity_id_fkey
  foreign key (opportunity_id)
  references c_opportunity(id);


alter table c_opportunity_like drop constraint if exists c_opportunity_like_opportunity_id_fkey;

alter table c_opportunity_like add constraint c_opportunity_like_opportunity_id_fkey
  foreign key (opportunity_id)
  references c_opportunity(id);



