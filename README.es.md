# NEAMURI — Motor de parentesco: de un camino en el árbol genealógico al término de parentesco correcto, en muchos idiomas; funciona sobre analizadores GEDCOM.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · **Español** · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versión: 0.1.0-alpha.1 · Última actualización: 2026-11-14

🌐 https://neamuri.chiuta.ro

## Estado

Esqueleto inicial (pre-alfa): el workspace compila y las pruebas pasan en CI. Cada función prevista es una tarea abierta en `docs/issues/`.

## Alcance previsto

- Una gramática de caminos con forma canónica (hermanastro, parientes políticos, adopción, segundas nupcias) y términos de parentesco para primos de cualquier grado y remoción.
- Tablas de términos para rumano, inglés, francés, italiano, español, portugués, alemán y húngaro, guardadas como archivos de datos (`data/<lang>.toml`) con validador.
- Una nota de diseño para las lenguas en las que importa la edad o el lado (húngaro báty/öcs, chino paterno/materno).
- Integración con `ged_io`: la relación entre dos identificadores de individuos de un archivo GEDCOM.
- Una CLI (`neamuri relate FILE.ged ID1 ID2 --lang ro`) y una página de demostración WebAssembly para la aplicación Arbore.

Nada de esto está implementado todavía: cada punto es una tarea abierta en `docs/issues/`.

## Compilar y probar

```bash
cargo test --workspace
cargo run -p neamuri-cli -- term ro female parent sibling
```

## Contribuir

Lee `CONTRIBUTING.md`, elige una tarea con la etiqueta `good first issue` en `docs/issues/` y abre un pull request. Ninguna contribución es demasiado pequeña: una fila de tabla, una prueba o una traducción cuentan.

## Implementación independiente

Escrita a partir de especificaciones públicas y manuales; no se ha copiado código fuente de terceros (véase `CLEANROOM.md`).

## Licencia

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoyo

Trade-free: sin rastreo ni publicidad. Si te resulta útil, puedes apoyar al autor: https://buymeacoffee.com/echo.of.the.strings
