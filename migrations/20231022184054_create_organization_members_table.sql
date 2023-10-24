-- Add migration script here

CREATE TABLE organization_members
(
    user_id         UUID     NOT NULL,
    organization_id UUID     NOT NULL,
    role            SMALLINT NOT NULL DEFAULT 0,

    CONSTRAINT fk_user_id
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE CASCADE,

    CONSTRAINT fk_organization_id
        FOREIGN KEY (organization_id)
            REFERENCES organizations (id)
            ON DELETE CASCADE
)