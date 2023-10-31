-- Add migration script here

CREATE TABLE proxies
(
    id              UUID PRIMARY KEY,
    slug            VARCHAR NOT NULL,

    bridge_id       UUID,
    bs_proxy_id     UUID,

    template_id     UUID    NOT NULL,
    organization_id UUID    NOT NULL,

    /* TODO: ADD POD INFO STUFF */

    CONSTRAINT fk_template_id
        FOREIGN KEY (template_id)
            REFERENCES proxy_templates (id),

    CONSTRAINT fk_organization_id
        FOREIGN KEY (organization_id)
            REFERENCES organizations (id),

    CONSTRAINT unique_proxy_slug_per_organization
        UNIQUE (slug, organization_id)
)