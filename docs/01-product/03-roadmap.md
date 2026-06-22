# Roadmap SDD

| Campo | Valor |
|---|---|
| Estado | Borrador |
| Versión | 0.1.0 |
| Fecha | 2026-06-21 |

El roadmap expresa intención y prioridad; no autoriza implementación.

| Orden | Feature candidata | Objetivo | Dependencias |
|---:|---|---|---|
| 001 | `video-catalog` | Formalizar catálogo y detalle existentes | Baseline actual |
| 002 | `contract-alignment` | Alinear modelos, filtros y métodos soportados | 001 |
| 003 | `video-detail-ui` | Hacer observable el detalle en frontend | 002 |
| 004 | `filter-and-sort` | Resolver año y orden remoto | 002 |
| 005 | `runtime-configuration` | Externalizar API URL y restringir CORS | 002 |
| 006 | `deployment-baseline` | Definir infraestructura, health y rollback | 005 |

Cada candidata debe obtener número definitivo y crear su paquete mediante las plantillas antes de cambiar código.
