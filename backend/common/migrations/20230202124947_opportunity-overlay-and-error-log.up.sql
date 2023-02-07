create table c_partner_error_log (
       id bigserial primary key,
       partner_id integer not null references c_partner(id),
       "when" timestamptz not null default current_timestamp,
       "title" text,
       "raw" text,
       "level" smallint not null, -- 0 debug, 1 warning, 2 error
       "message" text not null
);

create index c_partner_error_log_by_partner on c_partner_error_log(partner_id);

create table c_opportunity_overlay (
       opportunity_id integer primary key references c_opportunity(id),
       exterior jsonb not null,
       interior jsonb not null
);
