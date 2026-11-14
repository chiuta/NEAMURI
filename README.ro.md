# NEAMURI — Motor de rudenie: dintr-un traseu în arborele genealogic la termenul corect de rudenie, în multe limbi; funcționează peste parsere GEDCOM.

[English](README.md) · **Română** · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versiune: 0.1.0-alpha.1 · Ultima actualizare: 2026-11-14

🌐 https://neamuri.chiuta.ro

## Stare

Schelet timpuriu (pre-alfa): workspace-ul se compilează, iar testele trec în CI. Fiecare funcție planificată este o sarcină deschisă în `docs/issues/`.

## Domeniu planificat

- O gramatică a traseelor cu formă canonică (frate vitreg, rude prin alianță, adopție, recăsătorire) și termeni de rudenie pentru veri de orice grad și îndepărtare.
- Tabele de termeni pentru română, engleză, franceză, italiană, spaniolă, portugheză, germană și maghiară, păstrate ca fișiere de date (`data/<lang>.toml`) cu validator.
- O notă de proiectare pentru limbile în care contează vârsta sau linia (maghiară báty/öcs, chineză paternă/maternă).
- Integrare cu `ged_io`: relația dintre doi indivizi după id, dintr-un fișier GEDCOM.
- Un CLI (`neamuri relate FILE.ged ID1 ID2 --lang ro`) și o pagină demo WebAssembly pentru aplicația Arbore.

Nimic din asta nu este implementat încă: fiecare punct este o sarcină deschisă în `docs/issues/`.

## Compilare și testare

```bash
cargo test --workspace
cargo run -p neamuri-cli -- term ro female parent sibling
```

## Contribuie

Citește `CONTRIBUTING.md`, alege o sarcină etichetată `good first issue` din `docs/issues/` și deschide un pull request. Nicio contribuție nu e prea mică: un rând de tabel, un test sau o traducere contează.

## Implementare independentă

Scrisă pornind de la specificații publice și manuale; nu s-a copiat cod sursă de la terți (vezi `CLEANROOM.md`).

## Licență

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Sprijin

Trade-free: fără urmărire, fără reclame. Dacă ți-e de folos, poți sprijini autorul: https://buymeacoffee.com/echo.of.the.strings
