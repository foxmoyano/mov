# Contrato API

## Listado

`GET /api/v1/videos?page=0&size=10&title=texto`

Respuesta: `items`, `total`, `page`, `size`.

## Detalle

`GET /api/v1/videos/{uuid}`

Respuesta: metadatos, `main_image_url` y `scene_images`.

Errores observados: UUID inválido, `404` y `500`. POST, PUT y DELETE no están implementados.
