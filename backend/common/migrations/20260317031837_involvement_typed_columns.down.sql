begin;

alter table c_involvement
  add column exterior jsonb,
  add column interior jsonb;

update c_involvement set
  exterior = jsonb_build_object(
    'opportunity', opportunity,
    'first', first,
    'latest', latest,
    'mode', mode
  ),
  interior = jsonb_build_object(
    'participant', participant,
    'location', "location"
  );

alter table c_involvement
  alter column exterior set not null,
  alter column interior set not null;

drop index if exists c_involvement_by_opportunity;
drop index if exists c_involvement_by_participant;
drop index if exists c_involvement_unique_participant_and_opportunity;

create index c_involvement_by_opportunity on c_involvement using GIN ((exterior -> 'opportunity'));
create index c_involvement_by_participant on c_involvement using GIN ((interior -> 'participant'));
create unique index c_involvement_unique_opporunity_and_participant on c_involvement ((exterior -> 'opportunity'), (interior -> 'participant'));

alter table c_involvement
  drop column opportunity,
  drop column first,
  drop column latest,
  drop column mode,
  drop column participant,
  drop column "location";

commit;
