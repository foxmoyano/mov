# Features SDD

Cada directorio representa una unidad independiente de intención, diseño, implementación y operación.

## Índice

| Feature | Estado | Descripción |
|---|---|---|
| [001-video-catalog](001-video-catalog/00-README.md) | Baseline implementado parcialmente | Listado, filtro y detalle de videos |

## Estructura obligatoria

```text
NNN-nombre-feature/
├── 00-README.md
├── 01-spec.md
├── 02-clarifications.md
├── 03-checklists/01-requirements.md
├── 04-research.md
├── 05-plan.md
├── 06-architecture.md
├── 07-data-model.md
├── 08-contracts/01-api.md
├── 09-ui/
│   ├── 01-wireframes.md
│   └── 02-process-flow.md
├── 10-configuration.md
├── 11-tasks.md
├── 12-analysis.md
├── 13-implementation.md
├── 14-testing/
│   ├── 01-test-strategy.md
│   ├── 02-test-cases.md
│   ├── 03-acceptance-tests.md
│   └── 04-non-functional-tests.md
├── 15-quickstart.md
├── 16-release.md
├── 17-deployment.md
└── 18-operations.md
```

Un artefacto que no aplica permanece con una justificación explícita. Los escenarios y casos de testing son documentales; no autorizan crear o ejecutar pruebas.
