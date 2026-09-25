CREATE TABLE site_stats_daily (
    date DATE PRIMARY KEY DEFAULT CURRENT_DATE,

    -- HTTP traffic
    requests BIGINT NOT NULL DEFAULT 0,
    page_views BIGINT NOT NULL DEFAULT 0,

    -- Ensure counters cannot become negative
    CONSTRAINT site_stats_daily_nonnegative CHECK (
        requests >= 0
        AND page_views >= 0
    )
);
