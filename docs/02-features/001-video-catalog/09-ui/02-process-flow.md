# Flujo de proceso

```text
Usuario -> filtros/paginador -> store -> VideosApi
       -> GET /videos -> handler -> service -> PostgreSQL
       <- PageResponse <- signals <- tabla
```

El detalle agrega consulta a MinIO antes de responder.
