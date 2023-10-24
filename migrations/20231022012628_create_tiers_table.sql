-- Add migration script here

CREATE TABLE tiers
(
    id        UUID PRIMARY KEY,
    slug      VARCHAR NOT NULL UNIQUE,
    region_id UUID    NOT NULL,
    tag       VARCHAR NOT NULL,

    CONSTRAINT fk_region_id
        FOREIGN KEY (region_id)
            REFERENCES regions (id)
            ON DELETE CASCADE
);

INSERT INTO tiers(id, slug, region_id, tag)
VALUES (gen_random_uuid(), 'r9-7900x', '8c4144a5-4340-4169-b1cb-e8f963022a0c'::uuid, '')