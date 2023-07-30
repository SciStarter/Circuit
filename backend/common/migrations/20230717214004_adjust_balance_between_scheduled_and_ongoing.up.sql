create or replace function c_opportunity_is_scheduled(interior jsonb, exterior jsonb) returns boolean as
$func$
BEGIN
 RETURN (
   jsonb_array_length(exterior -> 'start_datetimes') > 1
   OR
   jsonb_array_length(exterior -> 'end_datetimes') > 1
   OR (
     jsonb_array_length(exterior -> 'start_datetimes') = 1
     AND
     jsonb_array_length(exterior -> 'end_datetimes') = 1
     AND
     (exterior #> '{start_datetimes,0}')::text::date = (exterior #> '{end_datetimes,0}')::text::date
   )
 );
END
$func$ language plpgsql stable;

create or replace function c_opportunity_is_ondemand(interior jsonb, exterior jsonb) returns boolean as
$func$
BEGIN
 RETURN (
   jsonb_array_length(exterior -> 'start_datetimes') <= 1
   AND
   jsonb_array_length(exterior -> 'end_datetimes') <= 1
   AND (
     jsonb_array_length(exterior -> 'start_datetimes') != 1
     OR
     jsonb_array_length(exterior -> 'end_datetimes') != 1
     OR
     (exterior #> '{start_datetimes,0}')::text::date != (exterior #> '{end_datetimes,0}')::text::date
   )
 );
END
$func$ language plpgsql stable;

