begin;

create unique index c_opportunity_by_slug on c_opportunity (lower(exterior ->> 'slug'));

commit;
