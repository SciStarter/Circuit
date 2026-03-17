select *
from
  (
    ( -- Count each "I did this" in the timeframe as one
      select
        I.opportunity as "opportunity!",
        I.latest as "when!"
      from c_involvement I
      where
        I.participant = $1::uuid
      and
        I.mode = 30
      and
        I.latest >= $2::timestamptz
      and
        I.latest <= $3::timestamptz
    )
  union
    ( -- Count each partner-reported contribution in the timeframe as one
      select
        P.opportunity as "opportunity!",
        P."when" as "when!"
      from c_participation P
      where
        P.participant = $1::uuid
      and
        P."when" >= $2::timestamptz
      and
        P."when" <= $3::timestamptz
    )
  ) as "merged"
order by "when!" asc
;
