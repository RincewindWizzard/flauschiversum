## 1. Deploy-Befehl im Werkzeug

- [x] 1.1 Unterbefehl `deploy` (`--target`/`DEPLOY_TARGET`, `--dry-run`, `--no-build`, `--method`, Basis-URL, Cache, Ausgabe wie `build`, `--staging` abgewiesen), Sperre `.deploy.lock`, Build ohne Staging, Prüfung von `dist/` (kein Staging, kein `noindex`, kanonische URL, Manifest); prüfen: `flauschiversum deploy --help`, Tests für Staging-Build, `noindex`, falsche URL und fehlendes Manifest
- [x] 1.2 Abgleich der Manifeste (nur geänderte Hashes, Manifest vom Server per `ssh cat`) und Pfadprüfung; Transport `tar` über SSH (Archiv aus den Crates `tar` und `flate2`, Modus 644) und SFTP (`-mkdir`/`put`/`chmod`), automatische Erkennung, Weg im Trockenlauf genannt; prüfen: Tests mit lokalem Ziel und Ersatzprogrammen für `ssh` und `sftp` (erstes Deployment, zweiter Lauf 0 Dateien, geänderter Text nur Seiten, beide Wege gleiches Ergebnis)
- [x] 1.3 Einbau als gerades Skript für `sh -s` (Verzeichnisse, erst Medien/Index dann Seiten per `mv -f`, Löschung veralteter Dateien und `rmdir`, Manifest zuletzt, Zwischenverzeichnis auch bei Fehlern entfernt); prüfen: Test der Reihenfolge, fremde Datei bleibt, `../`-Pfad im Server-Manifest wird abgewiesen, abgebrochener Transfer lässt Manifest und alte Seite unverändert
- [x] 1.4 Fehlerfälle und Hinweise: nicht erreichbarer Server, kein Übertragungsweg, Hinweis auf fremde Einträge beim ersten Mal, Trockenlauf; prüfen: Tests, Lauf gegen die echte Seite (10 146 Dateien) in ein lokales Ziel, zweiter Lauf überträgt nichts
- [ ] 1.5 Erkennungslauf am echten Server: `flauschiversum deploy --dry-run` zeigt, welchen Weg der Server erlaubt (Betreiber führt ihn aus, Ergebnis fließt in die README); prüfen: Weg ist `tar über SSH` oder `SFTP`

## 2. Automatisierung

- [x] 2.1 `scripts/smoke-test.sh` um `SMOKE_MODE=production` erweitern (kein `noindex`, keine Staging-Leiste, kanonische URL, `robots.txt`, Sitemap); prüfen: gegen eine lokale Produktions-`serve`-Instanz grün, gegen einen Staging-Build rot
- [x] 2.2 `.github/workflows/production.yml` (Trigger `master` und manuell, Tests, SSH-Umgebung aus Secrets, `flauschiversum deploy`, Smoke-Test, `concurrency` ohne Abbruch, `environment: production`, nur `contents: read`, Schlüssel am Ende entfernt); prüfen: `actionlint` sauber, SSH-Schritt lokal durchgespielt (Konfiguration, Rechte, Abbruch bei fehlendem Secret)
- [x] 2.3 `scripts/git-hooks/post-receive`, `scripts/install-git-hook.sh` und `scripts/test-git-hook.sh` (nur `master`, temporäres Arbeitsverzeichnis, dauerhaftes Zustandsverzeichnis, Protokoll, Meldung an den Pusher); prüfen: `scripts/test-git-hook.sh` und `shellcheck` sauber

## 3. Dokumentation

- [x] 3.1 README: Abschnitt „Produktiv-Deployment“ (Befehl, Optionen, Übertragungswege und warum kein `rsync` nötig ist, Voraussetzungen, SSH-Eintrag `flauschiversum`, Einrichtung der Action mit Secrets und Fingerabdruck-Prüfung, Hook-Installation, erstes Deployment mit `--dry-run`, Hinweis auf fremde Dateien), `scripts/` und `.github/workflows/` in der Tabelle; prüfen: Anleitung stimmt mit `--help` überein
- [x] 3.2 Die Spec `deployment` verweist im Purpose auf `production-deployment`; prüfen: `openspec validate --specs --strict` ohne Fehler

## Workflow follow-up

- Nach der Abnahme `/opsx:archive prod-deployment-ssh` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
- Einmalige Einrichtung durch den Betreiber (nicht Teil der Umsetzung): Schlüsselpaar, Secrets, `known_hosts`-Zeile prüfen, Branch-Schutz für `master` (siehe README).
