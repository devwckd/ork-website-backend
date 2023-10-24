-- Add migration script here

CREATE TABLE regions
(
    id      UUID PRIMARY KEY,
    slug    VARCHAR NOT NULL UNIQUE,
    api_url VARCHAR NOT NULL,
    cert    BYTEA   NOT NULL
);

INSERT INTO regions(id, slug, api_url, cert)
VALUES ('8c4144a5-4340-4169-b1cb-e8f963022a0c'::uuid, 'us-west', '', ''::bytea)