# NEAMURI — Motor de parentesco: de um caminho na árvore genealógica ao termo de parentesco correto, em muitas línguas; funciona sobre analisadores GEDCOM.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · **Português** · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versão: 0.1.0-alpha.1 · Última atualização: 2026-11-22

🌐 https://neamuri.chiuta.ro

## Estado

Esqueleto inicial (pré-alfa): o workspace compila e os testes passam no CI. Cada funcionalidade prevista é uma tarefa aberta em `docs/issues/`.

## Âmbito previsto

- Uma gramática de caminhos com forma canónica (meio-irmão, parentes por afinidade, adoção, segundas núpcias) e termos de parentesco para primos de qualquer grau e remoção.
- Tabelas de termos para romeno, inglês, francês, italiano, espanhol, português, alemão e húngaro, guardadas como ficheiros de dados (`data/<lang>.toml`) com validador.
- Uma nota de conceção para as línguas em que a idade ou o lado conta (húngaro báty/öcs, chinês paterno/materno).
- Integração com `ged_io`: a relação entre dois identificadores de indivíduos de um ficheiro GEDCOM.
- Uma CLI (`neamuri relate FILE.ged ID1 ID2 --lang ro`) e uma página de demonstração WebAssembly para a aplicação Arbore.

Nada disto está ainda implementado: cada ponto é uma tarefa aberta em `docs/issues/`.

## Compilar e testar

```bash
cargo test --workspace
cargo run -p neamuri-cli -- term ro female parent sibling
```

## Contribuir

Leia `CONTRIBUTING.md`, escolha uma tarefa com a etiqueta `good first issue` em `docs/issues/` e abra um pull request. Nenhuma contribuição é demasiado pequena: uma linha de tabela, um teste ou uma tradução contam.

## Implementação independente

Escrita a partir de especificações públicas e manuais; não foi copiado código-fonte de terceiros (ver `CLEANROOM.md`).

## Licença

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoio

Trade-free: sem rastreamento nem publicidade. Se lhe for útil, pode apoiar o autor: https://buymeacoffee.com/echo.of.the.strings
