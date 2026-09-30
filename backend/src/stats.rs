use crate::AppState;
use axum::{
    Json,
    body::Body,
    extract::{MatchedPath, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::NaiveDate;
use chrono::Utc;
use serde::Serialize;
use sqlx::PgPool;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::error;

#[derive(Default, Clone, Copy)]
pub struct DailyCounters {
    pub requests: i64,
    pub page_views: i64,
}

#[derive(Clone, Default)]
pub struct RequestStats {
    counters: Arc<Mutex<HashMap<NaiveDate, DailyCounters>>>,
}

impl RequestStats {
    pub fn record_request(&self, date: NaiveDate) {
        let mut counters = self.counters.lock().unwrap();

        counters.entry(date).or_default().requests += 1;
    }

    pub fn record_page_view(&self, date: NaiveDate) {
        let mut counters = self.counters.lock().unwrap();

        counters.entry(date).or_default().page_views += 1;
    }

    pub fn take_all(&self) -> HashMap<NaiveDate, DailyCounters> {
        let mut counters = self.counters.lock().unwrap();

        std::mem::take(&mut *counters)
    }

    pub fn restore(&self, mut failed: HashMap<NaiveDate, DailyCounters>) {
        let mut counters = self.counters.lock().unwrap();

        for (date, counts) in failed.drain() {
            let current = counters.entry(date).or_default();

            current.requests += counts.requests;
            current.page_views += counts.page_views;
        }
    }
}

pub async fn flush(pool: &PgPool, counters: &RequestStats) {
    let batch = counters.take_all();

    if batch.is_empty() {
        return;
    }

    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(error) => {
            error!(error = %error, "stats transaction failed");
            counters.restore(batch);
            return;
        }
    };

    for (date, counts) in &batch {
        let result = sqlx::query(
            r#"
            INSERT INTO site_stats_daily (
                date,
                requests,
                page_views
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (date)
            DO UPDATE SET
                requests = site_stats_daily.requests
                    + EXCLUDED.requests,
                page_views = site_stats_daily.page_views
                    + EXCLUDED.page_views
            "#,
        )
        .bind(*date)
        .bind(counts.requests)
        .bind(counts.page_views)
        .execute(&mut *tx)
        .await;

        if let Err(error) = result {
            error!(error = %error, "stats flush failed");

            let _ = tx.rollback().await;
            counters.restore(batch);
            return;
        }
    }

    if let Err(error) = tx.commit().await {
        error!(error = %error, "stats commit failed");
        counters.restore(batch);
    }
}

pub async fn request_middleware(
    State(state): State<Arc<AppState>>,
    matched_path: Option<MatchedPath>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let route = matched_path.as_ref().map(|p| p.as_str()).unwrap_or("");

    // Exclude stats, metrics and health endpoints.
    let excluded = matches!(route, "/stats" | "/health" | "/metrics");

    if !excluded {
        let today = Utc::now().date_naive();

        state.request_stats.record_request(today);
    }

    let response = next.run(request).await;

    if !excluded && response.status().is_success() && matches!(route, "/posts" | "/posts/{slug}") {
        let today = Utc::now().date_naive();

        state.request_stats.record_page_view(today);
    }

    response
}

#[derive(Serialize)]
pub struct StatsTotals {
    pub requests: i64,
    pub page_views: i64,
    pub posts_created: i64,
    pub posts_published: i64,
    pub devlog_entries_created: i64,
}

#[derive(Serialize)]
pub struct ContentStats {
    pub published_posts: i64,
    pub drafts: i64,
    pub devlog_entries: i64,
}

#[derive(Serialize)]
pub struct DailyStats {
    pub date: chrono::NaiveDate,
    pub requests: i64,
    pub page_views: i64,
    pub posts_created: i64,
    pub posts_published: i64,
    pub devlog_entries_created: i64,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub totals: StatsTotals,
    pub content: ContentStats,
    pub daily: Vec<DailyStats>,
}

pub async fn stats_aggregator(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<StatsResponse>), StatusCode> {
    let pool = &state.pool;
    flush(pool, &state.request_stats).await;

    // content stats summary
    let content = sqlx::query_as::<_, (i64, i64)>(
        r#"
        SELECT 
            COUNT(CASE WHEN published_at IS NOT NULL AND NOT archived THEN 1 END) as published_posts,
            COUNT(CASE WHEN published_at IS NULL AND NOT archived THEN 1 END) as drafts
        FROM posts
        "#,
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        error!(error = %error, status = 500, "failed to fetch content statistics");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let devlog_entries_count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM devlog_entries
        "#,
    )
    .fetch_one(pool)
    .await
    .map_err(|error| {
        error!(error = %error, status = 500, "failed to fetch devlog statistics");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let content_stats = ContentStats {
        published_posts: content.0,
        drafts: content.1,
        devlog_entries: devlog_entries_count.0,
    };

    // 2. Fetch daily traffic metrics combined with daily content creation counts
    let daily_rows = sqlx::query_as::<_, (NaiveDate, i64, i64, i64, i64, i64)>(
        r#"
        WITH daily_posts AS (
            SELECT 
                DATE(created_at) as date,
                COUNT(*) as posts_created,
                COUNT(CASE WHEN published_at IS NOT NULL THEN 1 END) as posts_published
            FROM posts
            GROUP BY DATE(created_at)
        ),
        daily_devlog AS (
            SELECT 
                DATE(created_at) as date,
                COUNT(*) as devlog_entries_created
            FROM devlog_entries
            GROUP BY DATE(created_at)
        ),
        all_dates AS (
            SELECT date FROM site_stats_daily
            UNION
            SELECT date FROM daily_posts
            UNION
            SELECT date FROM daily_devlog
        )
        SELECT 
            ad.date,
            COALESCE(s.requests, 0) as requests,
            COALESCE(s.page_views, 0) as page_views,
            COALESCE(p.posts_created, 0) as posts_created,
            COALESCE(p.posts_published, 0) as posts_published,
            COALESCE(d.devlog_entries_created, 0) as devlog_entries_created
        FROM all_dates ad
        LEFT JOIN site_stats_daily s ON ad.date = s.date
        LEFT JOIN daily_posts p ON ad.date = p.date
        LEFT JOIN daily_devlog d ON ad.date = d.date
        ORDER BY ad.date DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|error| {
        error!(error = %error, status = 500, "failed to fetch daily statistics");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let daily: Vec<DailyStats> = daily_rows
        .into_iter()
        .map(
            |(
                date,
                requests,
                page_views,
                posts_created,
                posts_published,
                devlog_entries_created,
            )| {
                DailyStats {
                    date,
                    requests,
                    page_views,
                    posts_created,
                    posts_published,
                    devlog_entries_created,
                }
            },
        )
        .collect();

    // 3. Compute overall totals
    let mut total_requests = 0;
    let mut total_page_views = 0;
    let mut total_posts_created = 0;
    let mut total_posts_published = 0;
    let mut total_devlog_entries_created = 0;

    for d in &daily {
        total_requests += d.requests;
        total_page_views += d.page_views;
        total_posts_created += d.posts_created;
        total_posts_published += d.posts_published;
        total_devlog_entries_created += d.devlog_entries_created;
    }

    let totals = StatsTotals {
        requests: total_requests,
        page_views: total_page_views,
        posts_created: total_posts_created,
        posts_published: total_posts_published,
        devlog_entries_created: total_devlog_entries_created,
    };

    Ok((
        StatusCode::OK,
        Json(StatsResponse {
            totals,
            content: content_stats,
            daily,
        }),
    ))
}
