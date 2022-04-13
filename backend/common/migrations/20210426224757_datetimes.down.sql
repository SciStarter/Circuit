update c_opportunity
set exterior = exterior - 'start_datetimes' ||
               jsonb_build_object('start_dates', exterior->'start_datetimes')
where exterior ? 'start_datetimes';

update c_opportunity
set exterior = exterior - 'end_datetimes' ||
               jsonb_build_object('end_dates', exterior->'end_datetimes')
where exterior ? 'end_datetimes';

