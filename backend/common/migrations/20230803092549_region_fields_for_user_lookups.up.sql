alter table c_person drop column "home_location";
alter table c_person drop column "last_location";

alter table c_person add column "home_location" geography(Point, 4326);
alter table c_person add column "last_location" geography(Point, 4326);

alter table c_person add column "state" varchar(64);
alter table c_person add column "metro" varchar(64);

create index c_person_by_state on c_person("state");
create index c_person_by_state_and_metro on c_person("state", "metro");
