use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos_chartistry::*;
use leptos_meta::{Meta, Title};
use reqwasm::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Response};

#[component]
fn ViewsChart(daily: Vec<DailyStats>) -> impl IntoView {
    let mut sorted = daily;
    sorted.reverse();
    let data = RwSignal::new(sorted);

    let series = Series::new(|d: &DailyStats| d.date.and_hms_opt(0, 0, 0).unwrap().and_utc())
        .line(
            Line::new(|d: &DailyStats| d.page_views as f64)
                .with_name("Page Views")
                .with_colour(Colour::from_rgb(0x4a, 0x9e, 0xff)),
        )
        .line(
            Line::new(|d: &DailyStats| d.requests as f64)
                .with_name("Requests")
                .with_colour(Colour::from_rgb(0xff, 0x6b, 0x6b)),
        );

    view! {
        <Chart
            aspect_ratio=AspectRatio::from_env_width(300.0)
            series=series
            data=data
            top=RotatedLabel::middle("Daily Activity")
            left=TickLabels::aligned_floats()
            bottom=TickLabels::timestamps()
            inner=[
                AxisMarker::left_edge().into(),
                AxisMarker::bottom_edge().into(),
                XGridLine::default().into(),
                YGridLine::default().into(),
            ]
            tooltip=Tooltip::left_cursor()
        />
    }
}

#[derive(Deserialize, Clone)]
pub struct StatsTotals {
    pub requests: i64,
    pub page_views: i64,
    pub posts_created: i64,
    pub posts_published: i64,
    pub devlog_entries_created: i64,
}

#[derive(Deserialize, Clone)]
pub struct ContentStats {
    pub published_posts: i64,
    pub drafts: i64,
    pub devlog_entries: i64,
}

#[derive(Deserialize, Clone)]
pub struct DailyStats {
    pub date: chrono::NaiveDate,
    pub requests: i64,
    pub page_views: i64,
    pub posts_created: i64,
    pub posts_published: i64,
    pub devlog_entries_created: i64,
}

#[derive(Deserialize, Clone)]
pub struct StatsResponse {
    pub totals: StatsTotals,
    pub content: ContentStats,
    pub daily: Vec<DailyStats>,
}

async fn fetch_stats() -> Result<StatsResponse, String> {
    let resp = Request::get("/api/stats")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let json = resp.text().await.map_err(|e| e.to_string())?;
    let stats: StatsResponse = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(stats)
}

async fn get_wasm_size() -> Option<u64> {
    let win = window()?;
    let resp: Response = JsFuture::from(win.fetch_with_str("/blog_bg.wasm"))
        .await
        .ok()?
        .dyn_into()
        .ok()?;
    resp.headers()
        .get("content-length")
        .ok()
        .flatten()
        .and_then(|s| s.parse().ok())
}

#[component]
pub fn Stats() -> impl IntoView {
    let wasm_size = LocalResource::new(get_wasm_size);
    let stats = LocalResource::new(fetch_stats);

    view! {
        <Title text="Site Stats | Lukiana's Blog" />
        <Meta name="description" content="View technical and usage statistics for Lukiana's blog." />
        <ErrorBoundary fallback=|errors| {
            view! {
                <div class="stats-error">
                    <h1>"Uh oh! Something went wrong!"</h1>
                    <p>"Errors: "</p>
                    <ul>
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect_view()
                        }}
                    </ul>
                </div>
            }
        }>
            <div class="stats-container">
                <div class="stats-header">
                    <h1>"Site Statistics"</h1>
                </div>

                // Bundle Size Telemetry Bar
                <div class="stats-telemetry-bar">
                    <div class="telemetry-item">
                        <span class="telemetry-label">"WASM Bundle Size"</span>
                        <span class="telemetry-value">
                            {move || {
                                wasm_size
                                    .get()
                                    .map(|opt| match opt {
                                        Some(bytes) => format!("{:.2} KB ({} bytes)", bytes as f64 / 1024.0, bytes),
                                        None => "unavailable".to_string(),
                                    })
                                    .unwrap_or_else(|| "measuring…".to_string())
                            }}
                        </span>
                    </div>
                </div>

                {move || {
                    match stats.get() {
                        None => view! { <div class="stats-loading">"Loading statistics..."</div> }.into_any(),
                        Some(Err(err)) => view! { <div class="stats-error">"Failed to load stats: " {err}</div> }.into_any(),
                        Some(Ok(data)) => {
                            let totals = data.totals.clone();
                            let content = data.content.clone();
                            let daily = data.daily.clone();

                            view! {
                                <div class="stats-content">
                                    <div class="stats-section-title"><h2>"requests"</h2></div>
                                    <div class="stats-grid">
                                        <div class="stat-card">
                                            <span class="stat-value">{totals.requests}</span>
                                            <span class="stat-label">"Total Requests"</span>
                                        </div>
                                        <div class="stat-card">
                                            <span class="stat-value">{totals.page_views}</span>
                                            <span class="stat-label">"Page Views"</span>
                                        </div>
                                        <div class="stat-card">
                                            <span class="stat-value">{daily[0].requests}</span>
                                            <span class="stat-label">"Requests Today"</span>
                                        </div>
                                    </div>

                                    <div class="stats-section-title"><h2>"content"</h2></div>
                                    <div class="stats-grid content-grid">
                                        <div class="stat-card">
                                            <span class="stat-value">{content.published_posts}</span>
                                            <span class="stat-label">"Published Articles"</span>
                                        </div>
                                        <div class="stat-card">
                                            <span class="stat-value">{totals.posts_created}</span>
                                            <span class="stat-label">"Posts Created"</span>
                                        </div>
                                        <div class="stat-card">
                                            <span class="stat-value">{content.drafts}</span>
                                            <span class="stat-label">"Drafts in Progress"</span>
                                        </div>
                                        <div class="stat-card">
                                            <span class="stat-value">{content.devlog_entries}</span>
                                            <span class="stat-label">"Total Devlogs"</span>
                                        </div>
                                    </div>

                                    <div class="stats-section-title"><h2>"daily graph"</h2></div>
                                    <div class="stats-graph">
                                        <ViewsChart daily=daily.clone()/>

                                    </div>


                                    <div class="stats-section-title"><h2>"daily activity"</h2></div>
                                    <div class="stats-table-wrapper">
                                        <table class="stats-table">
                                            <thead>
                                                <tr>
                                                    <th>"Date"</th>
                                                    <th>"Requests"</th>
                                                    <th>"Page Views"</th>
                                                    <th>"Posts Created"</th>
                                                    <th>"Published"</th>
                                                    <th>"Devlogs"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {daily.into_iter().take(5).map(|day| {
                                                    view! {
                                                        <tr>
                                                            <td>{day.date.to_string()}</td>
                                                            <td>{day.requests}</td>
                                                            <td>{day.page_views}</td>
                                                            <td>{day.posts_created}</td>
                                                            <td>{day.posts_published}</td>
                                                            <td>{day.devlog_entries_created}</td>
                                                        </tr>
                                                    }
                                                }).collect_view()}
                                            </tbody>
                                        </table>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }
                }}
            </div>
        </ErrorBoundary>
    }
}
