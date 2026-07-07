ALTER TABLE chat_rooms ADD COLUMN tickets_enabled BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE tickets (
    id           UUID        PRIMARY KEY DEFAULT uuidv7(),
    created      TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated      TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    room_id      UUID        NOT NULL REFERENCES chat_rooms(id) ON DELETE CASCADE,
    submitter_id UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title        TEXT        NOT NULL CHECK(length(title) BETWEEN 1 AND 100),
    description  TEXT        NOT NULL CHECK(length(description) BETWEEN 1 AND 2000),
    categories   TEXT[]      NOT NULL DEFAULT '{}',
    priority     TEXT        CHECK(priority IN ('very_low','low','medium','high','very_high','urgent')),
    status       TEXT        NOT NULL DEFAULT 'open' CHECK(status IN ('open','closed'))
);

CREATE INDEX tickets_room_status ON tickets(room_id, status, created DESC);
CREATE INDEX tickets_submitter   ON tickets(submitter_id, created DESC);
