# NEAMURI — primeras diez tareas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · **Español** · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créalas en GitHub con `scripts/create-issues.sh OWNER/REPO`. Comenta en una issue para reclamarla.

1. **Gramática de rutas y forma canónica (medio hermanos, parentesco político, adopción, segundas nupcias)** — `good first issue`, `docs`, `area:model`
   Escribe `docs/PATHS.md` definiendo cada `Step` y cómo normalizar rutas equivalentes; añade pruebas para 10 normalizaciones.

2. **Amplía `Kin` a primos de cualquier grado y remoción (primo segundo una vez removido…)** — `help wanted`, `area:model`
   Calcula grado/remoción a partir de las distancias al antepasado común más cercano; pruebas contra un esquema publicado (cítalo).

3. **Rumano: completa la tabla (parentesco político, bisabuelos, padrinos como opcional)** — `good first issue`, `i18n`, `area:lang`
   Añade los términos que faltan con variantes de sexo; una revisión de un hablante nativo en el hilo del PR.

4. **Inglés: primo removido una vez, prefijos step- y half-** — `good first issue`, `i18n`, `area:lang`
   Tabla + pruebas; documenta que la salida neutra en cuanto al género es un interruptor aparte.

5. **Francés, italiano, español, portugués, alemán, húngaro — un idioma por PR** — `good first issue`, `i18n`, `area:lang`
   Se buscan hablantes nativos. Elige un idioma en la issue; sigue el formato de tabla de la issue 7.

6. **Idiomas donde importa la edad (húngaro báty/öcs, chino paterno/materno, …): nota de diseño** — `help wanted`, `docs`, `area:model`
   Amplía `term` para aceptar la edad relativa y el lado de la familia sin romper a los llamadores existentes.

7. **Tablas de términos como archivos de datos (`data/<lang>.toml`) y un validador** — `help wanted`, `area:data`
   Esquema, cargador en compilación, validador que liste las combinaciones parentesco/sexo que faltan por idioma.

8. **Integración con `ged_io`: relación entre dos ids de individuos** — `help wanted`, `area:gedcom`
   Tras una funcionalidad de cargo; BFS hasta la conexión más cercana; con varias rutas devuelve la más corta.

9. **CLI: `neamuri relate FILE.ged ID1 ID2 --lang ro`** — `good first issue`, `area:cli`
   Depende de la issue 8; imprime el término y la ruta usada; error claro para ids desconocidos.

10. **Compilación WebAssembly y una página de demostración que se enchufe a la aplicación Arbore** — `help wanted`, `area:wasm`
   Página estática en `docs/demo/`; documenta cómo puede Arbore llamar al módulo.
