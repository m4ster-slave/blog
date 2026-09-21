use reqwasm::http::Request;

pub async fn is_admin() -> Result<bool, String> {
    let resp = Request::get("/api/admin/check")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    match resp.status() {
        200 => Ok(true),
        _ => Ok(false),
    }
}
