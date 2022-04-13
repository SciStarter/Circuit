insert
into c_opportunity (exterior, interior)
values (
  $1::jsonb,
  case
    when ($2::jsonb -> 'accepted') = 'null'::jsonb
    then jsonb_set($2::jsonb, '{accepted}', 'false'::jsonb)
    else $2::jsonb
  end
) returning id;
