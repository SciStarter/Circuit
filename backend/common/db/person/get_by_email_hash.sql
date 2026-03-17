select id, uid, username, person_image_url, email, email_hashes, "password",
  join_channel, join_channel_detail, first_name, last_name, genders, gender_other,
  joined_at, active_at, phone, whatsapp, zip_code, birth_year, ethnicities,
  ethnicity_other, family_income, education_level, opt_in_research, opt_in_volunteer,
  permissions, "private", newsletter, allow_emails, recent_point,
  last_used_people_recruiter, extra
from c_person where $1 = ANY(email_hashes) order by id limit 1;
