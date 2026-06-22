# Arquitectura

```text
Usuario -> Angular 21 -> API Rust/Axum -> PostgreSQL movie_db
                                      -> MinIO/S3
```

El frontend no accede directamente a PostgreSQL ni MinIO. El backend coordina metadatos e imágenes.
