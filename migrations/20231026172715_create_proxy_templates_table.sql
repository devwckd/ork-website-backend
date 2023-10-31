-- Add migration script here

CREATE TABLE proxy_templates
(
    id              UUID PRIMARY KEY,
    slug            VARCHAR NOT NULL,

    image           VARCHAR NOT NULL,
    plugins_dir     VARCHAR NOT NULL,

    bridge_id       UUID,
    organization_id UUID    NOT NULL,


    CONSTRAINT fk_bridge_id
        FOREIGN KEY (bridge_id)
            REFERENCES bridges (id)
            ON DELETE SET NULL,

    CONSTRAINT fk_organization_id
        FOREIGN KEY (organization_id)
            REFERENCES organizations (id)
            ON DELETE CASCADE,

    CONSTRAINT unique_proxy_template_slug_per_organization
        UNIQUE (organization_id, slug)
)