# NEAMURI — primele zece sarcini

[English](ISSUES.md) · **Română** · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creează-le pe GitHub cu `scripts/create-issues.sh OWNER/REPO`. Comentează la un issue ca să-l revendici.

1. **Gramatica căilor și forma canonică (frați vitregi, rude prin alianță, adopție, recăsătorire)** — `good first issue`, `docs`, `area:model`
   Scrie `docs/PATHS.md` definind fiecare `Step` și cum se normalizează căile echivalente; adaugă teste pentru 10 normalizări.

2. **Extinde `Kin` la veri de orice grad și îndepărtare (văr de gradul doi o dată îndepărtat…)** — `help wanted`, `area:model`
   Calculează gradul/îndepărtarea din distanțele până la cel mai apropiat strămoș comun; teste față de o diagramă publicată (citează-o).

3. **Română: completează tabelul (rude prin alianță, străbuni, nași ca opțional)** — `good first issue`, `i18n`, `area:lang`
   Adaugă termenii lipsă cu variante de sex; o revizuire de la un vorbitor nativ în firul PR-ului.

4. **Engleză: văr îndepărtat o dată, prefixele step- și half-** — `good first issue`, `i18n`, `area:lang`
   Tabel + teste; documentează că ieșirea neutră ca gen este un comutator separat.

5. **Franceză, italiană, spaniolă, portugheză, germană, maghiară — o limbă per PR** — `good first issue`, `i18n`, `area:lang`
   Se caută vorbitori nativi. Alege o limbă în issue; urmează formatul de tabel din issue-ul 7.

6. **Limbi în care contează vârsta (maghiară báty/öcs, chineză pe linie paternă/maternă, …): notă de proiectare** — `help wanted`, `docs`, `area:model`
   Extinde `term` să primească vârsta relativă și latura familiei fără a strica apelanții existenți.

7. **Tabele de termeni ca fișiere de date (`data/<lang>.toml`) și un validator** — `help wanted`, `area:data`
   Schemă, încărcător la compilare, validator care listează combinațiile rudă/sex lipsă pentru fiecare limbă.

8. **Integrare cu `ged_io`: relația dintre doi identificatori de indivizi** — `help wanted`, `area:gedcom`
   În spatele unei funcționalități cargo; BFS până la cea mai apropiată legătură; la căi multiple o returnează pe cea mai scurtă.

9. **CLI: `neamuri relate FILE.ged ID1 ID2 --lang ro`** — `good first issue`, `area:cli`
   Depinde de issue-ul 8; afișează termenul și calea folosită; eroare clară pentru identificatori necunoscuți.

10. **Build WebAssembly și o pagină demo care se conectează la aplicația Arbore** — `help wanted`, `area:wasm`
   Pagină statică în `docs/demo/`; documentează cum poate Arbore apela modulul.
