create table c_widget_views (
   "id" serial primary key,
   "at" timestamptz not null default NOW(),
   "site" text not null
);

create index c_widget_views_by_at on c_widget_views ("at");
create index c_widget_views_by_site on c_widget_views ("site");
