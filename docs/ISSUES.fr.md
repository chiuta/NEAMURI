# NEAMURI — dix premières tâches

[English](ISSUES.md) · [Română](ISSUES.ro.md) · **Français** · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créez-les sur GitHub avec `scripts/create-issues.sh OWNER/REPO`. Commentez un ticket pour le réclamer.

1. **Grammaire des chemins et forme canonique (demi-frères, beaux-parents, adoption, remariage)** — `good first issue`, `docs`, `area:model`
   Écrire `docs/PATHS.md` définissant chaque `Step` et la façon de normaliser les chemins équivalents ; ajouter des tests pour 10 normalisations.

2. **Étendre `Kin` aux cousins de tout degré et éloignement (cousin au second degré une fois éloigné…)** — `help wanted`, `area:model`
   Calculer le degré/l’éloignement à partir des distances au plus proche ancêtre commun ; tests contre un schéma publié (le citer).

3. **Roumain : compléter la table (belle-famille, arrière-grands-parents, parrains/marraines en option)** — `good first issue`, `i18n`, `area:lang`
   Ajouter les termes manquants avec variantes de sexe ; une relecture par un locuteur natif dans le fil de la PR.

4. **Anglais : cousin éloigné une fois, préfixes step- et half-** — `good first issue`, `i18n`, `area:lang`
   Table + tests ; documenter que la sortie neutre quant au genre est un réglage séparé.

5. **Français, italien, espagnol, portugais, allemand, hongrois — une langue par PR** — `good first issue`, `i18n`, `area:lang`
   Locuteurs natifs recherchés. Choisir une langue dans le ticket ; suivre le format de table du ticket 7.

6. **Langues où l’âge compte (hongrois báty/öcs, chinois paternel/maternel, …) : note de conception** — `help wanted`, `docs`, `area:model`
   Étendre `term` pour prendre l’âge relatif et le côté de la famille sans casser les appelants existants.

7. **Tables de termes en fichiers de données (`data/<lang>.toml`) et un validateur** — `help wanted`, `area:data`
   Schéma, chargeur à la compilation, validateur listant les combinaisons parenté/sexe manquantes par langue.

8. **Intégration avec `ged_io` : relation entre deux identifiants d’individus** — `help wanted`, `area:gedcom`
   Derrière une fonctionnalité cargo ; BFS jusqu’au lien le plus proche ; en cas de chemins multiples, retourner le plus court.

9. **CLI : `neamuri relate FILE.ged ID1 ID2 --lang ro`** — `good first issue`, `area:cli`
   Dépend du ticket 8 ; afficher le terme et le chemin utilisé ; erreur claire pour les identifiants inconnus.

10. **Build WebAssembly et une page de démo qui se branche sur l’application Arbore** — `help wanted`, `area:wasm`
   Page statique sous `docs/demo/` ; documenter comment Arbore peut appeler le module.
