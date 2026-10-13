## 1. Deploy-Skript

- [ ] 1.1 `scripts/deploy.sh` (`--dry-run`, `--no-build`, `DEPLOY_TARGET`, `FLAUSCHI_BASE_URL`, `FLAUSCHI_CACHE_DIR`): Sperre, Produktionsbuild mit `check` und `build`, Prüfung von `dist/` (kein Staging, kanonische URL, Manifest), SSH- und `rsync`-Prüfung, lokales Ziel ohne SSH; prüfen: `shellcheck` ohne Befunde, Aufruf mit Staging-`dist/` bricht ab
- [ ] 1.2 Manifest-Vergleich (nur geänderte Hashes, Manifest vom Server per `ssh cat`), Transport austauschbar (`tar` über SSH, SFTP, `rsync` falls vorhanden; `DEPLOY_METHOD`, automatische Erkennung, Weg im Trockenlauf genannt) in ein Zwischenverzeichnis, gemeinsamer Einbauschritt (Rechte 644/755, Nicht-HTML vor HTML per `mv -f`, Manifest zuletzt, Aufräumen per `trap`); prüfen: im Test zweiter Lauf ohne Änderung überträgt 0 Dateien, geänderter Text nur HTML, kein Zwischenverzeichnis bleibt zurück, bei jedem Transport dasselbe Ergebnis
- [ ] 1.3 Löschung nur über das Manifest des Servers samt Pfadprüfung und `rmdir` leerer Verzeichnisse, Hinweis auf fremde Einträge beim ersten Mal, `--dry-run` zeigt beides; prüfen: fremde Datei bleibt, entfernte Seite verschwindet, `../`-Pfad wird abgewiesen
- [ ] 1.4 `scripts/test-deploy.sh` (erstes Deployment, Folge-Deployment, Änderung, Entfernung, fremde Datei, Staging-Abbruch, Trockenlauf, Transport `tar` und `sftp` mit einem lokalen Ersatz für `ssh`/`sftp` im Test) gegen ein temporäres Verzeichnis mit der Test-Site; prüfen: Skript endet mit 0, Fehlerfälle werden erkannt (Gegenprobe)

- [ ] 1.5 Erkennungslauf am echten Server: `scripts/deploy.sh --dry-run` zeigt, welche Werkzeuge der Server hat und welchen Weg es wählt (Betreiber führt ihn aus, Ergebnis fließt in die README); prüfen: Weg ist `tar` oder `sftp`, kein Abbruch wegen `rsync`

## 2. Automatisierung

- [ ] 2.1 `scripts/smoke-test.sh` um `SMOKE_MODE=production` erweitern (kein `noindex`, keine Staging-Leiste, kanonische URL, `robots.txt`, Sitemap); prüfen: gegen eine lokale Produktions-`serve`-Instanz grün, gegen einen Staging-Build rot
- [ ] 2.2 `.github/workflows/production.yml` (Trigger `master` und manuell, Tests, SSH-Umgebung aus Secrets, `deploy.sh`, Smoke-Test, `concurrency` ohne Abbruch, `environment: production`, nur `contents: read`); prüfen: `actionlint` (falls verfügbar) bzw. YAML-Prüfung, Schlüssel wird nie ausgegeben, Datei nach dem Lauf entfernt
- [ ] 2.3 `scripts/git-hooks/post-receive` und `scripts/install-git-hook.sh` (nur `master`, temporäres Arbeitsverzeichnis, dauerhafte Caches, Protokoll, Meldung an den Pusher); prüfen: Test mit einem lokalen Bare-Repository: Push auf `master` deployed in ein lokales Ziel, Push auf `dev` nicht

## 3. Dokumentation

- [ ] 3.1 README: Abschnitt „Produktiv-Deployment“ (Skript, Optionen, Übertragungswege und warum kein `rsync` nötig ist (netcup), Voraussetzungen, SSH-Eintrag `flauschiversum`, Einrichtung der Action mit Secrets und `ssh-keyscan`-Prüfung, Hook-Installation, erstes Deployment mit `--dry-run`, Hinweis auf fremde Dateien und das Aufräumen alter Inhalte), `scripts/` und `.github/workflows/` in der Tabelle; prüfen: Anleitung stimmt mit `--help` des Skripts überein
- [ ] 3.2 Die Spec `deployment` verweist im Purpose auf `production-deployment`; prüfen: `openspec validate --specs --strict` ohne Fehler

## Workflow follow-up

- Nach der Abnahme `/opsx:archive prod-deployment-ssh` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
- Einmalige Einrichtung durch den Betreiber (nicht Teil der Umsetzung): Schlüsselpaar, Secrets, `known_hosts`-Zeile prüfen, Branch-Schutz für `master` (siehe README).
