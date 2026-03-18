begin;

-- Add typed columns
alter table c_person
  add column uid uuid,
  add column username text,
  add column person_image_url text,
  add column email text not null default '',
  add column email_hashes text[] not null default '{}',
  add column "password" text,
  add column join_channel text not null default 'Local',
  add column join_channel_detail uuid,
  add column first_name text,
  add column last_name text,
  add column genders text[] not null default '{}',
  add column gender_other text,
  add column joined_at timestamptz not null default now(),
  add column active_at timestamptz not null default now(),
  add column phone text,
  add column whatsapp text,
  add column zip_code text,
  add column birth_year integer,
  add column ethnicities text[] not null default '{}',
  add column ethnicity_other text,
  add column family_income text,
  add column education_level text,
  add column opt_in_research boolean,
  add column opt_in_volunteer boolean,
  add column permissions text[] not null default '{}',
  add column "private" boolean not null default false,
  add column newsletter boolean not null default false,
  add column allow_emails boolean not null default true,
  add column recent_point jsonb,
  add column last_used_people_recruiter timestamptz,
  add column extra jsonb;

-- Backfill from JSONB
update c_person set
  uid = (exterior->>'uid')::uuid,
  username = exterior->>'username',
  person_image_url = exterior->>'image_url',
  email = coalesce(interior->>'email', ''),
  email_hashes = (select coalesce(array_agg(v::text), '{}') from jsonb_array_elements_text(coalesce(interior->'email_hashes', '[]'::jsonb)) v),
  "password" = interior->>'password',
  join_channel = case
    when jsonb_typeof(interior->'join_channel') = 'object' then
      (select key from jsonb_each(interior->'join_channel') limit 1)
    when jsonb_typeof(interior->'join_channel') = 'string' then
      interior->>'join_channel'
    else 'Local'
  end,
  join_channel_detail = case
    when jsonb_typeof(interior->'join_channel') = 'object' and (select key from jsonb_each(interior->'join_channel') limit 1) = 'Exchange' then
      (select value from jsonb_each_text(interior->'join_channel') limit 1)::uuid
    else null
  end,
  first_name = interior->>'first_name',
  last_name = interior->>'last_name',
  genders = (select coalesce(array_agg(v::text), '{}') from jsonb_array_elements_text(coalesce(interior->'genders', '[]'::jsonb)) v),
  gender_other = interior->>'gender_other',
  joined_at = coalesce((interior->>'joined_at')::timestamptz, created),
  active_at = coalesce((interior->>'active_at')::timestamptz, created),
  phone = interior->>'phone',
  whatsapp = interior->>'whatsapp',
  zip_code = interior->>'zip_code',
  birth_year = (interior->>'birth_year')::integer,
  ethnicities = (select coalesce(array_agg(v::text), '{}') from jsonb_array_elements_text(coalesce(interior->'ethnicities', '[]'::jsonb)) v),
  ethnicity_other = interior->>'ethnicity_other',
  family_income = case
    when interior->>'family_income' is not null and interior->>'family_income' != 'null' then interior->>'family_income'
    else null
  end,
  education_level = case
    when interior->>'education_level' is not null and interior->>'education_level' != 'null' then interior->>'education_level'
    else null
  end,
  opt_in_research = (interior->>'opt_in_research')::boolean,
  opt_in_volunteer = (interior->>'opt_in_volunteer')::boolean,
  permissions = (select coalesce(array_agg(v::text), '{}') from jsonb_array_elements_text(coalesce(interior->'permissions', '[]'::jsonb)) v),
  "private" = coalesce((interior->>'private')::boolean, false),
  newsletter = coalesce((interior->>'newsletter')::boolean, false),
  allow_emails = coalesce((interior->>'allow_emails')::boolean, true),
  recent_point = interior->'recent_point',
  last_used_people_recruiter = (interior->>'last_used_people_recruiter')::timestamptz,
  extra = interior->'extra';

-- Add NOT NULL constraint on uid
alter table c_person
  alter column uid set not null;

-- Replace JSONB expression indexes
drop index if exists c_person_uid;
drop index if exists c_person_email;
drop index if exists c_person_email_hashes;

create unique index c_person_uid on c_person (uid);
create index c_person_email on c_person (email);
create index c_person_email_hashes on c_person using GIN (email_hashes);

-- Drop old JSONB columns
alter table c_person
  drop column exterior cascade,
  drop column interior cascade;

commit;
