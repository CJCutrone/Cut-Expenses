CREATE SCHEMA "User";

CREATE TABLE "credential_types" (
    "id" UUID PRIMARY KEY,
    "code" VARCHAR(16) NOT NULL,
    "name" VARCHAR(16) NOT NULL
);