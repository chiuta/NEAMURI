# NEAMURI — die ersten zehn Aufgaben

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · **Deutsch** · [Magyar](ISSUES.hu.md)

Lege sie auf GitHub mit `scripts/create-issues.sh OWNER/REPO` an. Kommentiere ein Issue, um es zu übernehmen.

1. **Pfadgrammatik und kanonische Form (Halbgeschwister, Stief- und Schwiegerverwandte, Adoption, Wiederheirat)** — `good first issue`, `docs`, `area:model`
   `docs/PATHS.md` schreiben, das jeden `Step` definiert und erklärt, wie gleichwertige Pfade normalisiert werden; Tests für 10 Normalisierungen ergänzen.

2. **`Kin` auf Cousins beliebigen Grades und beliebiger Entfernung erweitern (Cousin zweiten Grades einmal entfernt…)** — `help wanted`, `area:model`
   Grad/Entfernung aus den Abständen zum nächsten gemeinsamen Vorfahren berechnen; Tests gegen ein veröffentlichtes Schaubild (zitieren).

3. **Rumänisch: die Tabelle vervollständigen (Schwiegerverwandte, Urverwandte, Paten optional)** — `good first issue`, `i18n`, `area:lang`
   Fehlende Begriffe mit Geschlechtsvarianten ergänzen; eine Durchsicht durch einen Muttersprachler im PR-Thread.

4. **Englisch: Cousin einmal entfernt, Präfixe step- und half-** — `good first issue`, `i18n`, `area:lang`
   Tabelle + Tests; dokumentieren, dass geschlechtsneutrale Ausgabe ein eigener Schalter ist.

5. **Französisch, Italienisch, Spanisch, Portugiesisch, Deutsch, Ungarisch — eine Sprache pro PR** — `good first issue`, `i18n`, `area:lang`
   Muttersprachler gesucht. Im Issue eine Sprache wählen; dem Tabellenformat aus Issue 7 folgen.

6. **Sprachen, in denen das Alter zählt (ungarisch báty/öcs, chinesisch väterlich/mütterlich, …): Designnotiz** — `help wanted`, `docs`, `area:model`
   `term` so erweitern, dass es relatives Alter und Familienseite entgegennimmt, ohne bestehende Aufrufer zu brechen.

7. **Begriffstabellen als Datendateien (`data/<lang>.toml`) und ein Validator** — `help wanted`, `area:data`
   Schema, Lader zur Kompilierzeit, Validator, der pro Sprache fehlende Verwandtschafts-/Geschlechtskombinationen auflistet.

8. **Integration mit `ged_io`: Beziehung zwischen zwei Personen-IDs** — `help wanted`, `area:gedcom`
   Hinter einem Cargo-Feature; BFS bis zur nächsten Verbindung; bei mehreren Pfaden den kürzesten zurückgeben.

9. **CLI: `neamuri relate FILE.ged ID1 ID2 --lang ro`** — `good first issue`, `area:cli`
   Hängt von Issue 8 ab; den Begriff und den verwendeten Pfad ausgeben; klarer Fehler bei unbekannten IDs.

10. **WebAssembly-Build und eine Demoseite, die sich in die Arbore-App einklinkt** — `help wanted`, `area:wasm`
   Statische Seite unter `docs/demo/`; dokumentieren, wie Arbore das Modul aufrufen kann.
