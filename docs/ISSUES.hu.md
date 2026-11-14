# NEAMURI — az első tíz feladat

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · **Magyar**

Hozd létre őket a GitHubon a `scripts/create-issues.sh OWNER/REPO` paranccsal. Szólj hozzá egy issue-hoz, hogy lefoglald.

1. **Útnyelvtan és kanonikus alak (féltestvérek, házassági rokonok, örökbefogadás, újraházasodás)** — `good first issue`, `docs`, `area:model`
   Írd meg a `docs/PATHS.md`-t, amely meghatározza az egyes `Step`-eket és az egyenértékű utak normalizálását; adj hozzá teszteket 10 normalizálásra.

2. **A `Kin` bővítése bármilyen fokú és eltávolítású unokatestvérekre (másodunokatestvér egyszer eltávolítva…)** — `help wanted`, `area:model`
   A fokot/eltávolítást a legközelebbi közös ősig mért távolságokból számold; tesztek egy közzétett ábra ellen (hivatkozz rá).

3. **Román: a táblázat kiegészítése (házassági rokonok, nagy-rokonok, keresztszülők opcionálisan)** — `good first issue`, `i18n`, `area:lang`
   Add hozzá a hiányzó kifejezéseket nemi változatokkal; egy anyanyelvi átnézés a PR szálában.

4. **Angol: egyszer eltávolított unokatestvér, step- és half- előtagok** — `good first issue`, `i18n`, `area:lang`
   Táblázat + tesztek; dokumentáld, hogy a nemsemleges kimenet külön kapcsoló.

5. **Francia, olasz, spanyol, portugál, német, magyar — nyelvenként egy PR** — `good first issue`, `i18n`, `area:lang`
   Anyanyelvi beszélőket keresünk. Válassz egy nyelvet az issue-ban; kövesd a 7. issue táblázatformátumát.

6. **Nyelvek, ahol a kor számít (magyar báty/öcs, kínai apai/anyai, …): tervezési jegyzet** — `help wanted`, `docs`, `area:model`
   Bővítsd a `term`-et, hogy fogadja a relatív kort és a családi oldalt a meglévő hívók megtörése nélkül.

7. **Kifejezéstáblák adatfájlként (`data/<lang>.toml`) és egy validátor** — `help wanted`, `area:data`
   Séma, fordításkori betöltő, validátor, amely nyelvenként felsorolja a hiányzó rokonsági/nemi kombinációkat.

8. **Integráció a `ged_io`-val: két személyazonosító közti kapcsolat** — `help wanted`, `area:gedcom`
   Cargo feature mögött; BFS a legközelebbi kapcsolatig; több út esetén a legrövidebbet add vissza.

9. **CLI: `neamuri relate FILE.ged ID1 ID2 --lang ro`** — `good first issue`, `area:cli`
   A 8. issue-tól függ; írd ki a kifejezést és a használt utat; egyértelmű hiba ismeretlen azonosítóra.

10. **WebAssembly build és egy demóoldal, amely az Arbore alkalmazáshoz csatlakozik** — `help wanted`, `area:wasm`
   Statikus oldal a `docs/demo/` alatt; dokumentáld, hogyan hívhatja az Arbore a modult.
