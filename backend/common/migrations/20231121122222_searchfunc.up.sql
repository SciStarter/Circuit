BEGIN;

CREATE TYPE T_OpportunityQueryPhysical AS ENUM (
    'in-person-or-online',
    'in-person',
    'online'
);

CREATE TYPE T_OpportunityQueryTemporal AS ENUM (
    'on-demand-or-scheduled',
    'scheduled',
    'on-demand'
);

CREATE TYPE T_OpportunityQueryOrdering AS ENUM (
    'alphabetical',
    'closest',
    'soonest',
    'any',
    'native',
    'unique',
    'partner-name'
);

CREATE TYPE T_VenueType AS ENUM (
    'indoors',
    'outdoors',
    -- Following variants are deprecated
    'museum_or_science_center',
    'library',
    'pk12school'
    'community_organization',
    'bar',
    'college_university',
    'unspecified'
);

CREATE TYPE T_OpportunityQuery AS (
    "uid" uuid,
    "slug" text,
    "accepted" bool,
    "withdrawn" bool,
    "entity_type" T_EntityType[],
    "title_contains" text,
    "tags" text[],
    "topics" T_Topic[],
    "partner" uuid,
    "partner_member" uuid,
    "prefer_partner" uuid,
    "near" float4[],
    "physical" T_OpportunityQueryPhysical,
    "temporal" T_OpportunityQueryTemporal,
    "text" text,
    "beginning" timestamptz,
    "ending" timestamptz,
    "min_age" int2,
    "max_age" int2,
    "kids_only" bool,
    "adults_only" bool,
    "descriptors" T_Descriptor[],
    "cost" T_Cost,
    "venue_type" T_VenueType,
    "host" text,
    "sort" T_OpportunityQueryOrdering,
    "page" int4,
    "per_page" int2,
    "involved" uuid,
    "saved" uuid,
    "participated" uuid,
    -- probability of retaining any given result in the match set, in the range (0-1).
    "sample" float4,
    "exclude" uuid[],
    "current" bool,
    "calendar" int4[],
    "region" text
);

CREATE FUNCTION c_opportunity_matches(opp c_opportunity, query T_OpportunityQuery) RETURNS bool
    LANGUAGE sql
    STABLE
    RETURN false
;

COMMIT;
