# Contrato HTTP de 001-video-catalog

| Campo | Valor |
|---|---|
| Estado | Implementado con brechas documentadas |
| Versión | 1.1.0 |
| Fecha | 2026-06-21 |
| Base path | `/api/v1` |

## CTR-001 Listar videos

`GET /api/v1/videos`

### Query params

| Nombre | Tipo | Obligatorio | Regla |
|---|---|---:|---|
| `page` | integer | No | Default 0; mínimo efectivo 0 |
| `size` | integer | No | Default 10; máximo efectivo 100 |
| `title` | string | No | Contiene, sin distinguir caso; vacío se ignora |

`year`, parámetros de orden y otros query params no forman parte del contrato backend actual.

### Respuesta `200`

```json
{
  "items": [
    {
      "id": "uuid",
      "title": "string",
      "extension": "string|null",
      "size_mb": 123.45,
      "published_at": "2026-01-01T12:00:00",
      "duration_seconds": 120,
      "resolution": "1080p",
      "video_height": "1080",
      "image_url": null
    }
  ],
  "total": 0,
  "page": 0,
  "size": 10
}
```

### Errores observados

- `500`: error interno de consulta. El formato actual es texto plano y queda pendiente normalizarlo.

## CTR-002 Obtener detalle

`GET /api/v1/videos/{id}`

### Path params

| Nombre | Tipo | Obligatorio |
|---|---|---:|
| `id` | UUID | Sí |

### Respuesta `200`

```json
{
  "id": "uuid",
  "title": "string",
  "extension": "string",
  "size_mb": 0,
  "published_at": "datetime|null",
  "duration_seconds": "integer|null",
  "resolution": "string|null",
  "video_height": "integer|null",
  "image_url": "string|null",
  "main_image_url": "string|null",
  "scene_images": []
}
```

### Errores observados

- `400`: UUID inválido, gestionado por rechazo del extractor de Axum.
- `404`: video inexistente; texto plano actual.
- `500`: fallo al obtener metadatos; texto plano actual.

Los fallos de imagen principal o escenas se degradan a `null` o lista vacía y no cambian por sí solos el estado HTTP cuando el video existe.

## Política HTTP observada

- CORS permite actualmente cualquier origen.
- Métodos permitidos por la capa CORS: GET, POST, PUT, DELETE y OPTIONS.
- Las únicas rutas del recurso implementadas son los dos GET documentados.
- Headers permitidos: `Content-Type` y `Authorization`.
- El backend responde errores propios como texto plano; el frontend está preparado además para objetos con `message` y `code`.

## Contratos no implementados

`POST`, `PUT` y `DELETE /api/v1/videos` no están implementados en backend. Su presencia en `VideosApi` no autoriza su consumo.
