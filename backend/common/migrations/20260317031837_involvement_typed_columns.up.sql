begin;

-- Add typed columns alongside existing JSONB columns
alter table c_involvement
  add column opportunity uuid,
  add column first timestamptz,
  add column latest timestamptz,
  add column mode smallint,
  add column participant uuid,
  add column "location" jsonb;

-- Backfill from JSONB
update c_involvement set
  opportunity = (exterior->>'opportunity')::uuid,
  first = (exterior->>'first')::timestamptz,
  latest = (exterior->>'latest')::timestamptz,
  mode = (exterior->>'mode')::smallint,
  participant = (interior->>'participant')::uuid,
  "location" = interior->'location';

-- Add NOT NULL constraints
alter table c_involvement
  alter column opportunity set not null,
  alter column first set not null,
  alter column latest set not null,
  alter column mode set not null,
  alter column participant set not null;

-- Replace JSONB expression indexes with standard indexes
drop index if exists c_involvement_by_opportunity;
drop index if exists c_involvement_by_participant;
drop index if exists c_involvement_unique_opporunity_and_participant;

create index c_involvement_by_opportunity on c_involvement (opportunity);
create index c_involvement_by_participant on c_involvement (participant);
create unique index c_involvement_unique_participant_and_opportunity on c_involvement (participant, opportunity);

-- Drop old JSONB columns
alter table c_involvement
  drop column exterior,
  drop column interior;

commit;
