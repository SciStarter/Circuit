create or replace function c_opportunity_is_current_as_of(interior jsonb, exterior jsonb, stamp timestamptz) returns boolean as
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
       (
          jsonb_array_length(exterior -> 'start_datetimes') <= 1
          AND
          jsonb_array_length(exterior -> 'end_datetimes') = 0
       )
       OR
       EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'start_datetimes') WHERE value::timestamptz > stamp)
       OR
       EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'end_datetimes') WHERE value::timestamptz > stamp)
       OR
       (
          (exterior->>'recurrence' = 'daily' OR exterior->>'recurrence' = 'weekly')
          AND
          (exterior->>'end_recurrence' IS null OR (exterior->>'end_recurrence')::timestamptz > stamp)
       )
   )
 );
END
$func$ language plpgsql stable;
