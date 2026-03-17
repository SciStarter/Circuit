begin;

alter table c_person
  add column exterior jsonb,
  add column interior jsonb;

update c_person set
  exterior = jsonb_build_object(
    'uid', uid,
    'username', username,
    'image_url', person_image_url
  ),
  interior = (
    jsonb_build_object(
      'email', email,
      'email_hashes', to_jsonb(email_hashes),
      'password', "password",
      'join_channel', case
        when join_channel = 'Exchange' and join_channel_detail is not null then
          jsonb_build_object('Exchange', join_channel_detail)
        else to_jsonb(join_channel)
      end,
      'first_name', first_name,
      'last_name', last_name,
      'genders', to_jsonb(genders),
      'gender_other', gender_other,
      'joined_at', joined_at,
      'active_at', active_at,
      'phone', phone,
      'whatsapp', whatsapp,
      'zip_code', zip_code,
      'birth_year', birth_year,
      'ethnicities', to_jsonb(ethnicities),
      'ethnicity_other', ethnicity_other,
      'family_income', family_income,
      'education_level', education_level,
      'opt_in_research', opt_in_research,
      'opt_in_volunteer', opt_in_volunteer,
      'permissions', to_jsonb(permissions),
      'private', "private",
      'newsletter', newsletter,
      'allow_emails', allow_emails,
      'recent_point', recent_point,
      'last_used_people_recruiter', last_used_people_recruiter,
      'extra', extra
    )
  );

alter table c_person
  alter column exterior set not null,
  alter column interior set not null;

drop index if exists c_person_uid;
drop index if exists c_person_email;
drop index if exists c_person_email_hashes;

create index c_person_uid on c_person using GIN ((exterior -> 'uid'));
create index c_person_email on c_person using GIN ((interior -> 'email'));
create index c_person_email_hashes on c_person using GIN ((interior -> 'email_hashes'));

alter table c_person
  drop column uid,
  drop column username,
  drop column person_image_url,
  drop column email,
  drop column email_hashes,
  drop column "password",
  drop column join_channel,
  drop column join_channel_detail,
  drop column first_name,
  drop column last_name,
  drop column genders,
  drop column gender_other,
  drop column joined_at,
  drop column active_at,
  drop column phone,
  drop column whatsapp,
  drop column zip_code,
  drop column birth_year,
  drop column ethnicities,
  drop column ethnicity_other,
  drop column family_income,
  drop column education_level,
  drop column opt_in_research,
  drop column opt_in_volunteer,
  drop column permissions,
  drop column "private",
  drop column newsletter,
  drop column allow_emails,
  drop column recent_point,
  drop column last_used_people_recruiter,
  drop column extra;

commit;
