create or replace function c_opportunity_search_delete() returns trigger as
$body$
begin
  delete from c_opportunity_search where opp_id = old.id;
  return old;
end;
$body$
language plpgsql;
