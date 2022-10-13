begin;

create or replace function c_opportunity_is_current(interior jsonb, exterior jsonb) returns boolean as
$func$
BEGIN
 RETURN (
   coalesce(nullif(interior ->> 'review_status', ''), 'not_required') IN ('publish', 'not_required')
   AND
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

commit;
