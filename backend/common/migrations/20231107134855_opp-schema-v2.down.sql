BEGIN;

DROP INDEX "c_opportunity_via_fulltext_english";

DROP FUNCTION "c_opportunity_tsvector";
DROP FUNCTION "c_opportunity_is_current";
DROP FUNCTION "c_opportunity_is_current_as_of";
DROP FUNCTION "c_opportunity_is_ondemand";
DROP FUNCTION "c_opportunity_is_scheduled";
DROP FUNCTION "c_opportunity_by_uid_likes_during";
DROP FUNCTION "c_opportunity_by_uid_is_status";
DROP FUNCTION "c_opportunity_by_uid_domain";
DROP FUNCTION "c_opportunity_by_uid_is_current_as_of";
DROP FUNCTION "c_opportunity_by_uid_is_current";
DROP FUNCTION IF EXISTS "c_opportunity_is_ongoing";

DROP INDEX "c_opportunity_hashtag_via_opportunity_id";
DROP INDEX "c_opportunity_hashtag_via_hashtag";
DROP INDEX "c_opportunity_venue_type_via_opportunity_id";
DROP INDEX "c_opportunity_venue_type_via_venue_type";
DROP INDEX "c_opportunity_descriptor_via_opportunity_id";
DROP INDEX "c_opportunity_descriptor_via_descriptor";
DROP INDEX "c_opportunity_topic_via_opportunity_id";
DROP INDEX "c_opportunity_topic_via_topic";
DROP INDEX "c_opportunity_tag_via_tag";
DROP INDEX "c_opportunity_tag_via_opportunity_id";
DROP INDEX "c_opportunity_instance_via_opportunity_id";

DROP INDEX "c_opportunity_via_location_polygon";
DROP INDEX "c_opportunity_via_location_point";
DROP INDEX "c_opportunity_via_location_type";
DROP INDEX "c_opportunity_via_cost";
DROP INDEX "c_opportunity_via_is_online";
DROP INDEX "c_opportunity_via_organization_name";
DROP INDEX "c_opportunity_via_partner";

DROP TRIGGER c_opportunity_interior_set_updated ON c_opportunity_interior;
DROP TRIGGER c_opportunity_set_updated ON c_opportunity;

DROP TABLE "c_opportunity_tag";
DROP TABLE "c_opportunity_topic";
DROP TABLE "c_opportunity_descriptor";
DROP TABLE "c_opportunity_venue_type";
DROP TABLE "c_opportunity_hashtag";
DROP TABLE "c_opportunity_instance";
DROP TABLE "c_opportunity_interior";
DROP TABLE "c_opportunity_overlay";
DROP TABLE "c_opportunity";

DROP TYPE T_ReviewStatus;
DROP TYPE T_VenueType;
DROP TYPE T_Descriptor;
DROP TYPE T_Topic;
DROP TYPE T_LocationType;
DROP TYPE T_Cost;
DROP TYPE T_Recurrence;
DROP TYPE T_PESDomain;
DROP TYPE T_EntityType;
DROP TYPE T_OrganizationType;

ALTER TABLE "c_opportunity_overlay_v1" RENAME TO "c_opportunity_overlay";
ALTER TABLE "c_opportunity_v1" RENAME TO "c_opportunity";

CREATE FUNCTION c_opportunity_is_current_as_of(interior jsonb, exterior jsonb, stamp timestamptz) returns boolean as
$func$
BEGIN
 RETURN (
   coalesce(nullif(interior ->> 'review_status', ''), 'not_required') IN ('publish', 'not_required')
   AND
   interior ->> 'accepted' = 'true'
   AND
   interior ->> 'withdrawn' = 'false'
   AND
   (
       (
          jsonb_array_length(exterior -> 'start_datetimes') <= 1
          AND
          jsonb_array_length(exterior -> 'end_datetimes') = 0
       )
       OR
       EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'start_datetimes') WHERE value::timestamptz > stamp)
       OR
       EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'end_datetimes') WHERE value::timestamptz > stamp)
       OR
       (
          (exterior->>'recurrence' = 'daily' OR exterior->>'recurrence' = 'weekly')
          AND
          (exterior->>'end_recurrence' IS null OR (exterior->>'end_recurrence')::timestamptz > stamp)
       )
   )
 );
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_is_scheduled(interior jsonb, exterior jsonb) returns boolean as
$func$
BEGIN
 RETURN (
   jsonb_array_length(exterior -> 'start_datetimes') > 1
   OR
   jsonb_array_length(exterior -> 'end_datetimes') > 1
   OR (
     jsonb_array_length(exterior -> 'start_datetimes') = 1
     AND
     jsonb_array_length(exterior -> 'end_datetimes') = 1
     AND
     AGE((exterior #> '{end_datetimes,0}')::text::date, (exterior #> '{start_datetimes,0}')::text::date) <= interval '7 days'
   )
 );
END
$func$ language plpgsql stable;

CREATE FUNCTION c_opportunity_is_ondemand(interior jsonb, exterior jsonb) returns boolean as
$func$
BEGIN
 RETURN (
   jsonb_array_length(exterior -> 'start_datetimes') <= 1
   AND
   jsonb_array_length(exterior -> 'end_datetimes') <= 1
   AND (
     jsonb_array_length(exterior -> 'start_datetimes') != 1
     OR
     jsonb_array_length(exterior -> 'end_datetimes') != 1
     OR
     AGE((exterior #> '{end_datetimes,0}')::text::date, (exterior #> '{start_datetimes,0}')::text::date) > interval '7 days'
   )
 );
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_by_uid_likes_during("uid" uuid, "begin" timestamptz, "end" timestamptz) returns bigint as
$func$
DECLARE
 opp c_opportunity%ROWTYPE;
 val bigint;
BEGIN
 SELECT * INTO opp FROM c_opportunity WHERE ("exterior"->>'uid')::uuid = "uid" LIMIT 1;
 SELECT COALESCE(COUNT(*), 0) INTO val FROM c_opportunity_like WHERE "opportunity_id" = opp."id" AND "when" >= "begin" AND "when" < "end";
 RETURN val;
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_by_uid_is_status(uid uuid, status integer) returns boolean as
$func$
DECLARE
 opp c_opportunity%ROWTYPE;
BEGIN
 SELECT * INTO opp FROM c_opportunity WHERE ("exterior"->>'uid')::uuid = uid LIMIT 1;
 CASE status
  WHEN 2 THEN RETURN c_opportunity_is_current(opp.interior, opp.exterior) = false;
  WHEN 1 THEN RETURN c_opportunity_is_current(opp.interior, opp.exterior) = true;
  ELSE RETURN true;
 END CASE;
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_by_uid_domain(uid uuid) returns text as
$func$
DECLARE
 opp c_opportunity%ROWTYPE;
 partner c_partner%ROWTYPE;
BEGIN
 SELECT * INTO opp FROM c_opportunity WHERE ("exterior"->>'uid')::uuid = uid LIMIT 1;
 CASE opp."exterior"->>'pes_domain'
  WHEN 'unspecified' THEN
   SELECT * INTO partner FROM c_partner WHERE ("exterior"->'uid') = (opp."exterior"->'partner') LIMIT 1;
   RETURN partner."exterior"->>'pes_domain';
  ELSE RETURN opp."exterior"->>'pes_domain';
 END CASE;
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_is_current(interior jsonb, exterior jsonb) returns boolean as
$func$
BEGIN
 RETURN c_opportunity_is_current_as_of(interior, exterior, CURRENT_TIMESTAMP);
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_by_uid_is_current_as_of(uid uuid, stamp timestamptz) returns boolean as
$func$
DECLARE
 opp c_opportunity%ROWTYPE;
BEGIN
 SELECT * INTO opp FROM c_opportunity WHERE ("exterior"->>'uid')::uuid = uid LIMIT 1;
 RETURN c_opportunity_is_current_as_of(opp.interior, opp.exterior, stamp);
END
$func$ LANGUAGE plpgsql STABLE;

CREATE FUNCTION c_opportunity_by_uid_is_current(uid uuid) returns boolean as
$func$
DECLARE
 opp c_opportunity%ROWTYPE;
BEGIN
 SELECT * INTO opp FROM c_opportunity WHERE ("exterior"->>'uid')::uuid = uid LIMIT 1;
 RETURN c_opportunity_is_current(opp.interior, opp.exterior);
END
$func$ LANGUAGE plpgsql STABLE;

CREATE INDEX c_opportunity_location_type ON c_opportunity USING gin ((exterior -> 'location_type'::text));
CREATE INDEX c_opportunity_withdrawn ON c_opportunity USING gin ((interior -> 'withdrawn'::text));
CREATE INDEX c_opportunity_uid ON c_opportunity USING gin ((exterior -> 'uid'::text));
CREATE INDEX c_opportunity_tags ON c_opportunity USING gin ((exterior -> 'tags'::text));
CREATE INDEX c_opportunity_start_dates ON c_opportunity USING gin ((exterior -> 'end_dates'::text));
CREATE INDEX c_opportunity_partner ON c_opportunity USING gin ((exterior -> 'partner'::text));
CREATE INDEX c_opportunity_organization_name ON c_opportunity USING gin ((exterior -> 'organization_name'::text));
CREATE INDEX c_opportunity_min_domain ON c_opportunity USING gin ((exterior -> 'pes_domain'::text));
CREATE INDEX c_opportunity_min_age ON c_opportunity USING gin ((exterior -> 'min_age'::text));
CREATE INDEX c_opportunity_max_age ON c_opportunity USING gin ((exterior -> 'max_age'::text));
CREATE INDEX c_opportunity_is_online ON c_opportunity USING gin ((exterior -> 'is_online'::text));
CREATE INDEX c_opportunity_fulltext_english ON c_opportunity USING gin (fulltext_english);
CREATE INDEX c_opportunity_end_dates ON c_opportunity USING gin (((exterior -> 'start_dates'::text)));
CREATE INDEX c_opportunity_cost ON c_opportunity USING gin ((exterior -> 'cost'::text));
CREATE UNIQUE INDEX c_opportunity_by_slug ON c_opportunity USING btree (lower((exterior ->> 'slug'::text))) WHERE (NOT ((exterior ->> 'slug'::text) = ''::text));

COMMIT;
