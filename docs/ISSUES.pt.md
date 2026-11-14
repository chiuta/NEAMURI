# NEAMURI — primeiras dez tarefas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · **Português** · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Crie-as no GitHub com `scripts/create-issues.sh OWNER/REPO`. Comente numa issue para a reivindicar.

1. **Gramática de caminhos e forma canônica (meios-irmãos, parentes por afinidade, adoção, novo casamento)** — `good first issue`, `docs`, `area:model`
   Escreva `docs/PATHS.md` definindo cada `Step` e como normalizar caminhos equivalentes; adicione testes para 10 normalizações.

2. **Estenda `Kin` a primos de qualquer grau e remoção (primo de segundo grau uma vez removido…)** — `help wanted`, `area:model`
   Calcule grau/remoção a partir das distâncias ao ancestral comum mais próximo; testes contra um esquema publicado (cite-o).

3. **Romeno: complete a tabela (parentes por afinidade, bisavós, padrinhos como opcional)** — `good first issue`, `i18n`, `area:lang`
   Adicione os termos em falta com variantes de sexo; uma revisão de um falante nativo na conversa do PR.

4. **Inglês: primo removido uma vez, prefixos step- e half-** — `good first issue`, `i18n`, `area:lang`
   Tabela + testes; documente que a saída neutra em gênero é uma chave separada.

5. **Francês, italiano, espanhol, português, alemão, húngaro — uma língua por PR** — `good first issue`, `i18n`, `area:lang`
   Procuram-se falantes nativos. Escolha uma língua na issue; siga o formato de tabela da issue 7.

6. **Línguas em que a idade importa (húngaro báty/öcs, chinês paterno/materno, …): nota de projeto** — `help wanted`, `docs`, `area:model`
   Estenda `term` para aceitar a idade relativa e o lado da família sem quebrar os chamadores existentes.

7. **Tabelas de termos como arquivos de dados (`data/<lang>.toml`) e um validador** — `help wanted`, `area:data`
   Esquema, carregador na compilação, validador que liste as combinações parentesco/sexo em falta por língua.

8. **Integração com `ged_io`: relação entre dois ids de indivíduos** — `help wanted`, `area:gedcom`
   Atrás de uma funcionalidade do cargo; BFS até a ligação mais próxima; com vários caminhos devolva o mais curto.

9. **CLI: `neamuri relate FILE.ged ID1 ID2 --lang ro`** — `good first issue`, `area:cli`
   Depende da issue 8; imprima o termo e o caminho usado; erro claro para ids desconhecidos.

10. **Build WebAssembly e uma página de demonstração que se encaixe no aplicativo Arbore** — `help wanted`, `area:wasm`
   Página estática em `docs/demo/`; documente como o Arbore pode chamar o módulo.
