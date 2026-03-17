update c_person set
  uid = $2, username = $3, person_image_url = $4, email = $5, email_hashes = $6,
  "password" = $7, join_channel = $8, join_channel_detail = $9,
  first_name = $10, last_name = $11, genders = $12, gender_other = $13,
  joined_at = $14, active_at = $15, phone = $16, whatsapp = $17,
  zip_code = $18, birth_year = $19, ethnicities = $20, ethnicity_other = $21,
  family_income = $22, education_level = $23, opt_in_research = $24,
  opt_in_volunteer = $25, permissions = $26, "private" = $27,
  newsletter = $28, allow_emails = $29, recent_point = $30,
  last_used_people_recruiter = $31, extra = $32
where id = $1;
