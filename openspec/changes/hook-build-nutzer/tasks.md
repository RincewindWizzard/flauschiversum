## 1. Einstiegsbefehl und Hook

- [x] 1.1 `scripts/server/deploy-commit` (Argumentprüfung auf 40 Hexziffern, Konfiguration aus `/etc/flauschiversum/deploy.conf` bzw. `FLAUSCHI_CONF`, Sperre mit `flock`, `git archive` mit `safe.directory`, temporäres Verzeichnis, `cargo run --release --locked -- deploy` mit dauerhaftem Cache, Protokoll, Aufräumen) sowie `scripts/server/deploy.conf.example`; prüfen: `shellcheck` sauber, Tests mit Ersatz für `cargo`: gültiger Commit deployt, ungültige und unbekannte IDs werden abgewiesen, Aufräumen, Protokoll, Sperre
- [x] 1.2 `scripts/git-hooks/post-receive` ruft `sudo -n -u flauschiversum …/deploy-commit <ID>` auf (Ersatzprogramme über `DEPLOY_SUDO`, `DEPLOY_COMMIT_COMMAND`, `DEPLOY_USER`), Hinweis bei fehlschlagendem `sudo`; `scripts/test-git-hook.sh` anpassen (Push auf `master` übergibt die ID, `dev` nicht, Fehler erreichen den Pusher, fehlendes sudo meldet die Einrichtung); prüfen: `scripts/test-git-hook.sh` und `shellcheck` sauber
- [x] 1.3 `scripts/server/sudoers` (Regel für den Hook-Benutzer, Platzhalter für dessen Namen) ; prüfen: `visudo -cf` auf der Datei (mit eingesetztem Benutzernamen) meldet „parsed OK“, soweit `visudo` verfügbar ist

## 2. Anleitung

- [x] 2.1 README „Server für das Deployment einrichten“: alle Schritte aus der Design-Entscheidung 5 mit kopierbaren Befehlen, jeweils mit Benutzer (root, `flauschiversum`, `git`), Installation von Rust und Cargo per `rustup`, Kontrollbefehle nach jedem Schritt, Fehlersuche, Aktualisieren; den bisherigen Hook-Abschnitt anpassen; prüfen: alle Befehle der Anleitung stehen in der Reihenfolge, in der sie auf einem frischen Debian/Ubuntu funktionieren (Durchsicht, soweit ohne Server möglich), Pfade und Dateinamen stimmen mit den Skripten überein
- [ ] 2.2 Den Verweis von `production-deployment` („Deployment per Git-Hook“) auf `build-user` ergänzen, sobald beide Changes archiviert sind; prüfen: `openspec validate --specs --strict` ohne Fehler

## 3. Abnahme am Server (Betreiber)

- [ ] 3.1 Einrichtung nach der README auf dem echten Server, erster Lauf von Hand, dann ein Test-Push; prüfen: Seite liegt unter `/httpdocs/`, Protokoll vorhanden, `sudo -u git cat /var/share/flauschiversum/.ssh/deploy_key` wird verweigert

## Workflow follow-up

- Nach der Abnahme `/opsx:archive hook-build-nutzer` (nach `prod-deployment-ssh`) und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
