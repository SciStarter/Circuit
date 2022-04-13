update c_person set interior = jsonb_set(jsonb_set(interior,
'{ethnicities}', '[]'::jsonb) - 'ethnicity',
'{genders}', '[]'::jsonb) - 'gender';
