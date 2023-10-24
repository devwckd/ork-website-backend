-- Add migration script here

CREATE TABLE proxy_templates
(
    id              UUID PRIMARY KEY,
    slug            VARCHAR NOT NULL,
    image           VARCHAR NOT NULL,
    plugins_dir     VARCHAR NOT NULL,
    organization_id UUID    NOT NULL,

    CONSTRAINT unique_org_slug
        UNIQUE (organization_id, slug),

    CONSTRAINT fk_organization_id
        FOREIGN KEY (organization_id)
            REFERENCES organizations (id)
            ON DELETE CASCADE
)