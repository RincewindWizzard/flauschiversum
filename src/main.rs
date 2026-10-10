#![allow(dead_code)] // TODO: entfernen, sobald alle Module angebunden sind
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

mod config;
mod content;
mod diag;
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

fn run(cli: Cli) -> Result<ExitCode> {
    match cli.command {
        Command::Check(paths) => {
            let now = jiff::Zoned::now().with_time_zone(jiff::tz::TimeZone::get(config::TIME_ZONE)?);
            let diag = diag::Diagnostics::default();
            let site = content::load(&paths.root, &now, &diag);
            for d in diag.all() {
                eprintln!("{d}");
            }
            println!(
                "{} Posts ({} geplant), {} Seiten, {} statische Dateien",
                site.posts.len(),
                site.scheduled,
                site.pages.len(),
                site.static_files.len()
            );
            Ok(if diag.has_errors() { ExitCode::FAILURE } else { ExitCode::SUCCESS })
        }
        Command::Build(paths) => {
            let _ = (paths.resolve(&paths.out), paths.resolve(&paths.cache));
            anyhow::bail!("noch nicht umgesetzt")
        }
        Command::Serve { paths, port } => {
            let _ = (paths, port);
            anyhow::bail!("noch nicht umgesetzt")
        }
    }
}
