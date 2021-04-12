-- add an item to an array serving as a set, if the item is not already in the set

create or replace function set_add(anyarray, anyelement)
       returns anyarray
       immutable
       parallel safe
       language sql as
$$
    select case when $2=any($1) then $1 else $1 || $2 end;
$$;

-- array_remove built-in serves as the inverse of set_add
-- https://www.postgresql.org/docs/current/functions-array.html

create or replace function set_discard(anyarray, anyelement)
       returns anyarray
       immutable
       parallel safe
       language sql as
$$
    select array_remove($1, $2);
$$;

drop type if exists OrganizationType;

create type OrganizationType as enum (
       'Unspecified',
       'MuseumOrScienceCenter',
       'Festival',
       'Library',
       'CollegeUniversity',
       'PK12School',
       'CommunityOrganization',
       'Club',
       'Zoo',
       'Aquarium',
       'Planetarium',
       'BotanicalGarden',
       'ParksAndRec',
       'HistoricalSite',
       'MakerOrganization',
       'Company',
       'GovtAgency'
);

drop type if exists EntityType;

create type EntityType as enum (
       'Opportunity',
       'Attraction'
);

drop type if exists OppDomain;

create type OppDomain as enum (
       'Unspecified',
       'CitizenScience',
       'LiveScience',
       'Maker',
       'Policy',
       'OutOfSchoolTimeProgram'
);

drop type if exists OpenHours;

create type OpenHours as (
       opens timetz not null,
       closes timetz not null
);

drop type if exists OpenDays;

create type OpenDays as (
       monday OpenHours,
       tuesday OpenHours,
       wednesday Openhours,
       thursday OpenHours,
       friday OpenHours,
       saturday OpenHours,
       sunday Openhours
);

drop type if exists Cost;

create type Cost as enum (
       'Unknown',
       'Free',
       'Cost'
);

drop type if exists LocationType;

create type LocationType as enum (
       'Unknown',
       'Any',
       'At',
       'Near'
);

drop table if exists Opportunity;

create table Opportunity (
       id serial primary key,
       partner_uid UUID unique not null,
       partner_name text not null,
       partner_created timestamptz,
       partner_updated timestamptz,
       partner_opp_url text not null,
       organization_name text not null,
       organization_type OrganizationType not null,
       organization_website text not null,
       entity_type EntityType not null,
       min_age integer not null default 0,
       max_age integer not null default 999,
       pes_domain OppDomain not null,
       tags text[] not null,
       ticket_required boolean not null,
       title text not null,
       description text not null,
       image_url text not null,
       start_dates timestamptz[] not null,
       has_end boolean not null,
       end_dates timestamptz[] not null,
       attraction_hours OpenDays,
       cost Cost not null,
       languages text[] not null,
       is_online boolean not null,
       location_type LocationType not null,
       location_name text not null,
       location_point geography(POINT, 4326),
       location_polygon geography(MULTIPOLYGON, 4326),
       address_street text not null,
       address_city text not null,
       address_state text not null,
       address_country text not null,
       address_zip text not null,
       contact_name text not null,
       contact_email text not null,
       contact_phone text not null,
       opp_hashtags text[] not null,
       opp_social_handles jsonb default '{}' not null,
       extra_data jsonb default '{}' not null
);

drop table if exists VenueType;

create table VenueType (
       id serial primary key,
       variant varchar(32) not null
);

drop table if exists OpportunityVenueType;

create table OpportunityVenueType (
       opportunity_id integer not null references Opportunity on delete cascade,
       venuetype_id integer not null references VenueType on delete cascade,
       primary key (opportunity_id, venuetype_id)
);

insert into VenueType(variant) values
       ('Unspecified'),
       ('MuseumOrScienceCenter'),
       ('Library'),
       ('PK12School'),
       ('CommunityOrganization'),
       ('Bar'),
       ('Outdoors'),
       ('CollegeUniversity');

drop table if exists OppDescriptor;

create table OppDescriptor (
       id serial primary key,
       variant varchar(32) not null
);

drop table if exists OpportunityDescriptor;

create table OpportunityDescriptor (
       opportunity_id integer not null references Opportunity on delete cascade,
       oppdescriptor_id integer not null references OppDescriptor on delete cascade
       primary key (opportunity_id, oppdescriptor_id)
)

insert into OppDescriptor(variant) values
       ('AdvocacyDays'),
       ('Bioblitz'),
       ('Camp'),
       ('CitizenScience'),
       ('CleanUp'),
       ('Club'),
       ('Community'),
       ('Competition'),
       ('Concert'),
       ('Conference'),
       ('Createathon'),
       ('Dance'),
       ('Exhibition'),
       ('ExpoStyle'),
       ('Festival'),
       ('Forum'),
       ('Fundraising'),
       ('Hackathon'),
       ('Lecture'),
       ('LiveScience'),
       ('Makeathon'),
       ('Maker'),
       ('MakerFaire'),
       ('Media'),
       ('Outreach'),
       ('Overnight'),
       ('Panel'),
       ('Policy'),
       ('ProfessionalDevelopment'),
       ('Research'),
       ('ScienceBlogging'),
       ('ScienceCafeOrPub'),
       ('ScienceOnTap'),
       ('SciencePoetrySlam'),
       ('ScienceSlam'),
       ('Service'),
       ('StarParty'),
       ('StoryCollider'),
       ('Tinker'),
       ('TinkerFaire'),
       ('Training'),
       ('Volunteering'),
       ('Workshop');

drop table if exists Topic;

create table Topic (
       id serial primary key,
       variant varchar(32) not null
);

drop table if exists OpportunityTopic;

create table OpportunityTopic (
       opportunity_id integer not null references Opportunity on delete cascade,
       topic_id integer not null references Topic on delete cascade
       primary key (opportunity_id, topic_id)
)

insert into Topic(variant) values
       ('Agriculture'),
       ('Alcohol'),
       ('Animals'),
       ('ArchaeologyAndCultural'),
       ('Art'),
       ('AstronomyAndSpace'),
       ('Awards'),
       ('Biology'),
       ('Birds'),
       ('Chemistry'),
       ('ClimateAndWeather'),
       ('ComputersAndTechnology'),
       ('CrowdFunding'),
       ('Design'),
       ('DisasterResponse'),
       ('EcologyAndEnvironment'),
       ('Education'),
       ('Engineering'),
       ('Food'),
       ('Geography'),
       ('GeologyAndEarthScience'),
       ('HealthAndMedicine'),
       ('InsectsAndPollinators'),
       ('Mathematics'),
       ('Medicine'),
       ('NatureAndOutdoors'),
       ('OceanWaterMarine'),
       ('Paleontology'),
       ('Physics'),
       ('Policy'),
       ('Psychology'),
       ('Religion'),
       ('Robotics'),
       ('SciencePolicy'),
       ('SocialScience'),
       ('Sound'),
       ('Technology'),
       ('Transportation');

