update c_opportunity
set
  exterior = $2::jsonb,
  interior =
    case
      when ($3::jsonb -> 'accepted') = 'null'::jsonb
      then jsonb_set($3::jsonb, '{accepted}', c_opportunity.interior -> 'accepted')
      else $3::jsonb
    end
where id = $1;
