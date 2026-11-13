# NEAMURI — first ten tasks

Create them on GitHub with `scripts/create-issues.sh OWNER/REPO`. Comment on an issue to claim it.

1. **Path grammar and canonical form (half-sibling, step-relatives, adoption, remarriage)** — `good first issue`, `docs`, `area:model`
   Write `docs/PATHS.md` defining each `Step` and how to normalise equivalent paths; add tests for 10 normalisations.

2. **Extend `Kin` to cousins of any degree and removal (second cousin once removed…)** — `help wanted`, `area:model`
   Compute degree/removal from the lowest common ancestor distances; tests against a published chart (cite it).

3. **Romanian: complete the table (in-laws, great-relatives, godparents as optional)** — `good first issue`, `i18n`, `area:lang`
   Add missing terms with sex variants; one native-speaker review in the PR thread.

4. **English: cousin once removed, step- and half- prefixes** — `good first issue`, `i18n`, `area:lang`
   Table + tests; document that gender-neutral output is a separate switch.

5. **French, Italian, Spanish, Portuguese, German, Hungarian — one language per PR** — `good first issue`, `i18n`, `area:lang`
   Native speakers wanted. Pick one language in the issue; follow the table format from issue 7.

6. **Languages where age matters (Hungarian báty/öcs, Chinese paternal/maternal, …): design note** — `help wanted`, `docs`, `area:model`
   Extend `term` to take relative age and side of family without breaking existing callers.

7. **Term tables as data files (`data/<lang>.toml`) and a validator** — `help wanted`, `area:data`
   Schema, loader at build time, validator that lists missing kin/sex combinations per language.

8. **Integration with `ged_io`: relation between two individual ids** — `help wanted`, `area:gedcom`
   Behind a cargo feature; BFS to the closest connection; handle multiple paths by returning the shortest.

9. **CLI: `neamuri relate FILE.ged ID1 ID2 --lang ro`** — `good first issue`, `area:cli`
   Depends on issue 8; output the term and the path used; clear error for unknown ids.

10. **WebAssembly build and a demo page that plugs into the Arbore app** — `help wanted`, `area:wasm`
   Static page under `docs/demo/`; document how Arbore can call the module.
