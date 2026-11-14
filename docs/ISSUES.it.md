# NEAMURI — primi dieci compiti

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · **Italiano** · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creale su GitHub con `scripts/create-issues.sh OWNER/REPO`. Commenta una issue per prenderla in carico.

1. **Grammatica dei percorsi e forma canonica (fratellastri, parenti acquisiti, adozione, nuove nozze)** — `good first issue`, `docs`, `area:model`
   Scrivi `docs/PATHS.md` definendo ogni `Step` e come normalizzare i percorsi equivalenti; aggiungi test per 10 normalizzazioni.

2. **Estendi `Kin` ai cugini di qualsiasi grado e rimozione (cugino di secondo grado una volta rimosso…)** — `help wanted`, `area:model`
   Calcola grado/rimozione dalle distanze dal più vicino antenato comune; test contro uno schema pubblicato (citalo).

3. **Rumeno: completa la tabella (parenti acquisiti, bisnonni, padrini come opzione)** — `good first issue`, `i18n`, `area:lang`
   Aggiungi i termini mancanti con le varianti di sesso; una revisione di un madrelingua nel thread della PR.

4. **Inglese: cugino rimosso una volta, prefissi step- e half-** — `good first issue`, `i18n`, `area:lang`
   Tabella + test; documenta che l’output neutro rispetto al genere è un’opzione separata.

5. **Francese, italiano, spagnolo, portoghese, tedesco, ungherese — una lingua per PR** — `good first issue`, `i18n`, `area:lang`
   Cercasi madrelingua. Scegli una lingua nella issue; segui il formato di tabella della issue 7.

6. **Lingue in cui conta l’età (ungherese báty/öcs, cinese paterno/materno, …): nota di progetto** — `help wanted`, `docs`, `area:model`
   Estendi `term` per accettare l’età relativa e il lato della famiglia senza rompere i chiamanti esistenti.

7. **Tabelle di termini come file di dati (`data/<lang>.toml`) e un validatore** — `help wanted`, `area:data`
   Schema, caricatore in compilazione, validatore che elenca le combinazioni parentela/sesso mancanti per lingua.

8. **Integrazione con `ged_io`: relazione tra due id di individui** — `help wanted`, `area:gedcom`
   Dietro una funzionalità cargo; BFS fino al legame più vicino; con più percorsi restituisci il più breve.

9. **CLI: `neamuri relate FILE.ged ID1 ID2 --lang ro`** — `good first issue`, `area:cli`
   Dipende dalla issue 8; stampa il termine e il percorso usato; errore chiaro per id sconosciuti.

10. **Build WebAssembly e una pagina demo che si collega all’app Arbore** — `help wanted`, `area:wasm`
   Pagina statica in `docs/demo/`; documenta come Arbore può chiamare il modulo.
