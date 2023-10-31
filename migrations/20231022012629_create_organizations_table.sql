-- Add migration script here

CREATE TABLE organizations
(
    id        UUID PRIMARY KEY,
    slug      VARCHAR NOT NULL,
    region_id UUID    NOT NULL,

    CONSTRAINT fk_region_id
        FOREIGN KEY (region_id)
            REFERENCES regions (id)
            ON DELETE SET NULL,

    CONSTRAINT unique_organization_slug
        UNIQUE (slug)

)
