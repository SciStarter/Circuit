begin;

alter table c_opportunity
add column fulltext_english tsvector
generated always as
(to_tsvector('english', exterior ->> 'title') ||
 to_tsvector('english', exterior ->> 'description') ||
 to_tsvector('english', exterior ->> 'partner_name') ||
 to_tsvector('english', exterior ->> 'organization_name') ||
 to_tsvector('english', exterior ->> 'location_name') ||
 to_tsvector('english', exterior ->> 'address_city') ||
 to_tsvector('english', exterior ->> 'address_state') ||
 to_tsvector('english', exterior ->> 'address_country') ||
 to_tsvector('english', exterior -> 'tags') ||
 to_tsvector('english', exterior -> 'opp_topics') ||
 to_tsvector('english', exterior -> 'opp_descriptor') ||
 to_tsvector('english', exterior -> 'opp_hashtags') ||
 to_tsvector('english', exterior -> 'opp_social_handles')
)
stored;

drop index if exists c_opportunity_fulltext_english;

create index c_opportunity_fulltext_english on c_opportunity using GIN (fulltext_english);

commit;
