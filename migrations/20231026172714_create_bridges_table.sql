-- Add migration script here

CREATE TABLE bridges
(
    id              UUID PRIMARY KEY,
    slug            VARCHAR NOT NULL,
    bs_namespace_id UUID    NOT NULL, /* bridge service id */
    organization_id UUID    NOT NULL,

    CONSTRAINT fk_organization_id
        FOREIGN KEY (organization_id)
            REFERENCES organizations (id)
            ON DELETE SET NULL,

    CONSTRAINT unique_bridge_slug_per_organization
        UNIQUE (slug, organization_id)
)