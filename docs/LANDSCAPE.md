# Related projects and where NEAMURI fits

*Checked with a limited web search on 13 November 2026. Facts change quickly: re-check before announcing or depending on anything below.*

- **ged_io** (Rust, MIT) — full GEDCOM 5.5.1/7.0 parser and writer with streaming: <https://github.com/ge3224/ged-io>. NEAMURI should consume it, not duplicate it.
- **Other Rust crates** found (`family_graph`, `gedcom-rs`) build graphs or parse GEDCOM; none provides localised kinship terms in a limited search.
- **Elsewhere:** a Dart package `relationships` computes relationship labels with localised descriptions (<https://pub.dev/packages/relationships>), which shows the need; the Rust side looks open.
- **Gap targeted:** a data-driven kinship-term engine for many languages (age-relative, paternal/maternal and in-law distinctions included) with a CLI and WASM build.
