# Estrategia de testing documental de 001-video-catalog

| Campo | Valor |
|---|---|
| Estado | Especificado, no ejecutado |
| Versión | 0.2.0 |
| Fecha | 2026-06-21 |

## Objetivo

Definir qué comportamientos y riesgos deberían verificarse sin crear ni ejecutar pruebas en este workspace.

## Niveles documentados

- Componente frontend: filtros, paginación y estados visuales.
- Integración HTTP: serialización de query y respuestas.
- Backend: normalización de parámetros, SQL y manejo de errores.
- Integración PostgreSQL/MinIO: datos y degradación parcial.
- Aceptación: historias `US-001` a `US-007`.
- No funcional: seguridad, accesibilidad, observabilidad y rendimiento.

## Cobertura

Cada `AC-*` referencia un caso `TC-*`; cada `NFR-*` referencia un caso `NFT-*`. Los requisitos funcionales pueden agruparse en un caso cuando forman un mismo escenario. Los casos permanecen `No ejecutado` salvo evidencia externa.

## Restricciones

- No generar archivos `.spec.ts`, módulos `#[cfg(test)]`, fixtures, mocks o scripts.
- No ejecutar runners, compiladores, servicios o consultas para afirmar resultados.
- No convertir checklists Markdown en automatización.
