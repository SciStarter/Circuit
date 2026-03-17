begin;

-- Re-add JSONB columns
alter table c_participation
  add column exterior jsonb,
  add column interior jsonb;

-- Backfill JSONB from typed columns
update c_participation set
  exterior = jsonb_build_object(
    'opportunity', opportunity,
    'partner', partner,
    'when', "when",
    'mode', mode,
    'keywords', to_jsonb(keywords)
  ),
  interior = jsonb_build_object(
    'participant', participant,
    'snml', snml,
    'location', "location"
  );

-- Add NOT NULL constraints
alter table c_participation
  alter column exterior set not null,
  alter column interior set not null;

-- Restore JSONB expression indexes
drop index if exists c_participation_by_opportunity;
drop index if exists c_participation_by_participant;
drop index if exists c_participation_by_when;

create index c_participation_by_opportunity on c_participation using GIN ((exterior -> 'opportunity'));
create index c_participation_by_participant on c_participation using GIN ((interior -> 'participant'));
create index c_participation_by_when on c_participation using GIN ((exterior -> 'when'));

-- Drop typed columns
alter table c_participation
  drop column opportunity,
  drop column partner,
  drop column "when",
  drop column mode,
  drop column keywords,
  drop column participant,
  drop column snml,
  drop column "location";

commit;
