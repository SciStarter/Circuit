begin;

-- Add typed columns
alter table c_partner
  add column uid uuid,
  add column "name" text not null default '',
  add column organization_type text not null default 'unspecified',
  add column pes_domain text not null default 'unspecified',
  add column url text,
  add column image_url text,
  add column description text not null default '',
  add column background_color text,
  add column primary_color text,
  add column secondary_color text,
  add column tertiary_color text,
  add column under uuid,
  add column open_submission boolean,
  add column default_query text,
  add column manager jsonb not null default '{}',
  add column contact jsonb,
  add column prime uuid,
  add column authorized uuid[] not null default '{}',
  add column pending uuid[] not null default '{}',
  add column secret text;

-- Backfill from JSONB
update c_partner set
  uid = (exterior->>'uid')::uuid,
  "name" = coalesce(exterior->>'name', ''),
  organization_type = coalesce(exterior->>'organization_type', 'unspecified'),
  pes_domain = coalesce(exterior->>'pes_domain', 'unspecified'),
  url = exterior->>'url',
  image_url = exterior->>'image_url',
  description = coalesce(exterior->>'description', ''),
  background_color = exterior->>'background_color',
  primary_color = exterior->>'primary_color',
  secondary_color = exterior->>'secondary_color',
  tertiary_color = exterior->>'tertiary_color',
  under = (exterior->>'under')::uuid,
  open_submission = (exterior->>'open_submission')::boolean,
  default_query = exterior->>'default_query',
  manager = coalesce(interior->'manager', '{}'::jsonb),
  contact = interior->'contact',
  prime = (interior->>'prime')::uuid,
  authorized = (select coalesce(array_agg(v::uuid), '{}') from jsonb_array_elements_text(coalesce(interior->'authorized', '[]'::jsonb)) v),
  pending = (select coalesce(array_agg(v::uuid), '{}') from jsonb_array_elements_text(coalesce(interior->'pending', '[]'::jsonb)) v),
  secret = interior->>'secret';

-- Add NOT NULL constraints
alter table c_partner
  alter column uid set not null,
  alter column prime set not null;

-- Replace JSONB expression indexes
drop index if exists c_partner_uid;
drop index if exists c_partner_prime;
drop index if exists c_partner_authorized;

create unique index c_partner_uid on c_partner (uid);
create index c_partner_prime on c_partner (prime);
create index c_partner_authorized on c_partner using GIN (authorized);

-- Drop the view that depends on partner exterior before dropping columns
drop view if exists c_partner_stats;

-- Drop old JSONB columns (CASCADE to drop any remaining dependent objects)
alter table c_partner
  drop column exterior cascade,
  drop column interior cascade;

-- Recreate the view using typed partner columns (opportunity still uses JSONB)
create view c_partner_stats
as select
  count(c_opportunity.*) as "total",
  min(c_partner.uid::text) as "uid",
  min(c_partner."name") as "name"
  from c_opportunity inner join c_partner
  on c_opportunity.exterior -> 'partner' = to_jsonb(c_partner.uid)
  where (EXISTS
         (SELECT value
          FROM jsonb_array_elements_text(c_opportunity.exterior -> 'start_datetimes')
          WHERE value::timestamptz > CURRENT_TIMESTAMP
         )
         OR
         EXISTS
          (SELECT value
           FROM jsonb_array_elements_text(c_opportunity.exterior -> 'end_datetimes')
           WHERE value::timestamptz > CURRENT_TIMESTAMP
          )
         OR
          (
           jsonb_array_length(c_opportunity.exterior -> 'start_datetimes') = 0
           AND
           jsonb_array_length(c_opportunity.exterior -> 'end_datetimes') = 0
          )
        )
        AND
        c_opportunity.interior ->> 'accepted' = 'true'
        AND
        c_opportunity.interior ->> 'withdrawn' = 'false'
        GROUP BY c_opportunity.exterior -> 'partner';

commit;
