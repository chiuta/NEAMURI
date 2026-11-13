# NEAMURI — architecture

**Scope decision:** NEAMURI does *not* parse GEDCOM — `ged_io` already does (5.5.1 and 7.0, read/write). NEAMURI answers one question:
"given the path between two people, what do I call that relative in language X?".
Two layers: `classify(path) -> Kin` (language-independent, extendable to degrees and removals) and `term(lang, kin, sex)` (currently a `match`; will become data tables, one file per language, with a validator).
Languages are the contribution surface: a native speaker can add a language without writing Rust logic.
