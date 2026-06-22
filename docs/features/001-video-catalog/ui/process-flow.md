# Flujos de 001-video-catalog

## Listado y filtro

```text
Usuario abre catálogo
  -> tabla emite lazy load
  -> store combina página y filtros
  -> API solicita GET /videos
  -> backend normaliza parámetros
  -> PostgreSQL devuelve total y página
  -> store actualiza signals
  -> tabla representa resultado
```

## Detalle backend

```text
Cliente solicita UUID
  -> backend consulta PostgreSQL
  -> no existe: 404
  -> existe: consulta main.jpg y escenas en MinIO
  -> firma URLs disponibles
  -> devuelve VideoDetail
```

## Fallos parciales

Un fallo de imágenes se registra y degrada a `main_image_url: null` o `scene_images: []`. Un fallo de PostgreSQL impide formar el detalle y produce error interno.
