# Visión y requerimientos de producto

| Campo | Valor |
|---|---|
| Estado | En revisión, derivado del sistema construido |
| Versión | 0.2.0 |
| Fecha | 2026-06-21 |
| Responsable | Pendiente de validación de producto |

## Propósito

MOV permite consultar un catálogo de videos y sus metadatos sin acceder directamente a PostgreSQL ni al almacenamiento MinIO. El producto actual prioriza lectura, búsqueda y navegación paginada.

## Problema

Los usuarios necesitan identificar videos por título, revisar sus atributos técnicos y obtener imágenes asociadas mediante una interfaz web y una API estable. Los operadores necesitan configurar las conexiones de datos y almacenamiento sin exponer credenciales al frontend.

## Actores

- `ACT-001 Usuario de catálogo`: navega, filtra y consulta información de videos.
- `ACT-002 Cliente de API`: consume listado y detalle mediante HTTP.
- `ACT-003 Operador`: configura backend, PostgreSQL, MinIO y publicación del frontend.
- `ACT-004 Equipo de producto`: valida alcance, comportamiento y prioridades.

## Objetivos

- `OBJ-001`: presentar el catálogo mediante una vista paginada y localizada en español de Chile.
- `OBJ-002`: permitir localizar videos por coincidencia parcial de título.
- `OBJ-003`: exponer metadatos de detalle e imágenes temporales por UUID.
- `OBJ-004`: tolerar la ausencia de imágenes sin perder los metadatos disponibles.
- `OBJ-005`: mantener contratos explícitos entre Angular, Axum, PostgreSQL y MinIO.
- `OBJ-006`: operar el backend con configuración externa, observabilidad y cierre ordenado.

## Capacidades construidas

- Página principal de gestión de videos con ruta fallback.
- Tabla paginada con carga lazy, total y selección de 10, 20 o 50 filas.
- Filtro visual por título y año; solo el título tiene soporte backend.
- Listado backend paginado, filtrado y ordenado por título.
- Endpoint backend de detalle por UUID.
- Consulta de imagen principal y escenas en MinIO.
- URLs firmadas de lectura con duración actual de 3600 segundos.
- Normalización central de errores HTTP en frontend.
- Logging estructurado y apagado ordenado del backend.

## Capacidades parciales o no soportadas

- El detalle existe en API, pero no tiene página o navegación frontend.
- Crear, actualizar y eliminar están declarados en el cliente, pero no existen rutas backend.
- Nuevo Video y Exportar son controles visuales sin comportamiento implementado.
- Año se envía desde frontend, pero el backend lo ignora.
- La tabla muestra ordenamiento, pero el backend siempre ordena por título ascendente.
- El frontend no presenta una notificación visible desde el interceptor de errores.
- La configuración frontend de API no está externalizada para producción.
- CORS permite cualquier origen.
- No existe endpoint de healthcheck.

## Fuera de alcance vigente

- Escritura o carga de videos.
- Edición o eliminación.
- Exportación.
- Autenticación y autorización.
- Reproducción o streaming del archivo de video.
- Administración de imágenes desde la interfaz.
- Migraciones de base de datos.

## Restricciones

- Frontend Angular 21 standalone con PrimeNG y signals.
- Backend Rust 1.91, Axum 0.7, Tokio y SQLx/PostgreSQL.
- MinIO/S3 para imágenes asociadas.
- Sin dependencias nuevas sin autorización.
- Sin creación o ejecución de pruebas.
- Sin compilación como mecanismo de validación.

## Indicadores por definir

- Disponibilidad y latencia p50/p95 de listado y detalle.
- Tasa de `4xx` y `5xx` por endpoint.
- Errores PostgreSQL y MinIO.
- Porcentaje de videos sin imagen principal o escenas.
- Uso de filtros y paginación sin registrar valores sensibles.

Los umbrales requieren aprobación de producto y operación.
