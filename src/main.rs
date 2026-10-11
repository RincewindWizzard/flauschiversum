use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

mod archive;
mod config;
mod content;
mod diag;
mod feed;
mod markdown;
mod media;
mod output;
mod render;
mod search;
mod seo;
mod serve;
mod site;
mod topics;
mod urls;

/// Generator für das Flauschiversum.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Baut die Seite in das Ausgabeverzeichnis.
    Build(PathArgs),
    /// Prüft alle Inhalte, ohne etwas zu schreiben.
    Check(PathArgs),
    /// Baut die Seite, liefert sie lokal aus und baut bei Änderungen neu.
    Serve {
        #[command(flatten)]
        paths: PathArgs,
        /// Port des Vorschau-Servers.
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },
}

#[derive(Args, Clone)]
struct PathArgs {
    /// Projektverzeichnis (enthält `content/`).
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Ausgabeverzeichnis (relativ zum Projektverzeichnis).
    #[arg(long, default_value = "dist")]
    out: PathBuf,
    /// Verzeichnis für den Bild-Cache (relativ zum Projektverzeichnis).
    #[arg(long, default_value = ".cache")]
    cache: PathBuf,
    /// Basis-URL für Absolut-URLs (kanonische Verweise, Feed, Sitemap), nur Schema und Host.
    #[arg(long, env = "FLAUSCHI_BASE_URL", value_parser = urls::Base::parse, default_value = "https://flauschiversum.de")]
    base_url: urls::Base,
    /// Staging-Modus: noindex, robots.txt sperrt alles, keine Sitemap, Leiste mit dem Branchnamen.
    #[arg(long, value_name = "BRANCH", value_parser = branch_name)]
    staging: Option<String>,
}

fn branch_name(value: &str) -> Result<String, String> {
    let value = value.trim();
    if value.is_empty() { Err("ein Branchname wird erwartet".to_string()) } else { Ok(value.to_string()) }
}

impl PathArgs {
    fn resolve(&self, p: &PathBuf) -> PathBuf {
        if p.is_absolute() { p.clone() } else { self.root.join(p) }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Fehler: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn print_diagnostics(diag: &diag::Diagnostics) {
    for d in diag.all() {
        eprintln!("{d}");
    }
}

/// Führt einen Build aus und gibt eine Zusammenfassung aus. Liefert `false` bei Fehlern.
fn build_once(paths: &PathArgs, livereload: bool) -> Result<bool> {
    let started = std::time::Instant::now();
    let opts = site::BuildOptions {
        root: paths.root.clone(),
        cache: paths.resolve(&paths.cache),
        now: site::BuildOptions::now()?,
        livereload,
        base: paths.base_url.clone(),
        staging: paths.staging.clone(),
    };
    let diag = diag::Diagnostics::default();
    let stats = media::Stats::default();
    let generated = match site::generate(&opts, &diag, &stats) {
        Ok(g) => g,
        Err(e) => {
            print_diagnostics(&diag);
            eprintln!("Fehler: {e:#}");
            return Ok(false);
        }
    };
    let posts = generated.site.posts.len();
    let scheduled = generated.site.scheduled;
    let ws = output::write(&paths.resolve(&paths.out), generated.outputs.into_map())?;
    print_diagnostics(&diag);
    use std::sync::atomic::Ordering::Relaxed;
    println!(
        "{posts} Posts ({scheduled} geplant) · Bilder: {} neu berechnet, {} aus dem Cache",
        stats.processed.load(Relaxed),
        stats.cached.load(Relaxed)
    );
    println!(
        "Dateien: {} neu, {} geändert, {} unverändert, {} gelöscht · {} Warnungen · {:.1} s",
        ws.new,
        ws.changed,
        ws.unchanged,
        ws.deleted,
        diag.count(diag::Level::Warning),
        started.elapsed().as_secs_f64()
    );
    Ok(true)
}

fn run(cli: Cli) -> Result<ExitCode> {
    match cli.command {
        Command::Check(paths) => {
            let opts = site::BuildOptions {
                root: paths.root.clone(),
                cache: paths.resolve(&paths.cache),
                now: site::BuildOptions::now()?,
                livereload: false,
                base: paths.base_url.clone(),
                staging: paths.staging.clone(),
            };
            let diag = diag::Diagnostics::default();
            let result = site::generate(&opts, &diag, &media::Stats::default());
            print_diagnostics(&diag);
            match result {
                Ok(g) => {
                    println!(
                        "{} Posts ({} geplant), {} Seiten, {} Ausgabedateien · {} Warnungen",
                        g.site.posts.len(),
                        g.site.scheduled,
                        g.site.pages.len(),
                        g.outputs.len(),
                        diag.count(diag::Level::Warning)
                    );
                    Ok(ExitCode::SUCCESS)
                }
                Err(e) => {
                    eprintln!("Fehler: {e:#}");
                    Ok(ExitCode::FAILURE)
                }
            }
        }
        Command::Build(paths) => Ok(if build_once(&paths, false)? { ExitCode::SUCCESS } else { ExitCode::FAILURE }),
        Command::Serve { paths, port } => {
            // Eigenes Ausgabeverzeichnis, damit `dist/` nicht das Live-Reload-Skript enthält
            let mut serve_paths = paths.clone();
            serve_paths.out = paths.resolve(&paths.cache).join("serve");
            let out = serve_paths.out.clone();
            serve::run(&paths.root, &out, port, || build_once(&serve_paths, true))?;
            Ok(ExitCode::SUCCESS)
        }
    }
}
