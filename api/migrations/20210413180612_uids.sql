alter table c_opportunity drop column partner_uid;

alter table c_opportunity add column uid UUID unique not null generated always as ((exterior -> 'uid')::text::UUID) stored;

alter table c_person add column uid UUID unique not null generated always as ((exterior -> 'uid')::text::UUID) stored;
