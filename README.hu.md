# NEAMURI — Rokonsági motor: a családfabeli útvonalból a megfelelő rokonsági megnevezés, sok nyelven; GEDCOM-elemzők fölött működik.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · **Magyar**

Verzió: 0.1.0-alpha.1 · Utolsó frissítés: 2026-11-14

🌐 https://neamuri.chiuta.ro

## Állapot

Korai váz (pre-alfa): a workspace lefordul, a tesztek a CI-ben lefutnak. Minden tervezett funkció nyitott feladat a `docs/issues/` mappában.

## Tervezett terjedelem

- Útvonalnyelvtan kanonikus alakkal (féltestvér, házassági rokonok, örökbefogadás, újraházasodás) és rokonsági megnevezések tetszőleges fokú és eltávolítású unokatestvérekhez.
- Kifejezéstáblák román, angol, francia, olasz, spanyol, portugál, német és magyar nyelvhez, adatfájlként (`data/<lang>.toml`), ellenőrzővel.
- Tervezési jegyzet olyan nyelvekhez, ahol számít az életkor vagy az oldal (magyar báty/öcs, kínai apai/anyai).
- Integráció a `ged_io`-val: két egyén azonosítója közötti kapcsolat egy GEDCOM-fájlból.
- CLI (`neamuri relate FILE.ged ID1 ID2 --lang ro`) és WebAssembly demóoldal az Arbore alkalmazáshoz.

Ezek közül még semmi sincs megvalósítva: minden pont nyitott feladat a `docs/issues/` mappában.

## Fordítás és tesztelés

```bash
cargo test --workspace
cargo run -p neamuri-cli -- term ro female parent sibling
```

## Közreműködés

Olvasd el a `CONTRIBUTING.md` fájlt, válassz egy `good first issue` címkéjű feladatot a `docs/issues/` mappából, és nyiss pull requestet. Nincs túl kicsi hozzájárulás: egy táblázatsor, egy teszt vagy egy fordítás is számít.

## Független megvalósítás

Nyilvános specifikációk és tankönyvek alapján íródott; harmadik fél forráskódját nem másoltuk (lásd `CLEANROOM.md`).

## Licenc

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Támogatás

Trade-free: nincs követés, nincs reklám. Ha hasznos számodra, támogathatod a szerzőt: https://buymeacoffee.com/echo.of.the.strings
