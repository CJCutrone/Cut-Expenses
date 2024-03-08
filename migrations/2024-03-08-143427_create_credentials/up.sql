CREATE TABLE "credentials" (
   "id" UUID PRIMARY KEY,
   "credentialTypeId" UUID NOT NULL REFERENCES "credential_types" ("id"),
   "userId" UUID NOT NULL REFERENCES "users" ("id"),
   "salt" VARCHAR(255) NOT NULL,
   "hashed" VARCHAR(255) NOT NULL,
   "token" VARCHAR(255) NOT NULL,
   "createdAt" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);