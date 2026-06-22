# Arquitectura de 001-video-catalog

| Campo | Valor |
|---|---|
| Estado | Aprobado como baseline observado |
| Versión | 1.0.0 |
| Fecha | 2026-06-21 |
| Responsable | Equipo técnico |

## Contexto

```text
Usuario
  -> Frontend Angular 21
  -> API REST Rust/Axum
      -> PostgreSQL movie_db
      -> MinIO/S3
```

## Frontend

- `pages`: composición de cada vista.
- `components`: presentación y eventos.
- `*.store.ts`: estado de feature, carga y coordinación.
- `*.api.ts`: contrato HTTP tipado.
- `core`: modelos e interceptores transversales.

## Backend

- `routes`: path y método HTTP.
- `handlers`: extracción, validación y respuesta.
- `services`: lógica y coordinación de infraestructura.
- `dto`: contratos HTTP.
- `models`: filas persistidas.
- `db`: pool PostgreSQL.
- `storage`: acceso S3/MinIO.

## Flujo de listado

1. PrimeNG emite paginación.
2. `VideosStore` combina paginación y filtros.
3. `VideosApi` solicita `GET /api/v1/videos`.
4. Handler delega en `video_service`.
5. SQLx ejecuta conteo y datos con filtros equivalentes.
6. La respuesta actualiza signals y tabla.

## Flujo de detalle

1. Cliente solicita el UUID.
2. Backend obtiene metadatos de PostgreSQL.
3. `VideoImageService` consulta imagen principal y escenas.
4. El backend devuelve URLs firmadas o degrada imágenes a ausencia.

## Límites

- PostgreSQL es fuente de metadatos, no de binarios.
- MinIO es fuente de imágenes, no de identidad del video.
- El frontend no conoce credenciales de PostgreSQL o MinIO.
- Los DTO públicos no deben ser equivalentes por accidente a los models persistidos.
