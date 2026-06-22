# Quickstart documental de 001-video-catalog

Este recorrido explica el comportamiento esperado. No contiene comandos, no inicia servicios y no constituye una prueba ejecutada.

## Precondiciones conceptuales

- Frontend publicado con URL de API correcta.
- Backend configurado con PostgreSQL `movie_db` y bucket MinIO.
- Tabla `videos` compatible con `07-data-model.md`.

## Recorrido esperado

1. Abrir la ruta principal.
2. Observar la primera página con un máximo de 10 videos.
3. Filtrar por parte de un título y observar reinicio a página cero.
4. Limpiar filtros y observar el catálogo completo.
5. Solicitar un UUID existente mediante el contrato de detalle.
6. Observar metadatos y URLs temporales si existen imágenes.
7. Solicitar un UUID ausente y obtener `404`.

Los resultados reales solo pueden registrarse si un proceso externo autorizado aporta evidencia.
