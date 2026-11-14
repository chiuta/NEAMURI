# NEAMURI — Motore di parentela: da un percorso nell'albero genealogico al termine di parentela corretto, in molte lingue; funziona sopra i parser GEDCOM.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · **Italiano** · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versione: 0.1.0-alpha.1 · Ultimo aggiornamento: 2026-11-14

🌐 https://neamuri.chiuta.ro

## Stato

Scheletro iniziale (pre-alfa): il workspace compila e i test passano in CI. Ogni funzione prevista è un'attività aperta in `docs/issues/`.

## Ambito previsto

- Una grammatica dei percorsi con forma canonica (fratellastro, parenti acquisiti, adozione, seconde nozze) e termini di parentela per cugini di qualsiasi grado e rimozione.
- Tabelle di termini per romeno, inglese, francese, italiano, spagnolo, portoghese, tedesco e ungherese, conservate come file di dati (`data/<lang>.toml`) con validatore.
- Una nota di progettazione per le lingue in cui contano l’età o il lato (ungherese báty/öcs, cinese paterno/materno).
- Integrazione con `ged_io`: il rapporto tra due id di individui di un file GEDCOM.
- Una CLI (`neamuri relate FILE.ged ID1 ID2 --lang ro`) e una pagina dimostrativa WebAssembly per l’app Arbore.

Nulla di tutto ciò è ancora implementato: ogni voce è un compito aperto in `docs/issues/`.

## Compilare e testare

```bash
cargo test --workspace
cargo run -p neamuri-cli -- term ro female parent sibling
```

## Contribuire

Leggi `CONTRIBUTING.md`, scegli un'attività con etichetta `good first issue` in `docs/issues/` e apri una pull request. Nessun contributo è troppo piccolo: una riga di tabella, un test o una traduzione contano.

## Implementazione indipendente

Scritta a partire da specifiche pubbliche e manuali; nessun codice sorgente di terzi è stato copiato (vedi `CLEANROOM.md`).

## Licenza

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Sostegno

Trade-free: niente tracciamento, niente pubblicità. Se ti è utile, puoi sostenere l'autore: https://buymeacoffee.com/echo.of.the.strings
