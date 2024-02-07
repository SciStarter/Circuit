BEGIN;

CREATE TABLE c_misc_cache (
    "key" VARCHAR(32) PRIMARY KEY,
    "when" TIMESTAMPTZ NOT NULL,
    "data" TEXT NOT NULL
);

COMMIT;
