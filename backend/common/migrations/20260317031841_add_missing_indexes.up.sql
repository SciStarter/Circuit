-- High impact indexes

-- Search queries filter by partner on every partner-scoped search
CREATE INDEX c_opportunity_search_partner ON c_opportunity_search (partner);

-- all_by_participant queries ORDER BY updated DESC
CREATE INDEX c_involvement_by_updated ON c_involvement (updated);

-- Dynamic search EXISTS subqueries: participant = $n AND mode >= $n
-- Replaces the single-column c_involvement_by_participant index
DROP INDEX IF EXISTS c_involvement_by_participant;
CREATE INDEX c_involvement_by_participant_and_mode ON c_involvement (participant, mode);

-- Collator click-count queries join c_log on object with action='external' and time range
CREATE INDEX c_log_external_by_object_and_when ON c_log (object, "when") WHERE action = 'external';

-- Medium impact indexes

-- Collator hosts queries filter and group by organization_name
CREATE INDEX c_opportunity_organization_name ON c_opportunity (organization_name);

-- all_participation_between filters on latest >= $2 AND latest <= $3
CREATE INDEX c_involvement_by_latest ON c_involvement (latest);

-- Overview collator counts new users by created date range
CREATE INDEX c_person_by_created ON c_person (created);

-- Traffic chart queries filter and group by date
CREATE INDEX c_analytics_cache_by_date ON c_analytics_cache (date);

-- Lower impact indexes

-- exchanges.sql EXISTS subquery: join_channel = 'Exchange' AND join_channel_detail = uid
CREATE INDEX c_person_by_exchange_channel ON c_person (join_channel_detail) WHERE join_channel = 'Exchange';
