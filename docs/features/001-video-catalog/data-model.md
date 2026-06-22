# Modelo de datos de 001-video-catalog

| Campo | Valor |
|---|---|
| Estado | Aprobado como baseline observado |
| Versión | 1.0.0 |
| Fecha | 2026-06-21 |
| Fuente | `postgres-mov`, base `movie_db` |

## Tabla `public.videos`

| Columna | Tipo | Nulo | Default | Uso |
|---|---|---:|---|---|
| `id` | `uuid` | No | `uuid_generate_v4()` | Clave primaria |
| `title` | `text` | No | - | Título principal |
| `title_alternative` | `text` | Sí | - | Título alternativo |
| `description` | `text` | Sí | - | Descripción |
| `name_movie` | `text` | Sí | - | Nombre del archivo/recurso |
| `extension` | `varchar(10)` | Sí | - | Extensión |
| `size_mb` | `numeric(10,2)` | Sí | - | Tamaño en MB |
| `duration_seconds` | `integer` | Sí | - | Duración |
| `resolution` | `varchar(20)` | Sí | - | Etiqueta de resolución |
| `video_height` | `varchar(10)` | Sí | - | Altura almacenada como texto |
| `published_at` | `timestamp` | Sí | - | Fecha de publicación |
| `updated_at` | `timestamp` | Sí | `CURRENT_TIMESTAMP` | Actualización |
| `created_at` | `timestamp` | Sí | `CURRENT_TIMESTAMP` | Creación |
| `image_url` | `varchar(2000)` | Sí | - | Referencia de imagen persistida |

## Transformaciones relevantes

- `size_mb`: PostgreSQL numeric → Rust `Decimal` en listado → número en contrato de detalle.
- `video_height`: PostgreSQL/Rust texto → entero opcional en detalle.
- Campos nulos de listado permanecen opcionales en backend; el frontend debe representarlos sin asumir valor.

## Objetos MinIO

```text
media/{video-uuid}/main.jpg
media/{video-uuid}/images/{scene-file}
```

## Reglas

- Toda modificación debe definir migración, compatibilidad, backfill y rollback antes de cambiar schema.
- La estructura se vuelve a contrastar exclusivamente con `postgres-mov`.
- No documentar credenciales ni datos reales de usuarios.
