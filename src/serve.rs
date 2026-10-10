//! Vorschau-Server: baut die Seite, liefert sie lokal aus und baut bei Änderungen neu.

use crate::urls;
use anyhow::{Context, Result};
use notify::{RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::channel;
use std::time::Duration;

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).as_deref() {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("xml") => "application/xml; charset=utf-8",
        Some("txt") => "text/plain; charset=utf-8",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/vnd.microsoft.icon",
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",
        Some("otf") => "font/otf",
        Some("ttf") => "font/ttf",
        _ => "application/octet-stream",
    }
}

/// Ordnet einen Anfragepfad einer Datei im Ausgabeverzeichnis zu (inkl. `index.html`).
fn resolve(out: &Path, url: &str) -> Option<PathBuf> {
    let path = urls::decode_path(url.split(['?', '#']).next().unwrap_or("/"));
    if path.split('/').any(|seg| seg == "..") {
        return None;
    }
    let mut file = out.join(path.trim_start_matches('/'));
    if path.ends_with('/') {
        file.push("index.html");
    }
    file.is_file().then_some(file)
}

fn handle(request: tiny_http::Request, out: &Path, generation: &AtomicU64) {
    let url = request.url().to_string();
    // Live-Reload: die Seite fragt regelmäßig die Nummer des letzten Builds ab
    if url.split('?').next() == Some("/__livereload") {
        let nc = tiny_http::Header::from_bytes(&b"Cache-Control"[..], &b"no-store"[..]).unwrap();
        let _ = request.respond(tiny_http::Response::from_string(generation.load(Ordering::SeqCst).to_string()).with_header(nc));
        return;
    }
    // Verzeichnis ohne Schrägstrich → Weiterleitung wie auf dem Server
    let bare = url.split(['?', '#']).next().unwrap_or("/");
    if !bare.ends_with('/') && resolve(out, &format!("{bare}/")).is_some() && resolve(out, bare).is_none() {
        let loc = tiny_http::Header::from_bytes(&b"Location"[..], format!("{bare}/").as_bytes()).unwrap();
        let _ = request.respond(tiny_http::Response::empty(301).with_header(loc));
        return;
    }
    match resolve(out, &url).and_then(|f| std::fs::File::open(&f).ok().map(|h| (f, h))) {
        Some((path, file)) => {
            let ct = tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type(&path).as_bytes()).unwrap();
            let nc = tiny_http::Header::from_bytes(&b"Cache-Control"[..], &b"no-cache"[..]).unwrap();
            let _ = request.respond(tiny_http::Response::from_file(file).with_header(ct).with_header(nc));
        }
        None => {
            let _ = request.respond(tiny_http::Response::from_string("404 – nicht gefunden").with_status_code(404));
        }
    }
}

/// Startet den Vorschau-Server. `rebuild` baut die Seite (mit Live-Reload-Skript) nach `out`.
pub fn run(root: &Path, out: &Path, port: u16, rebuild: impl Fn() -> Result<bool>) -> Result<()> {
    rebuild()?;
    let server = tiny_http::Server::http(("127.0.0.1", port)).map_err(|e| anyhow::anyhow!("Port {port}: {e}"))?;
    println!("Vorschau: http://localhost:{port}/  (Strg+C beendet)");

    // Startwert aus der Uhrzeit, damit ein Neustart des Servers ebenfalls neu lädt
    let start = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(1);
    let generation = Arc::new(AtomicU64::new(start));
    let out_dir = out.to_path_buf();
    let server_generation = generation.clone();
    std::thread::spawn(move || {
        for request in server.incoming_requests() {
            let (out, generation) = (out_dir.clone(), server_generation.clone());
            std::thread::spawn(move || handle(request, &out, &generation));
        }
    });

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(event) = res
            && !matches!(event.kind, notify::EventKind::Access(_)) {
                let _ = tx.send(());
            }
    })?;
    for dir in ["content", "assets"] {
        watcher
            .watch(&root.join(dir), RecursiveMode::Recursive)
            .with_context(|| format!("{dir}/ kann nicht beobachtet werden"))?;
    }
    while rx.recv().is_ok() {
        // Entprellen: weitere Ereignisse kurz sammeln
        while rx.recv_timeout(Duration::from_millis(300)).is_ok() {}
        println!("Änderung erkannt – baue neu …");
        match rebuild() {
            Ok(true) => {
                generation.fetch_add(1, Ordering::SeqCst);
            }
            Ok(false) => eprintln!("Build fehlgeschlagen – Vorschau zeigt den letzten Stand"),
            Err(e) => eprintln!("Fehler: {e:#}"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_directories_and_blocks_traversal() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("a b")).unwrap();
        std::fs::write(dir.path().join("a b/index.html"), "x").unwrap();
        assert!(resolve(dir.path(), "/a%20b/").is_some());
        assert!(resolve(dir.path(), "/a%20b/?v=1").is_some());
        assert!(resolve(dir.path(), "/../etc/passwd").is_none());
        assert!(resolve(dir.path(), "/fehlt/").is_none());
    }
}
