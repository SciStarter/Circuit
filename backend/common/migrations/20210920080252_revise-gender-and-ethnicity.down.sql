update c_person set interior = jsonb_set(jsonb_set(interior,
'{ethnicity}', '"unspecified"'::jsonb) - 'ethnicities',
'{gender}', '"unspecified"'::jsonb) - 'genders';

