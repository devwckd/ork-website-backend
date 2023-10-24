-- Add migration script here

CREATE TABLE sessions
(
    id         UUID PRIMARY KEY,
    user_id    UUID      NOT NULL,
    expires_at TIMESTAMP NOT NULL,


    CONSTRAINT fk_user_id
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE CASCADE
)