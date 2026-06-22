# Investigación de 001-video-catalog

| Campo | Valor |
|---|---|
| Estado | Aprobado como baseline observado |
| Versión | 1.0.0 |
| Fecha | 2026-06-21 |
| Responsable | Equipo técnico |

## Método SDD

Se adopta el flujo de GitHub Spec Kit: constitución, especificación, aclaración, plan, tareas, análisis e implementación. Se agregan release, despliegue y operación para cubrir el ciclo solicitado. Los artefactos Markdown se mantienen en el repositorio y alimentan la siguiente etapa.

## Frontend observado

- Angular 21 standalone, TypeScript estricto y PrimeNG.
- Ruta principal única para gestión de videos.
- Estado en signals mediante `VideosStore`.
- Integración HTTP encapsulada en `VideosApi`.
- Paginación lazy y filtros por título/año en la interfaz.
- Base URL en `src/environment/environment.ts`.

## Backend observado

- Rust edition 2021 y MSRV 1.91.
- Axum/Tokio con estado compartido para PostgreSQL y S3.
- SQLx con consultas parametrizadas.
- Separación routes, handlers, services, DTO, models y storage.
- CORS actualmente permite cualquier origen.
- Dockerfile multi-stage solo para backend.

## Persistencia observada

La base `movie_db` contiene la tabla `public.videos`. Su clave primaria es UUID y sus campos cubren título, descripción, nombre de archivo, extensión, tamaño, duración, resolución, altura, fechas e imagen. La fuente de esta observación es `postgres-mov`.

## Almacenamiento observado

- Imagen principal: `media/{uuid}/main.jpg`.
- Escenas: `media/{uuid}/images/`.
- URLs firmadas de lectura con expiración actual de 3600 segundos.

## Decisiones de baseline

- `DEC-001`: conservar el stack y patrones actuales.
- `DEC-002`: tratar los endpoints GET como único contrato implementado.
- `DEC-003`: documentar CRUD de cliente, filtro por año y orden remoto como brechas.
- `DEC-004`: no instalar Spec Kit; adoptar su proceso mediante Markdown versionado.
- `DEC-005`: adaptar checklists y aceptación a revisión documental por las restricciones constitucionales.

## Referencias

- GitHub Spec Kit: <https://github.com/github/spec-kit>
- Metodología SDD: <https://github.com/github/spec-kit/blob/main/spec-driven.md>
