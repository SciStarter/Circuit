insert into c_involvement (opportunity, first, latest, mode, participant, "location")
values ($2, now(), now(), $3, $1, $4)
on conflict (participant, opportunity) do
update set
  latest = now(),
  mode = greatest(c_involvement.mode, $3),
  "location" = case when $4::jsonb is null then c_involvement."location" else $4::jsonb end
;
