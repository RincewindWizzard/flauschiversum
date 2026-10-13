//! `deploy`: legt die gebaute Seite per SSH auf den Server.
//!
//! Ablauf: `dist/` prüfen (nur ein Produktionsbuild), Manifest des Servers lesen, nur Geändertes übertragen (tar über SSH,
//! sonst SFTP; kein rsync nötig) in ein Zwischenverzeichnis und dort einzeln an den Platz verschieben: erst Medien und
//! Suchindex, dann die Seiten, dann Veraltetes löschen, zuletzt das neue Manifest. Gelöscht wird nur, was ein früheres
//! Deployment selbst abgelegt hat (Manifest); fremde Dateien im Ziel bleiben.

use crate::output::{self, Entry, MANIFEST};
use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{IsTerminal, Write};
use std::time::{Duration, Instant};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Übertragungsweg.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Method {
    Auto,
    Tar,
    Sftp,
}

pub struct Options {
    pub dist: PathBuf,
    /// `host:/pfad/` (SSH-Host aus der SSH-Konfiguration) oder ein lokaler Pfad.
    pub target: String,
    /// Basis-URL, zu der `dist/` gehören muss.
    pub base_url: String,
    pub dry_run: bool,
    /// Alles neu übertragen, auch wenn das Manifest des Servers übereinstimmt.
    pub full: bool,
    pub method: Method,
    /// `ssh`- und `sftp`-Programm (austauschbar für Tests).
    pub ssh: String,
    pub sftp: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Plan {
    pub assets: Vec<String>,
    pub pages: Vec<String>,
    pub delete: Vec<String>,
    pub bytes: u64,
}

impl Plan {
    pub fn upload_count(&self) -> usize {
        self.assets.len() + self.pages.len()
    }
}

/// Ziel: lokal oder per SSH.
struct Target {
    ssh: String,
    sftp: String,
    host: Option<String>,
    root: String,
}

fn sq(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

impl Target {
    fn parse(target: &str, ssh: &str, sftp: &str) -> Result<Target> {
        let (host, root) = if target.starts_with('/') || target.starts_with("./") || target.starts_with("../") {
            (None, target.to_string())
        } else if let Some((h, p)) = target.split_once(':') {
            (Some(h.to_string()), p.to_string())
        } else {
            (None, target.to_string())
        };
        let root = root.trim_end_matches('/').to_string();
        if root.is_empty() {
            bail!("das Ziel darf nicht das Wurzelverzeichnis sein");
        }
        if root.contains(['\'', '"', '\\', '$', '`', ' ', '\n']) {
            bail!("unzulässige Zeichen im Zielpfad: {root}");
        }
        Ok(Target { ssh: ssh.to_string(), sftp: sftp.to_string(), host, root })
    }

    fn name(&self) -> String {
        match &self.host {
            Some(h) => format!("{h}:{}/", self.root),
            None => format!("{}/", self.root),
        }
    }

    /// Befehl auf dem Ziel (lokal: `sh -c`, sonst `ssh <Host> <Befehl>` ohne Passwortabfrage).
    fn command(&self, cmd: &str) -> Command {
        match &self.host {
            None => {
                let mut c = Command::new("sh");
                c.args(["-c", cmd]);
                c
            }
            Some(h) => {
                let mut c = Command::new(&self.ssh);
                c.args(["-o", "BatchMode=yes", h, cmd]);
                c
            }
        }
    }

    fn run(&self, cmd: &str) -> Result<std::process::Output> {
        self.command(cmd).stdin(Stdio::null()).output().with_context(|| format!("`{}` konnte nicht gestartet werden", self.ssh))
    }

    fn ok(&self, cmd: &str) -> bool {
        self.run(cmd).map(|o| o.status.success()).unwrap_or(false)
    }

    /// Führt `cmd` aus und gibt die Standardeingabe über `feed` vor.
    fn run_with_input(&self, mut command: Command, feed: impl FnOnce(&mut dyn Write) -> Result<()>) -> Result<()> {
        let mut child = command.stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::piped()).spawn().context("Befehl konnte nicht gestartet werden")?;
        let mut stdin = child.stdin.take().expect("stdin");
        let fed = feed(&mut stdin);
        drop(stdin);
        let out = child.wait_with_output()?;
        if !out.status.success() {
            bail!("Befehl auf dem Ziel fehlgeschlagen: {}", String::from_utf8_lossy(&out.stderr).trim());
        }
        fed
    }

    fn sftp_command(&self) -> Option<Command> {
        let host = self.host.as_ref()?;
        let mut c = Command::new(&self.sftp);
        c.args(["-q", "-b", "-", host]);
        Some(c)
    }
}

/// Prüft, dass `dist/` ein Produktionsbuild zu `base_url` ist, und liest sein Manifest.
pub fn verify_dist(dist: &Path, base_url: &str) -> Result<BTreeMap<String, Entry>> {
    let index = dist.join("index.html");
    let html = std::fs::read_to_string(&index).with_context(|| format!("{} fehlt (Seite nicht gebaut?)", index.display()))?;
    if html.contains("class=\"staging-bar\"") {
        bail!("dist/ ist ein Staging-Build (Staging-Leiste auf der Startseite)");
    }
    if html.contains("<meta name=\"robots\" content=\"noindex\">") {
        bail!("dist/ ist nicht für Produktion gebaut (noindex auf der Startseite)");
    }
    let canonical = html.split("<link rel=\"canonical\" href=\"").nth(1).and_then(|r| r.split('"').next()).unwrap_or("");
    let base = base_url.trim_end_matches('/');
    if canonical != base && !canonical.starts_with(&format!("{base}/")) {
        bail!("dist/ gehört nicht zu {base_url} (kanonische URL der Startseite: {})", if canonical.is_empty() { "keine" } else { canonical });
    }
    output::read_manifest(dist).with_context(|| "dist/ wurde nicht mit diesem Generator gebaut")
}

/// Pfad aus dem Manifest: relativ, ohne `..` und ohne Zeichen, die in Befehlen Probleme machen.
fn safe_rel(key: &str) -> Result<String> {
    let rel = key.trim_start_matches('/');
    if rel.is_empty() || rel.split('/').any(|p| p == ".." || p.is_empty()) || rel.contains(['\n', '\0', '\'', '"']) {
        bail!("unzulässiger Pfad im Manifest: {key:?}");
    }
    Ok(rel.to_string())
}

fn is_page(path: &str) -> bool {
    path.ends_with(".html") || path.ends_with(".xml")
}

/// Vergleicht die Manifeste: übertragen wird, was dort fehlt oder einen anderen Hash hat; gelöscht, was früher geliefert
/// wurde und jetzt fehlt.
///
/// `present` sind die Einträge des Server-Manifests, deren Datei auch wirklich auf dem Server liegt; `recorded` ist das
/// Server-Manifest selbst (bestimmt, was gelöscht werden darf).
pub fn plan(dist: &Path, local: &BTreeMap<String, Entry>, present: &BTreeMap<String, Entry>, recorded: &BTreeMap<String, Entry>) -> Result<Plan> {
    let mut plan = Plan::default();
    for (key, entry) in local {
        let rel = safe_rel(key)?;
        if !dist.join(&rel).is_file() {
            bail!("laut Manifest vorhandene Datei fehlt in dist/: {rel}");
        }
        if present.get(key).map(|e| &e.hash) != Some(&entry.hash) {
            plan.bytes += entry.len;
            if is_page(&rel) { plan.pages.push(rel) } else { plan.assets.push(rel) }
        }
    }
    for key in recorded.keys().filter(|k| !local.contains_key(*k)) {
        plan.delete.push(safe_rel(key)?);
    }
    Ok(plan)
}

fn parents(path: &str) -> impl Iterator<Item = String> + '_ {
    let parts: Vec<&str> = path.split('/').collect();
    (1..parts.len()).map(move |i| parts[..i].join("/"))
}

/// Gerader Ablauf (ohne Schleifen und Bedingungen) für `sh -s` auf dem Ziel: Verzeichnisse anlegen, erst Medien, dann
/// Seiten verschieben, Veraltetes löschen, zuletzt das Manifest.
pub fn install_script(root: &str, tmp: &str, plan: &Plan) -> String {
    let mut s = String::from("set -e\n");
    s += &format!("cd {}\n", sq(root));
    let dirs: BTreeSet<String> = plan.assets.iter().chain(&plan.pages).flat_map(|p| parents(p)).collect();
    for d in &dirs {
        s += &format!("mkdir -p {}\nchmod 755 {} 2>/dev/null || true\n", sq(d), sq(d));
    }
    for f in plan.assets.iter().chain(&plan.pages) {
        s += &format!("mv -f {} {}\n", sq(&format!("{tmp}/{f}")), sq(f));
    }
    for f in &plan.delete {
        s += &format!("rm -f -- {}\n", sq(f));
    }
    // leere Verzeichnisse der gelöschten Dateien, tiefste zuerst
    let mut gone: Vec<String> = plan.delete.iter().flat_map(|p| parents(p)).collect::<BTreeSet<_>>().into_iter().collect();
    gone.sort_by_key(|d| std::cmp::Reverse(d.matches('/').count()));
    for d in gone {
        s += &format!("rmdir {} 2>/dev/null || true\n", sq(&d));
    }
    s += &format!("mv -f {} {}\n", sq(&format!("{tmp}/{MANIFEST}")), sq(MANIFEST));
    s += &format!("rm -rf {}\n", sq(tmp));
    s
}

/// Fortschrittsanzeige der Übertragung: im Terminal eine sich aktualisierende Zeile, sonst (z. B. in CI) alle paar Sekunden
/// eine eigene Zeile.
struct Progress {
    total_files: usize,
    total_bytes: u64,
    done_files: usize,
    done_bytes: u64,
    start: Instant,
    last: Instant,
    tty: bool,
}

/// `4521/10146 Dateien · 212/527 MB (40 %) · 18,3 MB/s · noch 17 s`
pub fn progress_line(done_files: usize, total_files: usize, done_bytes: u64, total_bytes: u64, elapsed: Duration) -> String {
    let mb = |b: u64| b as f64 / 1_048_576.0;
    let percent = if total_bytes == 0 { 100 } else { (done_bytes as f64 * 100.0 / total_bytes as f64).floor() as u64 };
    let secs = elapsed.as_secs_f64();
    let rate = if secs > 0.0 { done_bytes as f64 / secs } else { 0.0 };
    let eta = if rate > 0.0 && total_bytes > done_bytes { format!(" · noch {}", duration_text(((total_bytes - done_bytes) as f64 / rate).ceil() as u64)) } else { String::new() };
    let speed = if rate > 0.0 { format!(" · {:.1} MB/s", rate / 1_048_576.0).replace('.', ",") } else { String::new() };
    format!("{done_files}/{total_files} Dateien · {:.0}/{:.0} MB ({percent} %){speed}{eta}", mb(done_bytes), mb(total_bytes))
}

fn duration_text(secs: u64) -> String {
    if secs >= 3600 {
        format!("{} h {} min", secs / 3600, secs % 3600 / 60)
    } else if secs >= 60 {
        format!("{} min {} s", secs / 60, secs % 60)
    } else {
        format!("{secs} s")
    }
}

impl Progress {
    fn new(total_files: usize, total_bytes: u64) -> Progress {
        Progress { total_files, total_bytes, done_files: 0, done_bytes: 0, start: Instant::now(), last: Instant::now(), tty: std::io::stderr().is_terminal() }
    }

    fn advance(&mut self, bytes: u64) {
        self.done_files += 1;
        self.done_bytes += bytes;
        let every = if self.tty { Duration::from_millis(250) } else { Duration::from_secs(5) };
        if self.last.elapsed() >= every {
            self.last = Instant::now();
            self.print(false);
        }
    }

    fn print(&self, last: bool) {
        let line = progress_line(self.done_files, self.total_files, self.done_bytes, self.total_bytes, self.start.elapsed());
        if self.tty {
            eprint!("\r    Übertrage: {line}\x1b[K{}", if last { "\n" } else { "" });
        } else {
            eprintln!("    Übertrage: {line}");
        }
    }

    fn finish(&self) {
        self.print(true);
    }
}

fn upload_tar(target: &Target, dist: &Path, tmp: &str, plan: &Plan) -> Result<()> {
    let cmd = format!("tar -xzf - -C {}", sq(&format!("{}/{tmp}", target.root)));
    target.run_with_input(target.command(&cmd), |w| {
        let gz = flate2::write::GzEncoder::new(w, flate2::Compression::fast());
        let mut tar = tar::Builder::new(gz);
        let mut progress = Progress::new(plan.upload_count(), plan.bytes);
        for rel in plan.assets.iter().chain(&plan.pages).map(String::as_str).chain(std::iter::once(MANIFEST)) {
            let path = dist.join(rel);
            let mut file = std::fs::File::open(&path).with_context(|| path.display().to_string())?;
            let len = file.metadata()?.len();
            let mut header = tar::Header::new_gnu();
            header.set_size(len);
            header.set_mode(0o644);
            header.set_mtime(0);
            header.set_cksum();
            tar.append_data(&mut header, rel, &mut file).with_context(|| format!("{rel} packen"))?;
            if rel != MANIFEST {
                progress.advance(len);
            }
        }
        tar.into_inner()?.finish()?;
        progress.finish();
        Ok(())
    })
}

fn upload_sftp(target: &Target, dist: &Path, tmp: &str, plan: &Plan) -> Result<()> {
    let command = target.sftp_command().context("SFTP braucht ein SSH-Ziel")?;
    let base = format!("{}/{tmp}", target.root);
    let files: Vec<&str> = plan.assets.iter().chain(&plan.pages).map(String::as_str).chain(std::iter::once(MANIFEST)).collect();
    let dirs: BTreeSet<String> = files.iter().flat_map(|p| parents(p)).collect();
    target.run_with_input(command, |w| {
        for d in &dirs {
            writeln!(w, "-mkdir \"{base}/{d}\"")?;
        }
        let mut progress = Progress::new(plan.upload_count(), plan.bytes);
        for f in &files {
            writeln!(w, "put \"{}\" \"{base}/{f}\"", dist.join(f).display())?;
            writeln!(w, "chmod 644 \"{base}/{f}\"")?;
            if *f != MANIFEST {
                // sftp liest die Befehle der Reihe nach; das Schreiben blockiert, solange es noch zu tun hat
                w.flush()?;
                progress.advance(std::fs::metadata(dist.join(f)).map(|m| m.len()).unwrap_or(0));
            }
        }
        progress.finish();
        Ok(())
    })
}

fn detect(target: &Target, wanted: Method) -> Result<Method> {
    let tar = || target.ok("command -v tar >/dev/null 2>&1 && command -v gzip >/dev/null 2>&1");
    let sftp = || {
        target.sftp_command().is_some_and(|mut c| {
            c.stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null());
            c.spawn()
                .and_then(|mut ch| {
                    ch.stdin.take().expect("stdin").write_all(b"pwd\n")?;
                    ch.wait()
                })
                .map(|s| s.success())
                .unwrap_or(false)
        })
    };
    match wanted {
        Method::Tar if tar() => Ok(Method::Tar),
        Method::Sftp if sftp() => Ok(Method::Sftp),
        Method::Tar | Method::Sftp => bail!("der gewählte Übertragungsweg {wanted:?} ist auf dem Ziel nicht nutzbar"),
        Method::Auto if tar() => Ok(Method::Tar),
        Method::Auto if sftp() => Ok(Method::Sftp),
        Method::Auto => bail!("kein Übertragungsweg nutzbar: weder tar (mit gzip) noch SFTP auf dem Ziel gefunden"),
    }
}

/// Entfernt das Zwischenverzeichnis, auch bei Fehlern.
struct Cleanup<'a> {
    target: &'a Target,
    tmp: String,
    active: bool,
}

impl Drop for Cleanup<'_> {
    fn drop(&mut self) {
        if self.active {
            let _ = self.target.run(&format!("rm -rf {}", sq(&format!("{}/{}", self.target.root, self.tmp))));
        }
    }
}

/// Sperre gegen gleichzeitige Deployments im selben Arbeitsverzeichnis.
pub fn lock(path: &Path) -> Result<std::fs::File> {
    let file = std::fs::OpenOptions::new().create(true).truncate(false).write(true).open(path).with_context(|| path.display().to_string())?;
    match file.try_lock() {
        Ok(()) => Ok(file),
        Err(std::fs::TryLockError::WouldBlock) => bail!("es läuft bereits ein Deployment (Sperre {})", path.display()),
        Err(std::fs::TryLockError::Error(e)) => Err(e).context("Sperre"),
    }
}

/// Welche Dateien des Server-Manifests fehlen auf dem Server in Wirklichkeit (von Hand gelöscht, verloren gegangen)?
/// Das Manifest allein beweist nichts: Es bleibt als Punktdatei auch dann liegen, wenn jemand `rm -rf *` ausführt.
fn missing_on_server(target: &Target, recorded: &BTreeMap<String, Entry>) -> Result<BTreeSet<String>> {
    if recorded.is_empty() {
        return Ok(BTreeSet::new());
    }
    let mut script = format!("cd {}\n", sq(&target.root));
    for key in recorded.keys() {
        let rel = safe_rel(key)?;
        script += &format!("[ -f {0} ] || echo {0}\n", sq(&rel));
    }
    let mut child = target.command("sh -s").stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().context("Prüfung der Dateien auf dem Ziel")?;
    let mut stdin = child.stdin.take().expect("stdin");
    // Die Eingabe wird nebenher geschrieben: Fehlen sehr viele Dateien, füllt die Antwort die Pipe, während das Skript noch läuft
    let writer = std::thread::spawn(move || stdin.write_all(script.as_bytes()));
    let output = child.wait_with_output()?;
    let _ = writer.join();
    if !output.status.success() {
        bail!("Prüfung der Dateien auf dem Ziel fehlgeschlagen: {}", String::from_utf8_lossy(&output.stderr).trim());
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| l.trim_matches('\'').to_string())
        .filter(|l| !l.is_empty())
        .map(|l| format!("/{l}"))
        .collect())
}

/// Führt das Deployment aus. Gibt Plan und gewählten Weg zurück.
pub fn run(opts: &Options) -> Result<(Plan, Method)> {
    let local = verify_dist(&opts.dist, &opts.base_url)?;
    let target = Target::parse(&opts.target, &opts.ssh, &opts.sftp)?;
    if target.host.is_some() && !target.ok("true") {
        bail!(
            "{} ist per SSH nicht ohne Passwortabfrage erreichbar (SSH-Eintrag mit Schlüssel in ~/.ssh/config prüfen)",
            target.host.as_deref().unwrap_or("")
        );
    }
    let method = detect(&target, opts.method)?;

    let recorded: BTreeMap<String, Entry> = {
        let out = target.run(&format!("cat {} 2>/dev/null || true", sq(&format!("{}/{MANIFEST}", target.root))))?;
        serde_json::from_slice(&out.stdout).unwrap_or_default()
    };
    let missing = if opts.full { BTreeSet::new() } else { missing_on_server(&target, &recorded)? };
    let present: BTreeMap<String, Entry> = if opts.full {
        BTreeMap::new()
    } else {
        recorded.iter().filter(|(k, _)| !missing.contains(*k)).map(|(k, v)| (k.clone(), v.clone())).collect()
    };
    let plan = plan(&opts.dist, &local, &present, &recorded)?;
    let mb = plan.bytes.div_ceil(1024 * 1024);
    println!("Ziel: {}  ·  Übertragungsweg: {}", target.name(), if method == Method::Tar { "tar über SSH" } else { "SFTP" });
    println!(
        "Zu übertragen: {} Dateien (rund {mb} MB; {} Medien/Skripte/Index, {} Seiten), zu löschen: {}",
        plan.upload_count(),
        plan.assets.len(),
        plan.pages.len(),
        plan.delete.len()
    );
    if opts.full {
        println!("Vollständiges Deployment (--full): Alles wird neu übertragen.");
    } else if !missing.is_empty() {
        println!("Auf dem Server fehlen {} Dateien, die das Manifest dort verzeichnet; sie werden erneut übertragen.", missing.len());
    }
    if recorded.is_empty() {
        let out = target.run(&format!("ls -A {} 2>/dev/null | wc -l", sq(&target.root)))?;
        let foreign: usize = String::from_utf8_lossy(&out.stdout).trim().parse().unwrap_or(0);
        if foreign > 0 {
            println!("Hinweis: Im Ziel liegen {foreign} Einträge, die nicht von dieser Seite stammen; sie bleiben unberührt (erstes Deployment überlagert nur).");
        }
    }
    if opts.dry_run {
        println!("Trockenlauf: Es wurde nichts übertragen oder gelöscht.");
        for (title, items) in [("Zu löschen", &plan.delete), ("Zu übertragen", &plan.assets)] {
            if !items.is_empty() {
                println!("{title} (erste 20):");
                items.iter().take(20).for_each(|i| println!("  {i}"));
            }
        }
        return Ok((plan, method));
    }
    if plan.upload_count() == 0 && plan.delete.is_empty() {
        println!("0 Dateien übertragen: Der Server ist auf dem Stand von dist/.");
        return Ok((plan, method));
    }

    let tmp = format!(".deploy-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs());
    let mut cleanup = Cleanup { target: &target, tmp: tmp.clone(), active: true };
    let made = target.run(&format!("mkdir -p {}", sq(&format!("{}/{tmp}", target.root))))?;
    if !made.status.success() {
        bail!("Zwischenverzeichnis im Ziel konnte nicht angelegt werden: {}", String::from_utf8_lossy(&made.stderr).trim());
    }
    println!("Übertrage …");
    match method {
        Method::Tar => upload_tar(&target, &opts.dist, &tmp, &plan)?,
        _ => upload_sftp(&target, &opts.dist, &tmp, &plan)?,
    }
    println!("Baue ein …");
    let script = install_script(&target.root, &tmp, &plan);
    target.run_with_input(target.command("sh -s"), |w| Ok(w.write_all(script.as_bytes())?))?;
    cleanup.active = false;
    println!("Fertig: {} Dateien übertragen (rund {mb} MB), {} gelöscht.", plan.upload_count(), plan.delete.len());
    Ok((plan, method))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    const INDEX: &str = "<html><head><link rel=\"canonical\" href=\"https://flauschiversum.de/\"></head><body>Start</body></html>";

    fn write(dir: &Path, rel: &str, data: &str) {
        let p = dir.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, data).unwrap();
    }

    /// Schreibt das Manifest passend zu den Dateien in `dist`.
    fn write_manifest(dist: &Path) {
        let mut m = BTreeMap::new();
        for e in walkdir::WalkDir::new(dist).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
            let rel = e.path().strip_prefix(dist).unwrap().to_string_lossy().to_string();
            if rel == MANIFEST {
                continue;
            }
            let data = fs::read(e.path()).unwrap();
            m.insert(format!("/{rel}"), Entry { hash: blake3::hash(&data).to_hex().to_string(), len: data.len() as u64 });
        }
        fs::write(dist.join(MANIFEST), serde_json::to_vec(&m).unwrap()).unwrap();
    }

    fn make_dist(dist: &Path) {
        let _ = fs::remove_dir_all(dist);
        write(dist, "index.html", INDEX);
        write(dist, "2020/10/kurbis/index.html", "<html>Kürbis</html>");
        write(dist, "2020/10/kurbis/200x/kürbis thumb.jpg", "jpg");
        write(dist, "pagefind/pagefind.js", "idx");
        write(dist, "style.css", "css");
        write(dist, "images/Flauschi.png", "png");
        write(dist, "rss.xml", "<rss/>");
        write_manifest(dist);
    }

    /// Ersatz für ssh (führt den Befehl lokal aus) und sftp (versteht pwd, -mkdir, put, chmod).
    fn fakes(dir: &Path) -> (String, String) {
        let ssh = dir.join("fake-ssh");
        fs::write(&ssh, "#!/bin/sh\nwhile [ \"${1#-}\" != \"$1\" ]; do [ \"$1\" = -o ] && shift; shift; done\nshift\nexec sh -c \"$*\"\n").unwrap();
        let sftp = dir.join("fake-sftp");
        fs::write(
            &sftp,
            "#!/bin/sh\nwhile IFS= read -r line; do case \"$line\" in\n pwd) echo ok;;\n -mkdir*) mkdir -p \"$(printf '%s' \"$line\" | sed 's/^-mkdir \"\\(.*\\)\"$/\\1/')\";;\n put*) a=$(printf '%s' \"$line\" | sed 's/^put \"\\(.*\\)\" \"\\(.*\\)\"$/\\1/'); b=$(printf '%s' \"$line\" | sed 's/^put \"\\(.*\\)\" \"\\(.*\\)\"$/\\2/'); cp \"$a\" \"$b\" || exit 1;;\n chmod*) m=$(printf '%s' \"$line\" | cut -d' ' -f2); f=$(printf '%s' \"$line\" | sed 's/^chmod [0-9]* \"\\(.*\\)\"$/\\1/'); chmod \"$m\" \"$f\";;\n *) echo \"?: $line\" >&2; exit 1;; esac; done\n",
        )
        .unwrap();
        for f in [&ssh, &sftp] {
            fs::set_permissions(f, fs::Permissions::from_mode(0o755)).unwrap();
        }
        (ssh.to_string_lossy().into(), sftp.to_string_lossy().into())
    }

    struct Env {
        _dir: tempfile::TempDir,
        base: PathBuf,
        dist: PathBuf,
        remote: PathBuf,
        ssh: String,
        sftp: String,
    }

    fn env() -> Env {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_path_buf();
        let (ssh, sftp) = fakes(&base);
        let dist = base.join("dist");
        let remote = base.join("server");
        fs::create_dir_all(&remote).unwrap();
        make_dist(&dist);
        Env { _dir: dir, base, dist, remote, ssh, sftp }
    }

    impl Env {
        fn opts(&self, method: Method, dry_run: bool) -> Options {
            Options {
                dist: self.dist.clone(),
                target: format!("fake:{}/", self.remote.display()),
                base_url: "https://flauschiversum.de".into(),
                dry_run,
                full: false,
                method,
                ssh: self.ssh.clone(),
                sftp: self.sftp.clone(),
            }
        }
        fn files(&self) -> Vec<(String, u32)> {
            let mut v: Vec<_> = walkdir::WalkDir::new(&self.remote)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
                .map(|e| (e.path().strip_prefix(&self.remote).unwrap().to_string_lossy().to_string(), e.metadata().unwrap().permissions().mode() & 0o777))
                .collect();
            v.sort();
            v
        }
    }

    #[test]
    fn deploys_with_tar_and_sftp_and_only_changes() {
        for method in [Method::Tar, Method::Sftp, Method::Auto] {
            let e = env();
            write(&e.remote, "google123.html", "verify");
            write(&e.remote, ".well-known/acme", "x");
            let (plan1, used) = run(&e.opts(method, false)).unwrap();
            assert_eq!(used, if method == Method::Sftp { Method::Sftp } else { Method::Tar });
            assert_eq!(plan1.upload_count(), 7);
            assert_eq!(fs::read_to_string(e.remote.join("2020/10/kurbis/200x/kürbis thumb.jpg")).unwrap(), "jpg");
            assert!(e.remote.join(MANIFEST).is_file());
            assert!(e.remote.join("google123.html").is_file() && e.remote.join(".well-known/acme").is_file(), "fremde Dateien bleiben");
            assert!(fs::read_dir(&e.remote).unwrap().all(|d| !d.unwrap().file_name().to_string_lossy().starts_with(".deploy-")), "kein Zwischenverzeichnis");
            let modes = e.files();
            for (path, mode) in &modes {
                if !path.starts_with(".well-known") && path != "google123.html" {
                    assert_eq!(*mode, 0o644, "{path}");
                }
            }
            assert_eq!(fs::metadata(e.remote.join("pagefind")).unwrap().permissions().mode() & 0o777, 0o755);

            // zweiter Lauf: nichts zu tun
            let (plan2, _) = run(&e.opts(method, false)).unwrap();
            assert_eq!((plan2.upload_count(), plan2.delete.len()), (0, 0));

            // Text geändert, Datei entfernt, neue Seite; Trockenlauf ändert nichts
            write(&e.dist, "2020/10/kurbis/index.html", "<html>Kürbis neu</html>");
            fs::remove_file(e.dist.join("images/Flauschi.png")).unwrap();
            fs::remove_dir(e.dist.join("images")).unwrap();
            write(&e.dist, "neu/index.html", "<html>neu</html>");
            write_manifest(&e.dist);
            let before = e.files();
            let (dry, _) = run(&e.opts(method, true)).unwrap();
            assert_eq!(before, e.files(), "Trockenlauf ändert nichts");
            assert_eq!((dry.upload_count(), dry.delete.len()), (2, 1));
            let (plan3, _) = run(&e.opts(method, false)).unwrap();
            assert_eq!((plan3.assets.len(), plan3.pages.len(), plan3.delete.len()), (0, 2, 1), "nur Seiten übertragen");
            assert!(fs::read_to_string(e.remote.join("2020/10/kurbis/index.html")).unwrap().contains("neu"));
            assert!(!e.remote.join("images/Flauschi.png").exists() && !e.remote.join("images").exists(), "entfernte Datei und leeres Verzeichnis sind weg");
            assert!(e.remote.join("google123.html").is_file() && e.remote.join(".well-known/acme").is_file(), "fremde Dateien bleiben");
        }
    }

    #[test]
    fn files_deleted_on_the_server_are_uploaded_again_even_if_the_manifest_remains() {
        for method in [Method::Tar, Method::Sftp] {
            let e = env();
            run(&e.opts(method, false)).unwrap();
            // „rm -rf *“ lässt Punktdateien stehen: das Manifest bleibt, alles andere ist weg
            for entry in fs::read_dir(&e.remote).unwrap() {
                let path = entry.unwrap().path();
                if path.file_name().unwrap().to_string_lossy().starts_with('.') {
                    continue;
                }
                if path.is_dir() { fs::remove_dir_all(&path).unwrap() } else { fs::remove_file(&path).unwrap() }
            }
            assert!(e.remote.join(MANIFEST).is_file() && !e.remote.join("index.html").exists());
            let (plan, _) = run(&e.opts(method, true)).unwrap();
            assert_eq!(plan.upload_count(), 7, "Trockenlauf erkennt fehlende Dateien");
            assert!(!e.remote.join("index.html").exists(), "Trockenlauf ändert nichts");
            let (plan, _) = run(&e.opts(method, false)).unwrap();
            assert_eq!(plan.upload_count(), 7);
            assert_eq!(fs::read_to_string(e.remote.join("2020/10/kurbis/200x/kürbis thumb.jpg")).unwrap(), "jpg");
            assert_eq!(run(&e.opts(method, false)).unwrap().0.upload_count(), 0, "danach ist alles auf dem Stand");
            // einzelne Datei fehlt
            fs::remove_file(e.remote.join("style.css")).unwrap();
            let (plan, _) = run(&e.opts(method, false)).unwrap();
            assert_eq!((plan.upload_count(), plan.assets.as_slice()), (1, &["style.css".to_string()][..]));
        }
    }

    /// Fehlen Tausende Dateien, ist die Antwort der Prüfung größer als ein Pipe-Puffer; das darf nicht hängen bleiben.
    #[test]
    fn thousands_of_missing_files_do_not_block_the_check() {
        let e = env();
        for i in 0..2000 {
            write(&e.dist, &format!("bilder/ein-ziemlich-langer-dateiname-fuer-den-test-der-eine-volle-pipe-erzeugt-{i:05}.jpg"), "x");
        }
        write_manifest(&e.dist);
        let (tx, rx) = std::sync::mpsc::channel();
        let opts = e.opts(Method::Tar, false);
        let remote = e.remote.clone();
        std::thread::spawn(move || {
            run(&opts).unwrap();
            for entry in fs::read_dir(&remote).unwrap() {
                let path = entry.unwrap().path();
                if !path.file_name().unwrap().to_string_lossy().starts_with('.') {
                    if path.is_dir() { fs::remove_dir_all(&path).unwrap() } else { fs::remove_file(&path).unwrap() }
                }
            }
            let plan = run(&opts).unwrap().0;
            tx.send(plan.upload_count()).unwrap();
        });
        let uploaded = rx.recv_timeout(Duration::from_secs(60)).expect("die Prüfung der fehlenden Dateien hängt");
        assert_eq!(uploaded, 2007);
    }

    #[test]
    fn full_uploads_everything_again() {
        let e = env();
        run(&e.opts(Method::Tar, false)).unwrap();
        fs::write(e.remote.join("style.css"), "von Hand geändert").unwrap();
        assert_eq!(run(&e.opts(Method::Tar, false)).unwrap().0.upload_count(), 0, "ohne --full unbemerkt");
        let mut o = e.opts(Method::Tar, false);
        o.full = true;
        let (plan, _) = run(&o).unwrap();
        assert_eq!(plan.upload_count(), 7);
        assert_eq!(fs::read_to_string(e.remote.join("style.css")).unwrap(), "css");
    }

    #[test]
    fn local_target_works_without_ssh() {
        let e = env();
        let mut o = e.opts(Method::Auto, false);
        o.target = e.remote.to_string_lossy().to_string();
        let (plan, method) = run(&o).unwrap();
        assert_eq!((plan.upload_count(), method), (7, Method::Tar));
        assert!(e.remote.join("index.html").is_file());
    }

    #[test]
    fn install_order_is_assets_then_pages_then_delete_then_manifest() {
        let plan = Plan {
            assets: vec!["images/a.png".into(), "style.css".into()],
            pages: vec!["a/index.html".into(), "rss.xml".into()],
            delete: vec!["old/deep/x.html".into()],
            bytes: 0,
        };
        let s = install_script("/httpdocs", ".deploy-1", &plan);
        let pos = |needle: &str| s.find(needle).unwrap_or_else(|| panic!("{needle} fehlt in\n{s}"));
        assert!(pos("images/a.png") < pos("style.css") && pos("style.css") < pos("a/index.html") && pos("a/index.html") < pos("rss.xml"));
        assert!(pos("rss.xml") < pos("rm -f -- 'old/deep/x.html'") && pos("rm -f -- 'old/deep/x.html'") < pos("rmdir 'old/deep'") && pos("rmdir 'old/deep'") < pos("rmdir 'old'"));
        assert!(s.rfind("mv -f").unwrap() == s.find("mv -f '.deploy-1/.flauschiversum-manifest'").unwrap(), "Manifest wird zuletzt verschoben");
        assert!(s.starts_with("set -e\ncd '/httpdocs'\n") && s.trim_end().ends_with("rm -rf '.deploy-1'"));
    }

    #[test]
    fn staging_noindex_wrong_canonical_and_missing_manifest_are_refused() {
        let e = env();
        let index = e.dist.join("index.html");
        fs::write(&index, INDEX.replace("</body>", "<div class=\"staging-bar\">Staging: x</div></body>")).unwrap();
        let err = run(&e.opts(Method::Tar, false)).unwrap_err().to_string();
        assert!(err.contains("Staging-Build"), "{err}");
        fs::write(&index, INDEX.replace("<head>", "<head><meta name=\"robots\" content=\"noindex\">")).unwrap();
        assert!(run(&e.opts(Method::Tar, false)).unwrap_err().to_string().contains("noindex"));
        fs::write(&index, INDEX.replace("https://flauschiversum.de/", "https://staging.flauschiversum.de/")).unwrap();
        assert!(run(&e.opts(Method::Tar, false)).unwrap_err().to_string().contains("gehört nicht zu"));
        fs::write(&index, INDEX).unwrap();
        fs::remove_file(e.dist.join(MANIFEST)).unwrap();
        assert!(run(&e.opts(Method::Tar, false)).is_err());
        assert!(fs::read_dir(&e.remote).unwrap().next().is_none(), "nichts übertragen");
    }

    #[test]
    fn dangerous_paths_in_the_server_manifest_are_refused() {
        let e = env();
        write(&e.base, "außen/datei.txt", "bleibt");
        fs::write(e.remote.join(MANIFEST), r#"{"/../außen/datei.txt": {"hash": "x", "len": 1}, "/index.html": {"hash": "x", "len": 1}}"#).unwrap();
        let err = run(&e.opts(Method::Tar, false)).unwrap_err().to_string();
        assert!(err.contains("unzulässiger Pfad"), "{err}");
        assert!(e.base.join("außen/datei.txt").is_file());
        assert!(!e.remote.join("index.html").exists());
    }

    #[test]
    fn unreachable_server_and_missing_ways_are_reported() {
        let e = env();
        let mut o = e.opts(Method::Auto, false);
        o.ssh = "/nicht/vorhanden".into();
        assert!(run(&o).unwrap_err().to_string().contains("nicht ohne Passwortabfrage erreichbar"));
        // ssh läuft, aber ohne tar und ohne SFTP
        let bare = e.base.join("ssh-ohne-werkzeuge");
        fs::write(&bare, "#!/bin/sh\nwhile [ \"${1#-}\" != \"$1\" ]; do [ \"$1\" = -o ] && shift; shift; done\nshift\nexec env PATH=/nonexistent /bin/sh -c \"$*\"\n").unwrap();
        fs::set_permissions(&bare, fs::Permissions::from_mode(0o755)).unwrap();
        let mut o = e.opts(Method::Auto, false);
        o.ssh = bare.to_string_lossy().into();
        o.sftp = "/nicht/vorhanden".into();
        let err = run(&o).unwrap_err().to_string();
        assert!(err.contains("kein Übertragungsweg nutzbar"), "{err}");
        assert!(fs::read_dir(&e.remote).unwrap().next().is_none(), "nichts im Ziel");
    }

    #[test]
    fn interrupted_transfer_leaves_manifest_and_no_temp_dir() {
        let e = env();
        run(&e.opts(Method::Tar, false)).unwrap();
        let old_manifest = fs::read(e.remote.join(MANIFEST)).unwrap();
        write(&e.dist, "style.css", "css neu");
        write_manifest(&e.dist);
        // tar auf dem Ziel schlägt fehl: ssh-Ersatz, der tar-Aufrufe ablehnt
        let broken = e.base.join("ssh-tar-kaputt");
        fs::write(&broken, "#!/bin/sh\nwhile [ \"${1#-}\" != \"$1\" ]; do [ \"$1\" = -o ] && shift; shift; done\nshift\ncase \"$*\" in tar*) cat >/dev/null; echo kaputt >&2; exit 1;; esac\nexec sh -c \"$*\"\n").unwrap();
        fs::set_permissions(&broken, fs::Permissions::from_mode(0o755)).unwrap();
        let mut o = e.opts(Method::Tar, false);
        o.ssh = broken.to_string_lossy().into();
        assert!(run(&o).is_err());
        assert_eq!(fs::read(e.remote.join(MANIFEST)).unwrap(), old_manifest, "Manifest unverändert");
        assert_eq!(fs::read_to_string(e.remote.join("style.css")).unwrap(), "css", "alte Seite bleibt");
        assert!(fs::read_dir(&e.remote).unwrap().all(|d| !d.unwrap().file_name().to_string_lossy().starts_with(".deploy-")), "Zwischenverzeichnis wurde entfernt");
    }

    #[test]
    fn parallel_deployments_are_refused() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".deploy.lock");
        let first = lock(&path).unwrap();
        assert!(lock(&path).unwrap_err().to_string().contains("läuft bereits"));
        drop(first);
        assert!(lock(&path).is_ok());
    }

    #[test]
    fn progress_line_shows_files_megabytes_percent_speed_and_eta() {
        let line = progress_line(4521, 10146, 212 * 1_048_576, 527 * 1_048_576, Duration::from_secs(10));
        assert_eq!(line, "4521/10146 Dateien · 212/527 MB (40 %) · 21,2 MB/s · noch 15 s");
        assert_eq!(progress_line(0, 3, 0, 0, Duration::ZERO), "0/3 Dateien · 0/0 MB (100 %)");
        assert!(progress_line(1, 2, 1_048_576, 400 * 1_048_576, Duration::from_secs(1)).contains("noch 6 min"));
        assert_eq!(duration_text(3700), "1 h 1 min");
    }

    #[test]
    fn targets_are_parsed() {
        let t = Target::parse("flauschiversum:/httpdocs/", "ssh", "sftp").unwrap();
        assert_eq!((t.host.as_deref(), t.root.as_str()), (Some("flauschiversum"), "/httpdocs"));
        let t = Target::parse("flauschiversum:flauschiversum-upload/", "ssh", "sftp").unwrap();
        assert_eq!(t.root, "flauschiversum-upload");
        let t = Target::parse("/var/www/", "ssh", "sftp").unwrap();
        assert!(t.host.is_none());
        assert!(Target::parse("host:/", "ssh", "sftp").is_err());
        assert!(Target::parse("host:/a b/", "ssh", "sftp").is_err());
    }
}
