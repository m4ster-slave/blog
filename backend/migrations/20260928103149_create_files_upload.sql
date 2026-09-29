CREATE TABLE files (
    id          UUID PRIMARY KEY,
    sha256      TEXT NOT NULL UNIQUE,
    mime        TEXT NOT NULL,          
    kind        varchar(5) NOT NULL,          
    bytes       BIGINT NOT NULL,
    original_name TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
    CONSTRAINT chk_asset_type CHECK (kind IN ('image', 'file'))
);
