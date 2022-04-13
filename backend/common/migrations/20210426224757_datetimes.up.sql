update c_opportunity
set exterior = exterior - 'start_dates' ||
               jsonb_build_object('start_datetimes', exterior->'start_dates')
where exterior ? 'start_dates';

update c_opportunity
set exterior = exterior - 'end_dates' ||
               jsonb_build_object('end_datetimes', exterior->'end_dates')
where exterior ? 'end_dates';
