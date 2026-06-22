# Modelo de datos

`public.videos` usa UUID como clave primaria y contiene título, títulos alternativos, descripción, archivo, extensión, tamaño, duración, resolución, altura, fechas e imagen.

La estructura debe contrastarse exclusivamente con `postgres-mov`. Cambios futuros requieren migración y rollback documentados.
