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
    "near_longitude" float8,
    "near_latitude" float8,
    "near_distance" float8,
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
    "calendar_year" int4,
    "calendar_month" int4,
    "region" text
);

CREATE FUNCTION c_opportunity_distance(opp c_opportunity, "from" geography(POINT, 4326)) RETURNS float8
    LANGUAGE sql
    STABLE
    RETURN CASE
      WHEN "opp"."location_polygon" IS NOT NULL THEN CASE
        WHEN ST_CoveredBy("from", "opp"."location_polygon") THEN
          0
        ELSE
          ST_Distance("from", "opp"."location_polygon", false)
        END
      WHEN "opp"."location_point" IS NOT NULL THEN
        ST_Distance("from", "opp"."location_point")
      WHEN "opp"."location_type" = 'any' OR "opp"."is_online" THEN
        0
      ELSE
        -- This constant number is roughly the square root of the surface area of the earth, in meters, i.e. about as far away as you can get
        22585394        
      END
;

CREATE FUNCTION c_opportunity_locality(opp c_opportunity, "from" geography(POINT, 4326)) RETURNS float8
    LANGUAGE sql
    STABLE
    RETURN CASE
      WHEN "opp"."location_polygon" IS NOT NULL THEN CASE
        WHEN ST_CoveredBy("from", "opp"."location_polygon") THEN
          sqrt(ST_Area("opp"."location_polygon", false))
        ELSE
          ST_Distance("from", "opp"."location_polygon", false) + sqrt(ST_Area("opp"."location_polygon", false))
        END
      WHEN "opp"."location_point" IS NOT NULL THEN
        ST_Distance("from", "opp"."location_point")
      ELSE
        22585394        
      END
;

CREATE FUNCTION c_opportunity_until(opp c_opportunity, "from" timestamptz) RETURNS interval
    LANGUAGE sql
    STABLE
    RETURN CASE
      WHEN "opp"."recurrence" = 'weekly' AND ("opp"."end_recurrence" IS NULL OR "opp"."end_recurrence" > "from") THEN '4 days'::interval
      WHEN "opp"."recurrence" = 'daily'  AND ("opp"."end_recurrence" IS NULL OR "opp"."end_recurrence" > "from") THEN '2 days'::interval
      ELSE (
        SELECT min("i"."start")
        FROM c_opportunity_instance AS "i"
        WHERE
          "i"."opportunity_id" = "opp"."id"
          AND "i"."start" >= "from"
      ) - "from"
      END
;

CREATE FUNCTION c_opportunities_matching("query" T_OpportunityQuery)
    RETURNS TABLE ("opp" c_opportunity, "interior" c_opportunity_interior)
    STRICT
    STABLE
    PARALLEL SAFE
    LANGUAGE sql
    AS $func$
      SELECT
        "opp",
        "interior"
      FROM
        "c_opportunity" AS "opp"
        JOIN "c_opportunity_interior" AS "interior" ON "interior"."opportunity_id" = "opp"."id"
      WHERE (
        CASE WHEN "query"."uid" IS NOT NULL
          THEN "opp"."uid" = "query"."uid"
          ELSE true
        END
        AND CASE WHEN "query"."slug" IS NOT NULL
          THEN "opp"."slug" = "query"."slug"
          ELSE true
        END
        AND CASE WHEN "query"."accepted" IS NOT NULL
          THEN "interior"."accepted" = "query"."accepted"
          ELSE true
        END
        AND CASE WHEN "query"."withdrawn" IS NOT NULL
          THEN "interior"."withdrawn" = "query"."withdrawn"
          ELSE true
        END
        AND CASE WHEN "query"."region" IS NOT NULL
          THEN (
            SELECT coalesce(
              nullif(ST_Intersects(c_region."geometry", "opp"."location_point"), false),
              nullif(ST_Intersects(c_region."geometry", "opp"."location_polygon"), false),
              false
            ) FROM c_region WHERE c_region."name" = "query"."region"
          )
          ELSE true
        END
        AND CASE
          WHEN "query"."calendar_year" IS NOT NULL AND "query"."calendar_month" IS NOT NULL
            THEN 
              exists(SELECT 1 FROM c_opportunity_instance WHERE "opportunity_id" = "opp"."id" AND "start" > make_timestamptz("query"."calendar_year", "query"."calendar_month", 1, 0, 0, 0.0) AND "start" < make_timestamptz("query"."calendar_year", "query"."calendar_month", 1, 0, 0, 0.0) + interval '1 month')
              OR exists(SELECT 1 FROM c_opportunity_instance WHERE "opportunity_id" = "opp"."id" AND "end" > make_timestamptz("query"."calendar_year", "query"."calendar_month", 1, 0, 0, 0.0) AND "end" < make_timestamptz("query"."calendar_year", "query"."calendar_month", 1, 0, 0, 0.0) + interval '1 month')
              -- Not so sure this is a good idea
              -- OR coalesce(nullif("opp"."end_recurrence", ''), '0001-01-01')::timestamptz > make_timestamptz("query"."calendar_year", "query"."calendar_month", 1, 0, 0, 0.0)
          WHEN "query"."current" IS NOT NULL
            THEN c_opportunity_is_current(opp) = "query"."current"
          ELSE true
        END
        AND CASE WHEN "query"."involved" IS NOT NULL
          THEN exists(
            SELECT 1
            FROM c_involvement AS inv
            WHERE (inv.exterior ->> 'opportunity') = "opp"."uid"::text
              AND (inv.interior ->> 'participant') = "query"."involved"::text
              AND (inv.exterior ->> 'mode')::integer >= 10) -- 10 is Interest in the enum
          ELSE true
        END
        AND CASE WHEN "query"."saved" IS NOT NULL
          THEN exists(
            SELECT 1
            FROM c_involvement AS inv
            WHERE (inv.exterior ->> 'opportunity') = "opp"."uid"::text
              AND (inv.interior ->> 'participant') = "query"."saved"::text
              AND (inv.exterior ->> 'mode')::integer = 20) -- 20 is Saved in the enum
          ELSE true
        END
        AND CASE WHEN "query"."participated" IS NOT NULL
          THEN exists(
            SELECT 1
            FROM c_involvement AS inv
            WHERE (inv.exterior ->> 'opportunity') = "opp"."uid"::text
              AND (inv.interior ->> 'participant') = "query"."participated"::text
              AND (inv.exterior ->> 'mode')::integer >= 30) -- 30 is Logged in the enum
          ELSE true
        END
        AND CASE WHEN "query"."entity_type" IS NOT NULL
          THEN "opp"."entity_type" = any("query"."entity_type")
          ELSE true
        END
        AND CASE WHEN "query"."title_contains" IS NOT NULL
          THEN "opp"."title" ILIKE ('%' || "query"."title_contains" || '%')
          ELSE true
        END
        AND CASE WHEN "query"."tags" IS NOT NULL
          THEN exists(SELECT 1 FROM c_opportunity_tag WHERE "opportunity_id" = "opp"."id" AND "tag" = any("query"."tags"))
          ELSE true
        END
        AND CASE WHEN "query"."topics" IS NOT NULL
          THEN exists(SELECT 1 FROM c_opportunity_topic WHERE "opportunity_id" = "opp"."id" AND "topic" = any("query"."topics"))
          ELSE true
        END
        AND CASE WHEN "query"."descriptors" IS NOT NULL
          THEN exists(SELECT 1 FROM c_opportunity_descriptor WHERE "opportunity_id" = "opp"."id" AND "descriptor" = any("query"."descriptors"))
          ELSE true
        END
        AND CASE WHEN "query"."partner" IS NOT NULL
          THEN "opp"."partner" = "query"."partner"
          ELSE true
        END
        AND CASE WHEN "query"."partner_member" IS NOT NULL
          THEN "interior"."submitted_by" = "query"."partner_member"
            OR exists(SELECT 1 FROM c_partner WHERE (exterior->>'uid') = "opp"."partner"::text AND (interior->>'prime') = "query"."partner_member"::text)
            OR exists(SELECT 1 FROM c_partner, jsonb_array_elements_text(interior->'authorized') AS x(authed) WHERE (exterior->>'uid') = "opp"."partner"::text AND authed = "query"."partner_member"::text)
          ELSE true
        END
        AND CASE WHEN "query"."text" IS NOT NULL
          THEN c_opportunity_tsvector("opp") @@ websearch_to_tsquery("query"."text")
          ELSE true
        END
        AND CASE WHEN "query"."beginning" IS NOT NULL
          THEN
              NOT exists(SELECT 1 FROM c_opportunity_instance WHERE "opportunity_id" = "opp"."id")
              OR ("opp"."recurrence" != 'once' AND ("opp"."end_recurrence" IS NULL OR "opp"."end_recurrence" > "query"."beginning"))
              OR "query"."beginning" < any(SELECT "start" FROM c_opportunity_instance WHERE "opportunity_id" = "opp"."id")
              OR "query"."beginning" < any(SELECT "end" FROM c_opportunity_instance WHERE "opportunity_id" = "opp"."id" AND "end" IS NOT NULL)
          ELSE true
        END
        AND CASE WHEN "query"."ending" IS NOT NULL
          THEN 
              exists(SELECT 1 FROM c_opportunity_instance WHERE "opportunity_id" = "opp"."id")
              AND ("opp"."recurrence" = 'once' OR ("opp"."end_recurrence" IS NOT NULL AND "opp"."end_recurrence" < "query"."beginning"))
              AND "query"."ending" > all(SELECT "start" FROM c_opportunity_instance WHERE "opportunity_id" = "opp"."id")
              AND "query"."ending" > all(SELECT "end" FROM c_opportunity_instance WHERE "opportunity_id" = "opp"."id" AND "end" IS NOT NULL)
          ELSE true
        END
        -- Minimum and maximum age in queries each define a contraint on
        -- the opposite project field. A queried min age checks that the
        -- opportunity max age is greater than the query min age, and a
        -- queried max age checks that the opporuntity minimum is less
        -- than the queried minimum
        AND CASE WHEN "query"."min_age" IS NOT NULL
          THEN "opp"."max_age" >= "query"."min_age"
          ELSE true
        END
        AND CASE WHEN "query"."max_age" IS NOT NULL
          THEN "opp"."min_age" >= "query"."max_age"
          ELSE true
        END
        AND CASE WHEN "query"."kids_only" IS NOT NULL
          THEN !!!
          ELSE true
        END
        AND CASE WHEN "query"."near_distance" IS NOT NULL AND "query"."near_distance" > 0.001 AND "query"."near_longitude" IS NOT NULL AND "query"."near_latitude" IS NOT NULL
          THEN c_opportunity_distance("opp", ST_SetSRID(ST_Point("query"."near_longitude", "query"."near_latitude"), 4326)) < "query"."near_distance"
          ELSE true
        END
      )
    $func$
;

COMMIT;
