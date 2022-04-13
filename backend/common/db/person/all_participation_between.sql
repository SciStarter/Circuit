select *
from
  (
    ( -- Count each "I did this" in the timeframe as one
      select
        (I.exterior ->> 'opportunity')::uuid as "opportunity!",
        (I.exterior ->> 'latest')::timestamptz as "when!"
      from c_involvement I
      where
        (I.interior ->> 'participant') = $1::text
      and
        (I.exterior -> 'mode') = '30'::jsonb
      and
        (I.exterior ->> 'latest') >= $2::text
      and
        (I.exterior ->> 'latest') <= $3::text
    )
  union
    ( -- Count each partner-reported contribution in the timeframe as one
      select
        (P.exterior ->> 'opportunity')::uuid as "opportunity!",
        (P.exterior ->> 'when')::timestamptz as "when!"
      from c_participation P
      where
        (P.interior ->> 'participant') = $1::text
      and
        (P.exterior ->> 'when') >= $2::text
      and
        (P.exterior ->> 'when') <= $3::text
    )
  ) as "merged"
order by "when!" asc
;
