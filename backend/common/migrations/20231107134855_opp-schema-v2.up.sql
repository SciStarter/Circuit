BEGIN;

DROP FUNCTION "c_opportunity_is_current_as_of";
DROP FUNCTION "c_opportunity_is_scheduled";
DROP FUNCTION "c_opportunity_is_ondemand";
DROP FUNCTION "c_opportunity_by_uid_likes_during";
DROP FUNCTION "c_opportunity_by_uid_is_status";
DROP FUNCTION "c_opportunity_by_uid_domain";
DROP FUNCTION "c_opportunity_is_current";
DROP FUNCTION "c_opportunity_by_uid_is_current_as_of";
DROP FUNCTION "c_opportunity_by_uid_is_current";

DROP INDEX "c_opportunity_by_slug";
DROP INDEX "c_opportunity_cost";
DROP INDEX "c_opportunity_end_dates";
DROP INDEX "c_opportunity_fulltext_english";
DROP INDEX "c_opportunity_is_online";
DROP INDEX "c_opportunity_max_age";
DROP INDEX "c_opportunity_min_age";
DROP INDEX "c_opportunity_min_domain";
DROP INDEX "c_opportunity_organization_name";
DROP INDEX "c_opportunity_partner";
DROP INDEX "c_opportunity_start_dates";
DROP INDEX "c_opportunity_tags";
DROP INDEX "c_opportunity_uid";
DROP INDEX "c_opportunity_withdrawn";
DROP INDEX IF EXISTS "c_opportyunity_by_location_type_text";
DROP INDEX IF EXISTS "c_opportunity_location_type";

ALTER TABLE "c_opportunity" RENAME TO "c_opportunity_v1";
ALTER TABLE "c_opportunity_overlay" RENAME TO "c_opportunity_overlay_v1";

CREATE TYPE T_OrganizationType AS ENUM (
    'museum_or_science_center',
    'festival',
    'library',
    'college_university',
    'pk12school',
    'community_organization',
    'club',
    'zoo',
    'aquarium',
    'planetarium',
    'botanical_garden',
    'parks_and_rec',
    'historical_site',
    'maker_organization',
    'company',
    'govt_agency',
    'maker_space',
    'unspecified'
);

CREATE TYPE T_EntityType AS ENUM (
    'unspecified',
    'attraction',
    'page_just_content',
    'page_add_opportunities',
    'opportunity'
);

CREATE TYPE T_PESDomain AS ENUM (
    'citizen_science',
    'live_science',
    'museum_or_science_center',
    'maker',
    'policy',
    'out_of_school_time_program',
    'formal_education',
    'science_communications',
    'unspecified'
);

CREATE TYPE T_Recurrence AS ENUM (
    'once',
    'daily',
    'weekly'
);

CREATE TYPE T_Cost AS ENUM (
    'free',
    'cost',
    'unknown'
);

CREATE TYPE T_LocationType AS ENUM (
    'any',
    'at',
    'near',
    'unknown'
);

CREATE TYPE T_Topic AS ENUM (
    'agriculture',
    'alcohol',
    'animals',
    'archaeology_and_cultural',
    'art',
    'astronomy_and_space',
    'awards',
    'biology',
    'birds',
    'chemistry',
    'climate_and_weather',
    'computers_and_technology',
    'crowd_funding',
    'design',
    'disaster_response',
    'ecology_and_environment',
    'education',
    'engineering',
    'food',
    'general_science',
    'geography',
    'geology_and_earth_science',
    'health_and_medicine',
    'insects_and_pollinators',
    'mathematics',
    'nature_and_outdoors',
    'ocean_water_marine',
    'paleontology',
    'physics',
    'policy',
    'psychology',
    'religion',
    'robotics',
    'social_science',
    'sound',
    'technology',
    'transportation'
);

CREATE TYPE T_Descriptor AS ENUM (
    'advocacy_days',
    'bioblitz',
    'camp',
    'citizen_science',
    'clean_up',
    'club',
    'community',
    'competition',
    'concert',
    'conference',
    'create-a-thon',
    'dance',
    'exhibition',
    'expo_style',
    'festival',
    'forum',
    'fundraising',
    'hack-a-thon',
    'lecture',
    'live_science',
    'make-a-thon',
    'maker',
    'maker_faire',
    'media',
    'outreach',
    'overnight',
    'panel',
    'policy',
    'professional_development',
    'research',
    'science_blogging',
    'science_cafe_or_pub',
    'science_on_tap',
    'science_poetry_slam',
    'science_slam',
    'service',
    'star_party',
    'story_collider',
    'tinker',
    'tinker_faire',
    'training',
    'volunteering',
    'workshop'
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

CREATE TYPE T_ReviewStatus AS ENUM (
    'draft',
    'pending',
    'reject',
    'publish',
    'not_required'
);

CREATE TABLE "c_opportunity" (
    "id" serial PRIMARY KEY,
    "uid" uuid UNIQUE NOT NULL,
    "slug" text UNIQUE NOT NULL CHECK ("slug" = lower("slug")),
    "created" timestamptz NOT NULL DEFAULT now(),
    "updated" timestamptz NOT NULL DEFAULT now(),
    "gpt_record" boolean NOT NULL DEFAULT false,
    "partner_name" text NOT NULL,
    "partner_website" text,
    "partner_logo_url" text,
    "partner_created" timestamptz,
    "partner_updated" timestamptz,
    "partner_opp_url" text,
    "organization_name" text NOT NULL DEFAULT '',
    "organization_type" T_OrganizationType NOT NULL DEFAULT 'unspecified',
    "organization_website" text,
    "organization_logo_url" text,
    "entity_type" T_EntityType NOT NULL DEFAULT 'opportunity',
    "min_age" smallint NOT NULL DEFAULT 0,
    "max_age" smallint NOT NULL DEFAULT 999,
    "pes_domain" T_PESDomain NOT NULL DEFAULT 'unspecified',
    "ticket_required" boolean NOT NULL DEFAULT false,
    "title" text NOT NULL,
    "description" text NOT NULL DEFAULT '',
    "short_desc" text NOT NULL DEFAULT '',
    "image_url" text NOT NULL DEFAULT '',
    "image_credit" text NOT NULL DEFAULT '',
    "recurrence" T_Recurrence NOT NULL DEFAULT 'once',
    "end_recurrence" timestamptz,
    "timezone" text,
    "cost" T_Cost NOT NULL DEFAULT 'unknown',
    "is_online" boolean NOT NULL DEFAULT false,
    "location_type" T_LocationType NOT NULL DEFAULT 'unknown',
    "location_name" text NOT NULL,
    "location_point" geography(Point, 4326),
    "location_polygon" geography(MultiPolygon, 4326),
    "address_street" text NOT NULL DEFAULT '',
    "address_city" text NOT NULL DEFAULT '',
    "address_state" text NOT NULL DEFAULT '',
    "address_country" text NOT NULL DEFAULT '',
    "address_zip" text NOT NULL DEFAULT '',
    "partner" uuid NOT NULL,
    UNIQUE ("partner", "title")
);

CREATE TRIGGER c_opportunity_set_updated BEFORE UPDATE ON c_opportunity
    FOR EACH ROW EXECUTE PROCEDURE set_updated();

CREATE INDEX c_opportunity_via_location_type ON c_opportunity ("location_type");
CREATE INDEX c_opportunity_via_cost ON c_opportunity ("cost");
CREATE INDEX c_opportunity_via_is_online ON c_opportunity ("is_online");
CREATE INDEX c_opportunity_via_organization_name ON c_opportunity ("organization_name");
CREATE INDEX c_opportunity_via_partner ON c_opportunity ("partner");
CREATE INDEX c_opportunity_via_location_point ON c_opportunity USING GIST ("location_point");
CREATE INDEX c_opportunity_via_location_polygon ON c_opportunity USING GIST ("location_polygon");
CREATE INDEX c_opportunity_via_gpt_record ON c_opportunity ("gpt_record");

SELECT setval(
    (SELECT pg_get_serial_sequence('c_opportunity', 'id')),
    (SELECT max(id) FROM c_opportunity_v1)
);

CREATE TABLE c_opportunity_instance (
    "id" serial PRIMARY KEY,
    "opportunity_id" integer NOT NULL REFERENCES "c_opportunity" ON DELETE CASCADE,
    "start" timestamptz NOT NULL,
    "end" timestamptz,
    UNIQUE ("opportunity_id", "start")
);

CREATE INDEX c_opportunity_instance_via_opportunity_id ON c_opportunity_instance ("opportunity_id");

CREATE TABLE c_opportunity_interior (
    "opportunity_id" integer PRIMARY KEY REFERENCES "c_opportunity" ON DELETE CASCADE,
    "updated" timestamptz NOT NULL DEFAULT now(),
    "accepted" boolean DEFAULT false,
    "withdrawn" boolean NOT NULL DEFAULT false,
    "submitted_by" uuid,
    "review_status" T_ReviewStatus NOT NULL DEFAULT 'not_required',
    "contact_name" text NOT NULL DEFAULT '',
    "contact_email" text NOT NULL DEFAULT '',
    "contact_phone" text NOT NULL DEFAULT '',
    "extra_data" jsonb NOT NULL DEFAULT '{}'::jsonb
);

CREATE TRIGGER c_opportunity_interior_set_updated BEFORE UPDATE ON c_opportunity_interior
    FOR EACH ROW EXECUTE PROCEDURE set_updated();

CREATE TABLE c_opportunity_tag (
    "id" serial PRIMARY KEY,
    "overlay" boolean NOT NULL DEFAULT false,
    "opportunity_id" integer NOT NULL REFERENCES "c_opportunity" ON DELETE CASCADE,
    "tag" text NOT NULL CHECK ("tag" = lower("tag")),
    UNIQUE ("opportunity_id", "tag")
);

CREATE INDEX c_opportunity_tag_via_opportunity_id ON c_opportunity_tag ("opportunity_id");
CREATE INDEX c_opportunity_tag_via_tag ON c_opportunity_tag ("tag");

CREATE TABLE c_opportunity_topic (
    "id" serial PRIMARY KEY,
    "overlay" boolean NOT NULL DEFAULT false,
    "opportunity_id" integer NOT NULL REFERENCES "c_opportunity" ON DELETE CASCADE,
    "topic" T_Topic NOT NULL,
    UNIQUE ("opportunity_id", "topic")
);

CREATE INDEX c_opportunity_topic_via_opportunity_id ON c_opportunity_topic ("opportunity_id");
CREATE INDEX c_opportunity_topic_via_topic ON c_opportunity_topic ("topic");

CREATE TABLE c_opportunity_descriptor (
    "id" serial PRIMARY KEY,
    "overlay" boolean NOT NULL DEFAULT false,
    "opportunity_id" integer NOT NULL REFERENCES "c_opportunity" ON DELETE CASCADE,
    "descriptor" T_Descriptor NOT NULL,
    UNIQUE ("opportunity_id", "descriptor")
);

CREATE INDEX c_opportunity_descriptor_via_opportunity_id ON c_opportunity_descriptor ("opportunity_id");
CREATE INDEX c_opportunity_descriptor_via_descriptor ON c_opportunity_descriptor ("descriptor");

CREATE TABLE c_opportunity_venue_type (
    "id" serial PRIMARY KEY,
    "overlay" boolean NOT NULL DEFAULT false,
    "opportunity_id" integer NOT NULL REFERENCES "c_opportunity" ON DELETE CASCADE,
    "venue_type" T_VenueType NOT NULL,
    UNIQUE ("opportunity_id", "venue_type")
);

CREATE INDEX c_opportunity_venue_type_via_opportunity_id ON c_opportunity_venue_type ("opportunity_id");
CREATE INDEX c_opportunity_venue_type_via_venue_type ON c_opportunity_venue_type ("venue_type");

CREATE TABLE c_opportunity_hashtag (
    "id" serial PRIMARY KEY,
    "opportunity_id" integer NOT NULL REFERENCES "c_opportunity" ON DELETE CASCADE,
    "hashtag" text NOT NULL,
    UNIQUE ("opportunity_id", "hashtag")
);

CREATE INDEX c_opportunity_hashtag_via_opportunity_id ON c_opportunity_hashtag ("opportunity_id");
CREATE INDEX c_opportunity_hashtag_via_hashtag ON c_opportunity_hashtag ("hashtag");

CREATE TABLE c_opportunity_social_handle (
    "id" serial PRIMARY KEY,
    "opportunity_id" integer NOT NULL REFERENCES "c_opportunity" ON DELETE CASCADE,
    "network" text NOT NULL,
    "handle" text NOT NULL,
    UNIQUE ("opportunity_id", "network")
);

CREATE INDEX c_opportunity_social_handle_via_opportunity_id ON c_opportunity_social_handle ("opportunity_id");

CREATE TABLE c_opportunity_language (
    "id" serial PRIMARY KEY,
    "opportunity_id" integer NOT NULL REFERENCES "c_opportunity" ON DELETE CASCADE,
    "language" text NOT NULL,
    UNIQUE ("opportunity_id", "language")
);

CREATE INDEX c_opportunity_language_via_opportunity_id ON c_opportunity_language ("opportunity_id");
CREATE INDEX c_opportunity_language_via_language ON c_opportunity_language ("language");

CREATE FUNCTION "c_opportunity_tsvector" ("row" c_opportunity) RETURNS tsvector
LANGUAGE plpgsql
IMMUTABLE STRICT AS $func$
BEGIN
    RETURN to_tsvector(
        'english',
        "row"."title" || ' ' ||
        "row"."description" || ' ' ||
        "row"."partner_name" || ' ' ||
        "row"."organization_name" || ' ' ||
        "row"."location_name" || ' ' ||
        "row"."address_city" || ' ' ||
        "row"."address_state" || ' ' ||
        "row"."address_country"
    );
END;
$func$;

CREATE FUNCTION c_opportunity_is_current_as_of("opp" c_opportunity, stamp timestamptz) returns boolean AS
$func$
DECLARE
 "interior" c_opportunity_interior;
BEGIN
 SELECT * INTO "interior" FROM c_opportunity_interior WHERE "opportunity_id" = "opp"."id";
 RETURN (
   "interior"."review_status" IN ('publish', 'not_required')
   AND
   "interior"."accepted" = true
   AND
   "interior"."withdrawn" = false
   AND
   (
       (
          (opp."recurrence" = 'daily' OR opp."recurrence" = 'weekly')
          AND
          (opp."end_recurrence" IS NULL OR opp."end_recurrence"> "stamp")
       )
       OR
       EXISTS(SELECT 1 FROM c_opportunity_instance WHERE "opportunity_id" = opp."id" AND ("end" IS NULL OR "end" > "stamp"))
       OR NOT
       EXISTS(SELECT 1 from c_opportunity_instance WHERE "opportunity_id" = opp."id")
   )
 );
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_is_scheduled("opp" c_opportunity) returns boolean AS
$func$
DECLARE
  "total" bigint;
  "inst" c_opportunity_instance;
BEGIN
 SELECT COUNT(*) INTO "total" FROM c_opportunity_instance WHERE "opportunity_id" = opp."id";
 SELECT * INTO "inst" FROM c_opportunity_instance WHERE "opportunity_id" = opp."id" LIMIT 1;
 RETURN (
    "total" > 1
    OR (
     "total" = 1
     AND
     "inst"."end" IS NOT NULL
     AND
     AGE("inst"."end", "inst"."start") <= interval '7 days'
   )
 );
END
$func$ language plpgsql stable;

CREATE FUNCTION c_opportunity_is_ondemand("opp" c_opportunity) returns boolean AS
$func$
BEGIN
 RETURN NOT c_opportunity_is_scheduled(opp);
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_by_uid_likes_during("uid" uuid, "begin" timestamptz, "end" timestamptz) returns bigint AS
$func$
DECLARE
 val bigint;
BEGIN
 SELECT COALESCE(COUNT(*), 0) INTO val FROM c_opportunity_like l LEFT JOIN c_opportunity o ON l."opportunity_id" = o."id" WHERE o."uid" = "uid" AND "when" >= "begin" AND "when" < "end";
 RETURN val;
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_by_uid_is_status("uid" uuid, "status" integer) returns boolean AS
$func$
DECLARE
 opp c_opportunity;
BEGIN
 SELECT * INTO opp FROM c_opportunity o WHERE o."uid" = "uid" LIMIT 1;
 CASE status
  WHEN 2 THEN RETURN c_opportunity_is_current(opp) = false;
  WHEN 1 THEN RETURN c_opportunity_is_current(opp) = true;
  ELSE RETURN true;
 END CASE;
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_by_uid_domain("uid" uuid) returns T_PESDomain AS
$func$
DECLARE
 opp c_opportunity;
 partner c_partner;
BEGIN
 SELECT * INTO opp FROM c_opportunity o WHERE o."uid" = "uid" LIMIT 1;
 CASE opp."pes_domain"
  WHEN 'unspecified' THEN
   SELECT * INTO partner FROM c_partner p WHERE (p."exterior"->>'uid')::uuid = opp."partner" LIMIT 1;
   RETURN (partner."exterior"->>'pes_domain')::T_PESDomain;
  ELSE RETURN opp."pes_domain";
 END CASE;
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_is_current("opp" c_opportunity) returns boolean AS
$func$
BEGIN
 RETURN c_opportunity_is_current_as_of(opp, CURRENT_TIMESTAMP);
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_by_uid_is_current_as_of("uid" uuid, "stamp" timestamptz) returns boolean AS
$func$
DECLARE
 opp c_opportunity;
BEGIN
 SELECT * INTO opp FROM c_opportunity o WHERE o."uid" = "uid" LIMIT 1;
 RETURN c_opportunity_is_current_as_of(opp, stamp);
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_by_uid_is_current("uid" uuid) returns boolean AS
$func$
DECLARE
 opp c_opportunity;
BEGIN
 SELECT * INTO opp FROM c_opportunity o WHERE o."uid" = "uid" LIMIT 1;
 RETURN c_opportunity_is_current(opp);
END
$func$ LANGUAGE plpgsql STABLE;

-- SELECT string_agg("tag", ' ') INTO "tags" FROM c_opportunity_tag WHERE "opportunity_id" = "row"."id";
-- SELECT string_agg("topic", ' ') INTO "topics" FROM c_opportunity_topic WHERE "opportunity_id" = "row"."id";
-- SELECT string_agg("descriptor", ' ') INTO "activities" FROM c_opportunity_descriptor WHERE "opportunity_id" = "row"."id";
-- SELECT string_agg("hashtag", ' ') INTO "hashtags" FROM c_opportunity_hashtag WHERE "opportunity_id" = "row"."id";

-- searches should be done as WHERE c_opportunity_tsvector(c_opportunity) @@ websearch_to_tsquery(...)
CREATE INDEX "c_opportunity_via_fulltext_english" ON "c_opportunity" USING gin (c_opportunity_tsvector(c_opportunity));

INSERT
    INTO c_opportunity (
        "id",
        "uid",
        "slug",
        "partner_name",
        "partner_website",
        "partner_logo_url",
        "partner_created",
        "partner_updated",
        "partner_opp_url",
        "organization_name",
        "organization_type",
        "organization_website",
        "organization_logo_url",
        "entity_type",
        "min_age",
        "max_age",
        "pes_domain",
        "ticket_required",
        "title",
        "description",
        "short_desc",
        "image_url",
        "image_credit",
        "recurrence",
        "end_recurrence",
        "timezone",
        "cost",
        "is_online",
        "location_type",
        "location_name",
        "location_point",
        "location_polygon",
        "address_street",
        "address_city",
        "address_state",
        "address_country",
        "address_zip",
        "partner"
    )
    SELECT
        first("id" ORDER BY updated DESC) AS "id",
        first(exterior->>'uid' ORDER BY updated DESC)::uuid AS "uid",
        lower(first(exterior->>'slug' ORDER BY length(exterior->>'slug') ASC)) AS "slug",
        first(exterior->>'partner_name' ORDER BY updated DESC) AS "partner_name",
        first(exterior->>'partner_website' ORDER BY updated DESC) AS "partner_website",
        first(exterior->>'partner_logo_url' ORDER BY updated DESC) AS "partner_logo_url",
        first(exterior->>'partner_created' ORDER BY updated DESC)::timestamptz AS "partner_created",
        first(exterior->>'partner_updated' ORDER BY updated DESC)::timestamptz AS "partner_updated",
        first(exterior->>'partner_opp_url' ORDER BY updated DESC) AS "partner_opp_url",
        first(exterior->>'organization_name' ORDER BY updated DESC) AS "organization_name",
        first(exterior->>'organization_type' ORDER BY updated DESC)::T_OrganizationType AS "organization_type",
        first(exterior->>'organization_website' ORDER BY updated DESC) AS "organization_website",
        first(exterior->>'organization_logo_url' ORDER BY updated DESC) AS "organization_logo_url",
        first(exterior->>'entity_type' ORDER BY updated DESC)::T_EntityType AS "entity_type",
        first(exterior->'min_age' ORDER BY updated DESC)::smallint AS "min_age",
        first(exterior->'max_age' ORDER BY updated DESC)::smallint AS "max_age",
        first(exterior->>'pes_domain' ORDER BY updated DESC)::T_PESDomain AS "pes_domain",
        first(exterior->'ticket_required' ORDER BY updated DESC)::boolean AS "ticket_required",
        exterior->>'title' AS "title",
        first(exterior->>'description' ORDER BY updated DESC) AS "description",
        first(exterior->>'short_desc' ORDER BY updated DESC) AS "short_desc",
        first(exterior->>'image_url' ORDER BY updated DESC) AS "image_url",
        first(exterior->>'image_credit' ORDER BY updated DESC) AS "image_credit",
        first(exterior->>'recurrence' ORDER BY updated DESC)::T_Recurrence AS "recurrence",
        first(exterior->>'end_recurrence' ORDER BY updated DESC)::timestamptz AS "end_recurrence",
        first(exterior->>'timezone' ORDER BY updated DESC) AS "timezone",
        first(exterior->>'cost' ORDER BY updated DESC)::T_Cost AS "cost",
        first(exterior->'is_online' ORDER BY updated DESC)::boolean AS "is_online",
        first(exterior->>'location_type' ORDER BY updated DESC)::T_LocationType AS "location_type",
        first(exterior->>'location_name' ORDER BY updated DESC) AS "location_name",
        cast(ST_SetSRID(ST_GeomFromGeoJSON(first(exterior->>'location_point' ORDER BY updated DESC)), 4326) AS geography) AS "location_point",
        cast(ST_SetSRID(ST_GeomFromGeoJSON(first(exterior->>'location_polygon' ORDER BY updated DESC)), 4326) AS geography) AS "location_polygon",
        first(exterior->>'address_street' ORDER BY updated DESC) AS "address_street",
        first(exterior->>'address_city' ORDER BY updated DESC) AS "address_city",
        first(exterior->>'address_state' ORDER BY updated DESC) AS "address_state",
        first(exterior->>'address_country' ORDER BY updated DESC) AS "address_country",
        first(exterior->>'address_zip' ORDER BY updated DESC) AS "address_zip",
        (exterior->>'partner')::uuid AS "partner"
    FROM
        c_opportunity_v1
    GROUP BY
        exterior->>'partner',
        exterior->>'title'
;

INSERT
    INTO c_opportunity_interior (
        "opportunity_id",
        "accepted",
        "withdrawn",
        "submitted_by",
        "review_status",
        "contact_name",
        "contact_email",
        "contact_phone",
        "extra_data"
    )
    SELECT
        v2."id" AS "opportunity_id",
        (v1.interior->'accepted')::boolean AS "accepted",
        (v1.interior->'withdrawn')::boolean AS "withdrawn",
        (v1.interior->>'submitted_by')::uuid AS "submitted_by",
        (v1.interior->>'review_status')::T_ReviewStatus AS "review_status",
        v1.interior->>'contact_name' AS "contact_name",
        v1.interior->>'contact_email' AS "contact_email",
        v1.interior->>'contact_phone' AS "contact_phone",
        v1.interior->'extra_data' AS "extra_data"
    FROM
        c_opportunity v2
        JOIN c_opportunity_v1 v1 ON v1."id" = v2."id"
;

WITH old_instances AS (
    SELECT
        (
            SELECT
                cn."id"
            FROM
                c_opportunity_v1 cn
            WHERE
                cn.exterior->>'partner' = c.exterior->>'partner' AND
                cn.exterior->>'title' = c.exterior->>'title'
            ORDER BY cn."updated" DESC
            LIMIT 1
        ) AS "opportunity_id",
        c.exterior->>'partner' AS "partner",
        c.exterior->>'title' AS "title",
        c.updated AS "updated",
        s.v AS "start",
        e.v AS "end"
    FROM
        c_opportunity_v1 AS c,
            jsonb_array_elements_text(c.exterior->'start_datetimes') WITH ORDINALITY AS s(v,i)
        LEFT JOIN
            jsonb_array_elements_text(c.exterior->'end_datetimes') WITH ORDINALITY AS e(v,i)
        ON s.i = e.i
)
INSERT
    INTO c_opportunity_instance (
        "opportunity_id",
        "start",
        "end"
    )
    SELECT
        o."opportunity_id",
        o."start"::timestamptz AS "start",
        o."end"::timestamptz AS "end"
    FROM
        old_instances o
    ON CONFLICT ("opportunity_id", "start")
        DO NOTHING
;

WITH old_tags AS (
    SELECT
        (
            SELECT
                cn."id"
            FROM
                c_opportunity_v1 cn
            WHERE
                cn.exterior->>'partner' = c.exterior->>'partner' AND
                cn.exterior->>'title' = c.exterior->>'title'
            ORDER BY cn."updated" DESC
            LIMIT 1
        ) AS "opportunity_id",
        c.exterior->>'partner' AS "partner",
        c.exterior->>'title' AS "title",
        c.updated AS "updated",
        t AS "tag"
    FROM
        c_opportunity_v1 AS c,
        jsonb_array_elements_text(c.exterior->'tags') AS t
)
INSERT
    INTO c_opportunity_tag (
        "opportunity_id",
        "tag"
    )
    SELECT
        o."opportunity_id",
        lower(o."tag") AS "tag"
    FROM
        old_tags o
    ON CONFLICT
        DO NOTHING
;

WITH old_topics AS (
    SELECT
        (
            SELECT
                cn."id"
            FROM
                c_opportunity_v1 cn
            WHERE
                cn.exterior->>'partner' = c.exterior->>'partner' AND
                cn.exterior->>'title' = c.exterior->>'title'
            ORDER BY cn."updated" DESC
            LIMIT 1
        ) AS "opportunity_id",
        c.exterior->>'partner' AS "partner",
        c.exterior->>'title' AS "title",
        c.updated AS "updated",
        t AS "topic"
    FROM
        c_opportunity_v1 AS c,
        jsonb_array_elements_text(c.exterior->'opp_topics') AS t
)
INSERT
    INTO c_opportunity_topic (
        "opportunity_id",
        "topic"
    )
    SELECT
        o."opportunity_id",
        o."topic"::T_Topic
    FROM
        old_topics o
    ON CONFLICT
        DO NOTHING
;

WITH old_descriptors AS (
    SELECT
        (
            SELECT
                cn."id"
            FROM
                c_opportunity_v1 cn
            WHERE
                cn.exterior->>'partner' = c.exterior->>'partner' AND
                cn.exterior->>'title' = c.exterior->>'title'
            ORDER BY cn."updated" DESC
            LIMIT 1
        ) AS "opportunity_id",
        c.exterior->>'partner' AS "partner",
        c.exterior->>'title' AS "title",
        c.updated AS "updated",
        d AS "descriptor"
    FROM
        c_opportunity_v1 AS c,
        jsonb_array_elements_text(c.exterior->'opp_descriptor') AS d
)
INSERT
    INTO c_opportunity_descriptor (
        "opportunity_id",
        "descriptor"
    )
    SELECT
        o."opportunity_id",
        o."descriptor"::T_Descriptor
    FROM
        old_descriptors o
    ON CONFLICT
        DO NOTHING
;

WITH old_venue_types AS (
    SELECT
        (
            SELECT
                cn."id"
            FROM
                c_opportunity_v1 cn
            WHERE
                cn.exterior->>'partner' = c.exterior->>'partner' AND
                cn.exterior->>'title' = c.exterior->>'title'
            ORDER BY cn."updated" DESC
            LIMIT 1
        ) AS "opportunity_id",
        c.exterior->>'partner' AS "partner",
        c.exterior->>'title' AS "title",
        c.updated AS "updated",
        t AS "venue_type"
    FROM
        c_opportunity_v1 AS c,
        jsonb_array_elements_text(c.exterior->'opp_venue') AS t
)
INSERT
    INTO c_opportunity_venue_type (
        "opportunity_id",
        "venue_type"
    )
    SELECT
        o."opportunity_id",
        o."venue_type"::T_VenueType
    FROM
        old_venue_types o
    ON CONFLICT
        DO NOTHING
;

WITH old_hashtags AS (
    SELECT
        (
            SELECT
                cn."id"
            FROM
                c_opportunity_v1 cn
            WHERE
                cn.exterior->>'partner' = c.exterior->>'partner' AND
                cn.exterior->>'title' = c.exterior->>'title'
            ORDER BY cn."updated" DESC
            LIMIT 1
        ) AS "opportunity_id",
        c.exterior->>'partner' AS "partner",
        c.exterior->>'title' AS "title",
        c.updated AS "updated",
        trim(h) AS "hashtag"
    FROM
        c_opportunity_v1 AS c,
        jsonb_array_elements_text(c.exterior->'opp_hashtags') AS h
    WHERE
       trim(h) != ''
)
INSERT
    INTO c_opportunity_hashtag (
        "opportunity_id",
        "hashtag"
    )
    SELECT
        o."opportunity_id",
        CASE
            WHEN starts_with(o."hashtag", '#') THEN o."hashtag"
            ELSE '#' || o."hashtag"
        END
    FROM
        old_hashtags o
    ON CONFLICT
        DO NOTHING
;

CREATE TABLE "c_opportunity_overlay" (
    "opportunity_id" integer PRIMARY KEY REFERENCES "c_opportunity" ON DELETE CASCADE,
    "partner_name" text,
    "partner_website" text,
    "partner_logo_url" text,
    "partner_opp_url" text,
    "organization_name" text,
    "organization_type" T_OrganizationType,
    "organization_website" text,
    "organization_logo_url" text,
    "entity_type" T_EntityType,
    "min_age" smallint,
    "max_age" smallint,
    "pes_domain" T_PESDomain,
    "ticket_required" boolean,
    "title" text,
    "description" text,
    "short_desc" text,
    "image_url" text,
    "image_credit" text,
    "recurrence" T_Recurrence,
    "end_recurrence" timestamptz,
    "timezone" text,
    "cost" T_Cost,
    "is_online" boolean,
    "location_type" T_LocationType,
    "location_name" text,
    "location_point" geography(Point, 4326),
    "location_polygon" geography(MultiPolygon, 4326),
    "address_street" text,
    "address_city" text,
    "address_state" text,
    "address_country" text,
    "address_zip" text
);

INSERT
    INTO c_opportunity_overlay (
        "opportunity_id",
        "partner_name",
        "partner_website",
        "partner_logo_url",
        "partner_opp_url",
        "organization_name",
        "organization_type",
        "organization_website",
        "organization_logo_url",
        "entity_type",
        "min_age",
        "max_age",
        "pes_domain",
        "ticket_required",
        "title",
        "description",
        "short_desc",
        "image_url",
        "image_credit",
        "recurrence",
        "end_recurrence",
        "timezone",
        "cost",
        "is_online",
        "location_type",
        "location_name",
        "location_point",
        "location_polygon",
        "address_street",
        "address_city",
        "address_state",
        "address_country",
        "address_zip"
    )
    SELECT
        "opportunity_id",
        nullif(exterior->>'partner_name', '') AS "partner_name",
        nullif(exterior->>'partner_website', '') AS "partner_website",
        nullif(exterior->>'partner_logo_url', '') AS "partner_logo_url",
        nullif(exterior->>'partner_opp_url', '') AS "partner_opp_url",
        nullif(exterior->>'organization_name', '') AS "organization_name",
        nullif(exterior->>'organization_type', '')::T_OrganizationType AS "organization_type",
        nullif(exterior->>'organization_website', '') AS "organization_website",
        nullif(exterior->>'organization_logo_url', '') AS "organization_logo_url",
        nullif(exterior->>'entity_type', '')::T_EntityType AS "entity_type",
        nullif(exterior->'min_age', 'null'::jsonb)::smallint AS "min_age",
        nullif(exterior->'max_age', 'null'::jsonb)::smallint AS "max_age",
        nullif(exterior->>'pes_domain', '')::T_PESDomain AS "pes_domain",
        nullif(exterior->'ticket_required', 'null'::jsonb)::boolean AS "ticket_required",
        nullif(exterior->>'title', '') AS "title",
        nullif(exterior->>'description', '') AS "description",
        nullif(exterior->>'short_desc', '') AS "short_desc",
        nullif(exterior->>'image_url', '') AS "image_url",
        nullif(exterior->>'image_credit', '') AS "image_credit",
        nullif(exterior->>'recurrence', '')::T_Recurrence AS "recurrence",
        nullif(exterior->>'end_recurrence', '')::timestamptz AS "end_recurrence",
        nullif(exterior->>'timezone', '') AS "timezone",
        nullif(exterior->>'cost', '')::T_Cost AS "cost",
        nullif(exterior->'is_online', 'null'::jsonb)::boolean AS "is_online",
        nullif(exterior->>'location_type', '')::T_LocationType AS "location_type",
        nullif(exterior->>'location_name', '') AS "location_name",
        cast(ST_SetSRID(ST_GeomFromGeoJSON(nullif(exterior->'location_point', 'null'::jsonb)), 4326) AS geography) AS "location_point",
        cast(ST_SetSRID(ST_GeomFromGeoJSON(nullif(exterior->'location_polygon', 'null'::jsonb)), 4326) AS geography) AS "location_polygon",
        nullif(exterior->>'address_street', '') AS "address_street",
        nullif(exterior->>'address_city', '') AS "address_city",
        nullif(exterior->>'address_state', '') AS "address_state",
        nullif(exterior->>'address_country', '') AS "address_country",
        nullif(exterior->>'address_zip', '') AS "address_zip"
    FROM
        c_opportunity_overlay_v1
    ON CONFLICT
        DO NOTHING
;

WITH old_tags AS (
    SELECT
        c."opportunity_id" AS "opportunity_id",
        t AS "tag"
    FROM
        c_opportunity_overlay_v1 AS c,
        jsonb_array_elements_text(c.exterior->'tags') AS t
)
INSERT
    INTO c_opportunity_tag (
        "opportunity_id",
        "overlay",
        "tag"
    )
    SELECT
        o."opportunity_id",
        true,
        lower(o."tag") AS "tag"
    FROM
        old_tags o
    ON CONFLICT
        DO NOTHING
;

WITH old_topics AS (
    SELECT
        c."opportunity_id" AS "opportunity_id",
        t AS "topic"
    FROM
        c_opportunity_overlay_v1 AS c,
        jsonb_array_elements_text(c.exterior->'opp_topics') AS t
)
INSERT
    INTO c_opportunity_topic (
        "opportunity_id",
        "overlay",
        "topic"
    )
    SELECT
        o."opportunity_id",
        true,
        o."topic"::T_Topic AS "topic"
    FROM
        old_topics o
    ON CONFLICT
        DO NOTHING
;

WITH old_descriptors AS (
    SELECT
        c."opportunity_id" AS "opportunity_id",
        d AS "descriptor"
    FROM
        c_opportunity_overlay_v1 AS c,
        jsonb_array_elements_text(c.exterior->'opp_descriptor') AS d
)
INSERT
    INTO c_opportunity_descriptor (
        "opportunity_id",
        "overlay",
        "descriptor"
    )
    SELECT
        o."opportunity_id",
        true,
        o."descriptor"::T_Descriptor AS "descriptor"
    FROM
        old_descriptors o
    ON CONFLICT
        DO NOTHING
;

COMMIT;
