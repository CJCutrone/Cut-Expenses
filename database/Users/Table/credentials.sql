CREATE TABLE "credentials" (
    "id" UUID PRIMARY KEY,
    "credential_type_id" UUID NOT NULL REFERENCES "credential_types" ("id"),
    "user_id" UUID NOT NULL REFERENCES "users" ("id"),
    "salt" VARCHAR(255) NOT NULL,
    "hashed" VARCHAR(255) NOT NULL,
    "token" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);