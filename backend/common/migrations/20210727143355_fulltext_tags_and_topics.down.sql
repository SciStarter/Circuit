drop index if exists c_opportunity_fulltext_english;
create index c_opportunity_fulltext_english on c_opportunity using GIN (to_tsvector('english', (exterior -> 'title')::text || ' ' || (exterior -> 'description')::text));
alter table c_opportunity drop column if exists fulltext_english;
