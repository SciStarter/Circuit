insert into c_opportunity_import_record ("partner", "opportunity", "created", "ignored")
values ($1, $2, $3, $4)
returning "id", "when", "partner", "opportunity", "created", "ignored";
