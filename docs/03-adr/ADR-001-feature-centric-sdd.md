# ADR-001: SDD centrado en features

| Campo | Valor |
|---|---|
| Estado | Aceptado |
| Fecha | 2026-06-21 |

## Contexto

La documentación estaba organizada en carpetas globales por fase. Esto facilitaba recorrer el ciclo, pero separaba requisitos, contratos, tareas y evidencia de una misma capacidad.

## Decisión

Mantener constitución, producto, ADR y manuales como artefactos globales. Mantener todos los artefactos evolutivos dentro de `docs/02-features/NNN-nombre-feature/`.

## Consecuencias

- La trazabilidad de una feature queda contenida en un paquete.
- Varias features pueden avanzar sin mezclar sus estados.
- Las decisiones compartidas requieren ADR o actualización de artefactos globales.
- Los índices globales sustituyen las antiguas carpetas por fase.
