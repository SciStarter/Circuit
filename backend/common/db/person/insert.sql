insert into c_person (
  uid, username, person_image_url, email, email_hashes, "password",
  join_channel, join_channel_detail, first_name, last_name, genders, gender_other,
  joined_at, active_at, phone, whatsapp, zip_code, birth_year, ethnicities,
  ethnicity_other, family_income, education_level, opt_in_research, opt_in_volunteer,
  permissions, "private", newsletter, allow_emails, recent_point,
  last_used_people_recruiter, extra
) values (
  $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,
  $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24,
  $25, $26, $27, $28, $29, $30, $31
) returning id;
