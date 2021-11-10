begin;

create or replace function c_opportunity_is_current(interior jsonb, exterior jsonb) returns boolean as
$func$
BEGIN
  RETURN (
    interior ->> 'accepted' = 'true'
    AND
    interior ->> 'withdrawn' = 'false'
    AND
    (
      EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'start_datetimes') WHERE value::timestamptz > CURRENT_TIMESTAMP)
      OR
      EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'end_datetimes') WHERE value::timestamptz > CURRENT_TIMESTAMP)
      OR
      (
        jsonb_array_length(exterior -> 'start_datetimes') = 0
        AND
        jsonb_array_length(exterior -> 'end_datetimes') = 0
      )
    )
  );
END
$func$ language plpgsql immutable;


alter table c_opportunity
add column "current" boolean
generated always as (c_opportunity_is_current(interior, exterior)) stored;

create view c_partner_stats
as select
  count(c_opportunity.*) as "total",
  min(c_partner.exterior->>'uid') as "uid",
  min(c_partner.exterior->>'name') as "name"
  from c_opportunity inner join c_partner
  on c_opportunity.exterior -> 'partner' = c_partner.exterior->'uid'
  where (EXISTS
         (SELECT value
          FROM jsonb_array_elements_text(c_opportunity.exterior -> 'start_datetimes')
          WHERE value::timestamptz > CURRENT_TIMESTAMP
         )
         OR
         EXISTS
          (SELECT value
           FROM jsonb_array_elements_text(c_opportunity.exterior -> 'end_datetimes')
           WHERE value::timestamptz > CURRENT_TIMESTAMP
          )
         OR
          (
           jsonb_array_length(c_opportunity.exterior -> 'start_datetimes') = 0
           AND
           jsonb_array_length(c_opportunity.exterior -> 'end_datetimes') = 0
          )
        )
        AND
        c_opportunity.interior ->> 'accepted' = 'true'
        AND
        c_opportunity.interior ->> 'withdrawn' = 'false'
        GROUP BY c_opportunity.exterior -> 'partner';

commit;
