begin;

alter table c_partner
  add column exterior jsonb,
  add column interior jsonb;

update c_partner set
  exterior = jsonb_build_object(
    'uid', uid,
    'name', "name",
    'organization_type', organization_type,
    'pes_domain', pes_domain,
    'url', url,
    'image_url', image_url,
    'description', description,
    'background_color', background_color,
    'primary_color', primary_color,
    'secondary_color', secondary_color,
    'tertiary_color', tertiary_color,
    'under', under,
    'open_submission', open_submission,
    'default_query', default_query
  ),
  interior = jsonb_build_object(
    'manager', manager,
    'contact', contact,
    'prime', prime,
    'authorized', to_jsonb(authorized),
    'pending', to_jsonb(pending),
    'secret', secret
  );

alter table c_partner
  alter column exterior set not null,
  alter column interior set not null;

drop index if exists c_partner_uid;
drop index if exists c_partner_prime;
drop index if exists c_partner_authorized;

create index c_partner_uid on c_partner using GIN ((exterior -> 'uid'));
create index c_partner_prime on c_partner using GIN ((interior -> 'prime'));
create index c_partner_authorized on c_partner using GIN ((interior -> 'authorized'));

alter table c_partner
  drop column uid,
  drop column "name",
  drop column organization_type,
  drop column pes_domain,
  drop column url,
  drop column image_url,
  drop column description,
  drop column background_color,
  drop column primary_color,
  drop column secondary_color,
  drop column tertiary_color,
  drop column under,
  drop column open_submission,
  drop column default_query,
  drop column manager,
  drop column contact,
  drop column prime,
  drop column authorized,
  drop column pending,
  drop column secret;

commit;
