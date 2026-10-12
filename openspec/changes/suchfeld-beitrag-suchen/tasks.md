## 1. Text im Suchfeld

- [ ] 1.1 In `templates/layout.html` den Platzhalter von `#q-top` auf „Beitrag suchen …“ und das `sr-only`-Label auf „Beitrag suchen“ ändern; prüfen: `grep` findet „Anleitung suchen“ weder im Template noch in den Snapshots
- [ ] 1.2 Snapshots aller Seiten aktualisieren und im Browser-Test (`tests/browser/topbar.mjs`) Platzhalter und zugänglichen Namen prüfen; prüfen: `cargo test --release -- --include-ignored` und `node tests/browser/topbar.mjs`, `layout.mjs` grün

## Workflow follow-up

- Nach der Abnahme `/opsx:archive suchfeld-beitrag-suchen` und danach den Git-Tag `changes/<Archivordner-Name>` setzen.
