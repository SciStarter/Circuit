begin;

create table c_views (
    "id" serial primary key,  
    "when" timestamptz not null,
    "page" text not null,
    "user" integer references "c_person",
    "session" varchar(36) not null,
    "ip" varchar(54) not null,
    "lon" real,
    "lat" real
);

create table c_ip_coords (
    "ip" varchar(54) primary key,
    "lon" real,
    "lat" real
);

commit;
