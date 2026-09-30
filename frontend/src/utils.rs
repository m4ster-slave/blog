use pulldown_cmark::{html, Options, Parser};
use reqwasm::http::Request;

pub fn confirm(message: &str) -> bool {
    web_sys::window()
        .and_then(|window| window.confirm_with_message(message).ok())
        .unwrap_or(false)
}

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

pub fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
