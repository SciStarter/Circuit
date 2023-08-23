create table c_json_cache (
       "key" varchar(32) PRIMARY KEY,
       "value" jsonb NOT NULL,
       "when" timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);
