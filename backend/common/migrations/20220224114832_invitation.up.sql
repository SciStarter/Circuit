create table "c_invitation" (
  "uid" UUID not null,
  "target" UUID not null,
  "mode" varchar(32) not null
);

create unique index "c_invitation_by_uid" on "c_invitation" ("uid");
