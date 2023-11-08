BEGIN;

DROP INDEX "c_opportunity_fulltext_english";

DROP FUNCTION "c_opportunity_tsvector";

DROP INDEX c_opportunity_hashtag_by_opportunity_id;
DROP INDEX c_opportunity_hashtag_by_hashtag;
DROP INDEX c_opportunity_descriptor_by_opportunity_id;
DROP INDEX c_opportunity_descriptor_by_descriptor;
DROP INDEX c_opportunity_topic_by_opportunity_id;
DROP INDEX c_opportunity_topic_by_topic;
DROP INDEX c_opportunity_tag_by_tag;
DROP INDEX c_opportunity_tag_by_opportunity_id;
DROP INDEX c_opportunity_instance_by_opportunity_id;

DROP INDEX c_opportunity_by_location_polygon;
DROP INDEX c_opportunity_by_location_point;
DROP INDEX c_opportunity_by_location_type;
DROP INDEX c_opportunity_by_cost;
DROP INDEX c_opportunity_by_is_online;
DROP INDEX c_opportunity_by_organization_name;
DROP INDEX c_opportunity_by_partner;

DROP TRIGGER c_opportunity_interior_set_updated;
DROP TRIGGER c_opportunity_set_updated;

DROP TABLE "c_opportunity_tag";
DROP TABLE "c_opportunity_topic";
DROP TABLE "c_opportunity_descriptor";
DROP TABLE "c_opportunity_hashtag";
DROP TABLE "c_opportunity_instance";
DROP TABLE "c_opportunity_interior";
DROP TABLE "c_opportunity";

DROP TYPE T_ReviewStatus;
DROP TYPE T_Descriptor;
DROP TYPE T_Topic;
DROP TYPE T_LocationType;
DROP TYPE T_Cost;
DROP TYPE T_Recurrence;
DROP TYPE T_PESDomain;
DROP TYPE T_EntityType;
DROP TYPE T_OrganizationType;

ALTER TABLE "c_opportunity_v1" RENAME TO "c_opportunity";
ALTER TABLE "c_opportunityoverlay_v1" RENAME TO "c_opportunity_overlay";

CREATE INDEX c_opportunity_by_location_type USING gin (((exterior ->> 'location_type'::text)));
CREATE INDEX c_opportunity_withdrawn ON c_opportunity USING gin (((interior -> 'withdrawn'::text)));
CREATE INDEX c_opportunity_uid ON c_opportunity USING gin (((exterior -> 'uid'::text)));
CREATE INDEX c_opportunity_tags ON c_opportunity USING gin (((exterior -> 'tags'::text)));
CREATE INDEX c_opportunity_start_dates ON c_opportunity USING gin (((exterior -> 'end_dates'::text)));
CREATE INDEX c_opportunity_partner ON c_opportunity USING gin (((exterior -> 'partner'::text)));
CREATE INDEX c_opportunity_organization_name ON c_opportunity USING gin (((exterior -> 'organization_name'::text)));
CREATE INDEX c_opportunity_min_domain ON c_opportunity USING gin (((exterior -> 'pes_domain'::text)));
CREATE INDEX c_opportunity_min_age ON c_opportunity USING gin (((exterior -> 'min_age'::text)));
CREATE INDEX c_opportunity_max_age ON c_opportunity USING gin (((exterior -> 'max_age'::text)));
CREATE INDEX c_opportunity_is_online ON c_opportunity USING gin (((exterior -> 'is_online'::text)));
CREATE INDEX c_opportunity_fulltext_english ON c_opportunity USING gin (fulltext_english);
CREATE INDEX c_opportunity_end_dates ON c_opportunity USING gin (((exterior -> 'start_dates'::text)));
CREATE INDEX c_opportunity_cost ON c_opportunity USING gin (((exterior -> 'cost'::text)));
CREATE UNIQUE INDEX c_opportunity_by_slug ON c_opportunity USING btree (lower((exterior ->> 'slug'::text))) WHERE (NOT ((exterior ->> 'slug'::text) = ''::text));

COMMIT;
