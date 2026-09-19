CREATE TABLE devlog_entries(
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    content TEXT NOT NULL
);

CREATE INDEX devlog_entries_date_idx
    ON devlog_entries(created_at DESC);
