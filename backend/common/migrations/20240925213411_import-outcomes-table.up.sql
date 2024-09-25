create table c_import_outcomes (
    "partner" text not null,
    "date" text not null,
    "added" integer not null,
    "updated" integer not null,
    "failed" integer not null
);

create unique index c_import_outcomes_by_partner_and_date on c_import_outcomes (
    "partner",
    "date"
);
