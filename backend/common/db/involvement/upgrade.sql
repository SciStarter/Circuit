insert into c_involvement (exterior, interior)
values (
  jsonb_build_object('opportunity', $2::jsonb, 'first', to_jsonb(now()), 'latest', to_jsonb(now()), 'mode', $3::jsonb),
  jsonb_build_object('participant', $1::jsonb, 'location', $4::jsonb)
)
on conflict ((exterior -> 'opportunity'), (interior -> 'participant')) do
update set
  exterior = jsonb_set(
    jsonb_set(c_involvement.exterior, '{latest}', to_jsonb(now())),
    '{mode}',
    greatest((c_involvement.exterior -> 'mode'), $3::jsonb)
  ),
  interior = case when ($4::jsonb = 'null'::jsonb) then c_involvement.interior else jsonb_set(c_involvement.interior, '{location}', $4::jsonb) end
;
