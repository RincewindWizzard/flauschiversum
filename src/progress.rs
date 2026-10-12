//! Fortschrittsmeldungen auf der Konsole (stderr), damit ein langer Kaltstart nicht wie ein Hänger wirkt.
//! Nur aktiv, wenn `enable()` aufgerufen wurde (die Kommandos `build`, `check` und `serve`); in Tests still.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Instant;

static ENABLED: AtomicBool = AtomicBool::new(false);
static STATE: Mutex<Option<(Instant, Instant)>> = Mutex::new(None);
/// Zeitpunkt der letzten Zählermeldung in Millisekunden seit Beginn.
static LAST_COUNT_MS: AtomicU64 = AtomicU64::new(0);

/// Mindestabstand zwischen zwei Zählermeldungen.
const COUNT_INTERVAL_MS: u64 = 2000;

pub fn enable() {
    ENABLED.store(true, Ordering::Relaxed);
}

/// Beginnt einen neuen Lauf: setzt die Uhr zurück.
pub fn begin() {
    let now = Instant::now();
    *STATE.lock().unwrap() = Some((now, now));
    LAST_COUNT_MS.store(0, Ordering::Relaxed);
}

fn line(prefix: &str, message: &str) {
    let guard = STATE.lock().unwrap();
    let Some((start, _)) = *guard else { return };
    eprintln!("[{:>6.1} s] {prefix}{message}", start.elapsed().as_secs_f64());
}

/// Meldet den Beginn eines Arbeitsschritts und, wie lange der vorherige gedauert hat.
pub fn step(message: &str) {
    if !ENABLED.load(Ordering::Relaxed) {
        return;
    }
    let mut guard = STATE.lock().unwrap();
    let Some((start, last)) = *guard else { return };
    let previous = last.elapsed().as_secs_f64();
    *guard = Some((start, Instant::now()));
    let elapsed = start.elapsed().as_secs_f64();
    drop(guard);
    if previous >= 0.5 {
        eprintln!("[{elapsed:>6.1} s] {message}   (vorheriger Schritt: {previous:.1} s)");
    } else {
        eprintln!("[{elapsed:>6.1} s] {message}");
    }
}

/// Meldet einen Zwischenstand, höchstens alle zwei Sekunden (und immer beim letzten Element).
pub fn count(label: &str, done: usize, total: usize, detail: impl FnOnce() -> String) {
    if !ENABLED.load(Ordering::Relaxed) {
        return;
    }
    let elapsed_ms = match *STATE.lock().unwrap() {
        Some((start, _)) => start.elapsed().as_millis() as u64,
        None => return,
    };
    let last = LAST_COUNT_MS.load(Ordering::Relaxed);
    let due = done == total || elapsed_ms.saturating_sub(last) >= COUNT_INTERVAL_MS;
    if due && LAST_COUNT_MS.compare_exchange(last, elapsed_ms, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
        let percent = if total == 0 { 100 } else { done * 100 / total };
        line("    ", &format!("{label}: {done}/{total} ({percent} %) {}", detail()));
    }
}

/// Hinweis zu einem Schritt, eingerückt unter der letzten Schrittmeldung.
pub fn note(message: &str) {
    if ENABLED.load(Ordering::Relaxed) {
        line("    ", message);
    }
}
