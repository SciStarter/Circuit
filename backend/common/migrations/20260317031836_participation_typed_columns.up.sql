begin;

-- Add typed columns alongside existing JSONB columns
alter table c_participation
  add column opportunity uuid,
  add column partner uuid not null default '00000000-0000-0000-0000-000000000000',
  add column "when" timestamptz,
  add column mode text not null default 'signup',
  add column keywords text[] not null default '{}',
  add column participant uuid,
  add column snml text,
  add column "location" jsonb;

-- Backfill from JSONB
update c_participation set
  opportunity = (exterior->>'opportunity')::uuid,
  partner = coalesce((exterior->>'partner')::uuid, '00000000-0000-0000-0000-000000000000'),
  "when" = (exterior->>'when')::timestamptz,
  mode = exterior->>'mode',
  keywords = (select coalesce(array_agg(v), '{}') from jsonb_array_elements_text(coalesce(exterior->'keywords', '[]'::jsonb)) v),
  participant = (interior->>'participant')::uuid,
  snml = interior->>'snml',
  "location" = interior->'location';

-- Add NOT NULL constraints where appropriate
alter table c_participation
  alter column opportunity set not null,
  alter column "when" set not null;

-- Replace JSONB expression indexes with standard indexes
drop index if exists c_participation_by_opportunity;
drop index if exists c_participation_by_participant;
drop index if exists c_participation_by_when;

create index c_participation_by_opportunity on c_participation (opportunity);
create index c_participation_by_participant on c_participation (participant);
create index c_participation_by_when on c_participation ("when");

-- Drop old JSONB columns
alter table c_participation
  drop column exterior cascade,
  drop column interior cascade;

commit;
