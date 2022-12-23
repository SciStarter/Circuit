create table c_log (
       "when" timestamptz not null default NOW(),
       "action" varchar(16) not null default '',
       "subject" uuid,
       "object" uuid
);

create index c_log_by_subject on c_log ("subject") where "subject" is not null;
create index c_log_by_object on c_log ("object") where "object" is not null;
