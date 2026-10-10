# Spec Delta

## ADDED Requirements

### Requirement: Nur veröffentlichte Posts
Für einen Post mit einem Datum in der Zukunft (bezogen auf den Zeitpunkt des Builds) SHALL weder eine Seite noch eine Bildvariante noch ein anderes Medium veröffentlicht werden. Er erscheint erst nach einem Build, der nach seinem Datum stattfindet.

#### Scenario: Geplanter Post
- **WHEN** ein Post das Datum von morgen hat und heute gebaut wird
- **THEN** existiert seine URL im Ausgabeverzeichnis nicht

#### Scenario: Geplanter Post wird fällig
- **WHEN** derselbe Post nach seinem Datum erneut gebaut wird
- **THEN** ist er unter seiner URL erreichbar und erscheint auf den Indexseiten

### Requirement: Navigation zwischen Posts
Unter dem Inhalt SHALL die Post-Seite Links auf den nächstälteren und den nächstneueren veröffentlichten Post zeigen, jeweils mit deren Titel. Beim ältesten bzw. neuesten Post entfällt der jeweilige Link.

#### Scenario: Neuester Post
- **WHEN** die Seite des neuesten Posts geöffnet wird
- **THEN** gibt es einen Link auf den nächstälteren Post und keinen auf einen neueren
