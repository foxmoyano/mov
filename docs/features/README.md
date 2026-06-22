# Features SDD

Cada directorio representa una unidad independiente de intención, diseño, implementación y operación.

## Índice

| Feature | Estado | Descripción |
|---|---|---|
| [001-video-catalog](001-video-catalog/README.md) | Baseline implementado parcialmente | Listado, filtro y detalle de videos |

## Estructura obligatoria

```text
NNN-nombre-feature/
├── README.md
├── spec.md
├── clarifications.md
├── checklists/requirements.md
├── research.md
├── plan.md
├── architecture.md
├── data-model.md
├── contracts/
├── ui/
├── configuration.md
├── tasks.md
├── analysis.md
├── implementation.md
├── testing/
│   ├── test-strategy.md
│   ├── test-cases.md
│   ├── acceptance-tests.md
│   └── non-functional-tests.md
├── quickstart.md
├── release.md
├── deployment.md
└── operations.md
```

Un artefacto que no aplica permanece con una justificación explícita. Los escenarios y casos de testing son documentales; no autorizan crear o ejecutar pruebas.
