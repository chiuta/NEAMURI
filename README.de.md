# NEAMURI — Verwandtschafts-Engine: vom Pfad im Stammbaum zur richtigen Verwandtschaftsbezeichnung, in vielen Sprachen; setzt auf GEDCOM-Parsern auf.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · **Deutsch** · [Magyar](README.hu.md)

Version: 0.1.0-alpha.1 · Letzte Aktualisierung: 2026-11-14

🌐 https://neamuri.chiuta.ro

## Status

Frühes Grundgerüst (Pre-Alpha): Das Workspace baut, und die Tests laufen in der CI durch. Jede geplante Funktion ist eine offene Aufgabe in `docs/issues/`.

## Geplanter Umfang

- Eine Pfadgrammatik mit kanonischer Form (Halbgeschwister, angeheiratete Verwandte, Adoption, Wiederheirat) und Verwandtschaftsbezeichnungen für Cousins beliebigen Grades und beliebiger Entfernung.
- Begriffstabellen für Rumänisch, Englisch, Französisch, Italienisch, Spanisch, Portugiesisch, Deutsch und Ungarisch, als Datendateien (`data/<lang>.toml`) mit Validator.
- Eine Entwurfsnotiz für Sprachen, in denen Alter oder Seite zählt (ungarisch báty/öcs, chinesisch väterlich/mütterlich).
- Anbindung an `ged_io`: die Beziehung zwischen zwei Personen-IDs aus einer GEDCOM-Datei.
- Ein CLI (`neamuri relate FILE.ged ID1 ID2 --lang ro`) und eine WebAssembly-Demoseite für die App Arbore.

Nichts davon ist bisher umgesetzt: Jeder Punkt ist eine offene Aufgabe in `docs/issues/`.

## Bauen und testen

```bash
cargo test --workspace
cargo run -p neamuri-cli -- term ro female parent sibling
```

## Mitmachen

Lies `CONTRIBUTING.md`, wähle in `docs/issues/` eine Aufgabe mit dem Label `good first issue` und öffne einen Pull Request. Kein Beitrag ist zu klein: eine Tabellenzeile, ein Test oder eine Übersetzung zählt.

## Unabhängige Implementierung

Geschrieben nach öffentlichen Spezifikationen und Lehrbüchern; es wurde kein fremder Quellcode kopiert (siehe `CLEANROOM.md`).

## Lizenz

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Unterstützung

Trade-free: kein Tracking, keine Werbung. Wenn es dir nützt, kannst du den Autor unterstützen: https://buymeacoffee.com/echo.of.the.strings
