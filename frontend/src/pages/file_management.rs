use leptos::prelude::*;
use reqwasm::http::Request;
use std::time::Duration;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, HtmlInputElement};

use crate::models::file::FileEntry;

async fn fetch_entries() -> Result<Vec<FileEntry>, String> {
    let resp = Request::get("/api/admin/files")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("server returned status {}", resp.status()));
    }
    let json = resp.text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&json).map_err(|e| e.to_string())
}

async fn delete_entry(id: String) -> Result<(), String> {
    let resp = Request::delete(&format!("/api/admin/files/{}", id))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() != 204 {
        return Err(format!("server returned status {}", resp.status()));
    }
    Ok(())
}

fn resolve_mime(file: &File) -> String {
    let name = file.name().to_ascii_lowercase();
    let ext = name.rsplit('.').next().unwrap_or("");
    match ext {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "zip" => "application/zip",
        "pdf" => "application/pdf",
        "gz" | "tgz" => "application/gzip",
        "tar" => "application/x-tar",
        "xz" => "application/x-xz",
        "zst" => "application/zstd",
        "7z" => "application/x-7z-compressed",
        "txt" => "text/plain",
        "csv" => "text/csv",
        "md" | "markdown" => "text/markdown",
        "json" => "application/json",
        _ => return file.type_(),
    }
    .to_string()
}

/// Header values must be Latin-1 and can't contain quotes. The backend re-sanitizes anyway.
fn header_safe_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii() && c != '"' && !c.is_control() {
                c
            } else {
                '_'
            }
        })
        .collect()
}

async fn upload_file(file: File) -> Result<(), String> {
    let mime = resolve_mime(&file);
    let name = header_safe_name(&file.name());

    let resp = Request::post("/api/admin/files")
        .header("Content-Type", &mime)
        .header(
            "Content-Disposition",
            &format!("attachment; filename=\"{}\"", name),
        )
        .body(file) // raw File as body, browser streams it
        .send()
        .await
        .map_err(|e| e.to_string())?;

    match resp.status() {
        201 => Ok(()),
        400 => Err("bad request".into()),
        413 => Err("file too large".into()),
        415 => Err(format!("unsupported type ({mime})")),
        422 => Err("content doesn't match declared type".into()),
        s => Err(format!("server returned status {s}")),
    }
}

async fn copy_to_clipboard(text: String) -> Result<(), String> {
    let window = web_sys::window().ok_or("no window")?;
    let promise = window.navigator().clipboard().write_text(&text);
    JsFuture::from(promise)
        .await
        .map(|_| ())
        .map_err(|e| format!("clipboard: {:?}", e))
}

fn human_size(bytes: i64) -> String {
    const UNITS: [&str; 4] = ["B", "KiB", "MiB", "GiB"];
    let mut v = bytes as f64;
    let mut i = 0;
    while v >= 1024.0 && i < UNITS.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    if i == 0 {
        format!("{} B", bytes)
    } else {
        format!("{:.1} {}", v, UNITS[i])
    }
}

#[component]
pub fn FileEntries() -> impl IntoView {
    let refresh_trigger = RwSignal::new(0u32);
    let message = RwSignal::new(None::<String>);
    let uploading = RwSignal::new(false);
    let copied_id = RwSignal::new(None::<String>);

    let file_entries = LocalResource::new(move || {
        refresh_trigger.get();
        fetch_entries()
    });

    let on_file_change = move |ev: leptos::ev::Event| {
        let input = event_target::<HtmlInputElement>(&ev);
        let Some(list) = input.files() else { return };
        let files: Vec<File> = (0..list.length()).filter_map(|i| list.item(i)).collect();
        input.set_value("");
        if files.is_empty() {
            return;
        }

        uploading.set(true);
        message.set(None);
        leptos::task::spawn_local(async move {
            let mut errors = Vec::new();
            for f in files {
                let name = f.name();
                if let Err(e) = upload_file(f).await {
                    errors.push(format!("{name}: {e}"));
                }
            }
            uploading.set(false);
            refresh_trigger.update(|n| *n += 1);
            if !errors.is_empty() {
                message.set(Some(errors.join("; ")));
            }
        });
    };

    view! {
        <section class="page-section">
            <div class="files-container">
                <h2>"Files"</h2>

                <div class="file-upload">
                    <input
                        type="file"
                        multiple
                        disabled=move || uploading.get()
                        on:change=on_file_change
                    />
                    <Show when=move || uploading.get()>
                        <span class="loading-message">"Uploading..."</span>
                    </Show>
                </div>

                <Show when=move || message.get().is_some()>
                    <div class="error-message">
                        <p>{move || message.get().unwrap_or_default()}</p>
                    </div>
                </Show>

                {move || match file_entries.get() {
                    Some(Ok(files)) => view! {
                        <ul class="files-list">
                            {files.into_iter().map(|file| {
                                let id = file.id.to_string();
                                let path = format!(
                                    "/media/{}/{}/{}",
                                    file.kind, file.sha256, file.original_name
                                );
                                let md = if file.kind == "image" {
                                    format!("![{}]({})", file.original_name, path)
                                } else {
                                    format!("[{}]({})", file.original_name, path)
                                };

                                let copy_id = id.clone();
                                let on_copy = move |_| {
                                    let md = md.clone();
                                    let copy_id = copy_id.clone();
                                    leptos::task::spawn_local(async move {
                                        match copy_to_clipboard(md).await {
                                            Ok(()) => {
                                                copied_id.set(Some(copy_id));
                                                set_timeout(
                                                    move || copied_id.set(None),
                                                    Duration::from_millis(1500),
                                                );
                                            }
                                            Err(e) => message.set(Some(e)),
                                        }
                                    });
                                };

                                let del_id = id.clone();
                                let name = file.original_name.clone();
                                let on_delete = move |_| {
                                    let confirmed = web_sys::window()
                                        .and_then(|w| {
                                            w.confirm_with_message(&format!("Delete {name}?")).ok()
                                        })
                                        .unwrap_or(false);
                                    if !confirmed {
                                        return;
                                    }
                                    let del_id = del_id.clone();
                                    leptos::task::spawn_local(async move {
                                        match delete_entry(del_id).await {
                                            Ok(()) => {
                                                message.set(None);
                                                refresh_trigger.update(|n| *n += 1);
                                            }
                                            Err(e) => message.set(Some(e)),
                                        }
                                    });
                                };

                                let is_copied = {
                                    let id = id.clone();
                                    move || copied_id.get().as_deref() == Some(id.as_str())
                                };

                                view! {
                                    <li class="file-entry">
                                        <h3 class="file-title">{file.original_name.clone()}</h3>
                                        <p class="file-path"><a href={path.clone()}>{path.clone()}</a></p>
                                        <div class="file-meta">
                                            <div class="file-meta_byte-mime">
                                                <p class="file-meta-bytes">{human_size(file.bytes)}</p>
                                                <p class="file-meta-mime">" / " {file.mime.clone()}</p>
                                            </div>
                                            <p class="file-meta-created_at">
                                                {file.created_at.to_string()}
                                            </p>
                                        </div>
                                        <div class="file-actions">
                                            <button on:click=on_copy>
                                                {move || if is_copied() { "Copied!" } else { "Copy MD link" }}
                                            </button>
                                            <button class="danger" on:click=on_delete>"Delete"</button>
                                        </div>
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    }.into_any(),
                    Some(Err(e)) => view! {
                        <div class="error-message">
                            <p>{format!("Failed to fetch files: {}", e)}</p>
                        </div>
                    }.into_any(),
                    None => view! {
                        <div class="loading-message">
                            <p>"Loading files..."</p>
                        </div>
                    }.into_any(),
                }}
            </div>
        </section>
    }
}
