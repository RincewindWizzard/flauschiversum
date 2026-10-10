//! Sammeln von Fehlern und Warnungen mit Bezug auf die betroffene Datei.

use std::fmt;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Level {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: Level,
    /// Relativer Pfad der betroffenen Datei.
    pub file: String,
    pub message: String,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match self.level {
            Level::Error => "Fehler",
            Level::Warning => "Warnung",
        };
        write!(f, "{level}: {}: {}", self.file, self.message)
    }
}

/// Threadsichere Sammlung von Diagnosen.
#[derive(Default)]
pub struct Diagnostics {
    items: Mutex<Vec<Diagnostic>>,
}

impl Diagnostics {
    pub fn error(&self, file: impl Into<String>, message: impl Into<String>) {
        self.push(Level::Error, file.into(), message.into());
    }

    pub fn warning(&self, file: impl Into<String>, message: impl Into<String>) {
        self.push(Level::Warning, file.into(), message.into());
    }

    fn push(&self, level: Level, file: String, message: String) {
        self.items.lock().unwrap().push(Diagnostic { level, file, message });
    }

    pub fn has_errors(&self) -> bool {
        self.items.lock().unwrap().iter().any(|d| d.level == Level::Error)
    }

    /// Alle Diagnosen, sortiert nach Datei für eine stabile Ausgabe.
    pub fn all(&self) -> Vec<Diagnostic> {
        let mut items = self.items.lock().unwrap().clone();
        items.sort_by(|a, b| (&a.file, &a.message).cmp(&(&b.file, &b.message)));
        items
    }

    pub fn count(&self, level: Level) -> usize {
        self.items.lock().unwrap().iter().filter(|d| d.level == level).count()
    }
}
