# NEAMURI — Kinship engine: from a family-tree path to the right relative term, in many languages; works on top of GEDCOM parsers.

**English** · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version: 0.1.0-alpha.1 · Last updated: 2026-11-22

🌐 https://neamuri.chiuta.ro

## Status

Early skeleton (pre-alpha): the workspace builds and the tests pass in CI. Every planned feature is an open task in `docs/issues/`.

## Planned scope

- A path grammar with a canonical form (half-sibling, step-relatives, adoption, remarriage) and kin terms for cousins of any degree and removal.
- Term tables for Romanian, English, French, Italian, Spanish, Portuguese, German and Hungarian, stored as data files (`data/<lang>.toml`) with a validator.
- A design note for languages where age or side matters (Hungarian báty/öcs, Chinese paternal/maternal).
- Integration with `ged_io`: the relationship between two individual ids from a GEDCOM file.
- A CLI (`neamuri relate FILE.ged ID1 ID2 --lang ro`) and a WebAssembly demo page for the Arbore app.

None of this is implemented yet: each item is an open task in `docs/issues/`.

## Build and test

```bash
cargo test --workspace
cargo run -p neamuri-cli -- term ro female parent sibling
```

## Contribute

Read `CONTRIBUTING.md`, pick a task labelled `good first issue` from `docs/issues/` and open a pull request. No contribution is too small: a table row, a test or a translation counts.

## Independent implementation

Written from public specifications and textbooks; no third-party source code is copied (see `CLEANROOM.md`).

## License

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Support

Trade-free: no tracking, no ads. If it is useful, you can support the author: https://buymeacoffee.com/echo.of.the.strings
