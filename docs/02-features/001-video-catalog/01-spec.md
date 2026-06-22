# 001 - Catálogo de videos

## US-001 Acceder al catálogo

- `FR-001`: la raíz presenta Gestión de Videos.
- `FR-002`: rutas desconocidas redirigen a la raíz.
- `FR-003`: el frontend usa locale `es-CL`.

## US-002 Consultar videos

- `FR-004`: usar `GET /api/v1/videos`.
- `FR-005`: responder `items`, `total`, `page` y `size`.
- `FR-006`: presentar título, extensión, tamaño, duración, resolución, altura y fecha.
- `FR-007`: representar carga y resultado vacío.

## US-003 Paginar

- `FR-008`: usar página base cero.
- `FR-009`: tamaño por defecto 10, mínimo efectivo 10 y máximo 100.
- `FR-010`: ofrecer 10, 20 y 50 filas en UI.
- `FR-011`: ordenar por título ascendente.

## US-004 Filtrar

- `FR-012`: filtrar título mediante coincidencia parcial sin distinguir caso.
- `FR-013`: recortar espacios y omitir filtro vacío.
- `FR-014`: reiniciar página al buscar o limpiar.
- `FR-015`: año está visible, pero no forma parte del contrato backend.

## US-005 Consultar detalle

- `FR-016`: usar `GET /api/v1/videos/{id}` con UUID.
- `FR-017`: devolver metadatos, imagen principal y escenas.
- `FR-018`: recurso ausente devuelve `404`.
- `FR-019`: imagen principal usa `media/{uuid}/main.jpg`.
- `FR-020`: escenas usan `media/{uuid}/images/`.
- `FR-021`: URLs firmadas expiran actualmente en 3600 segundos.
- `FR-022`: fallos de imágenes degradan a `null` o lista vacía.

## US-006 Gestionar errores

- `FR-023`: frontend normaliza errores a `message`, `status` y `code`.
- `FR-024`: loading vuelve a falso al completar o fallar.
- `FR-025`: backend registra errores técnicos y devuelve mensajes acotados.

## US-007 Operar backend

- `FR-026`: PostgreSQL y MinIO se configuran por variables de entorno.
- `FR-027`: HOST y PORT usan `0.0.0.0:3000` por defecto.
- `FR-028`: el pool usa máximo cinco conexiones.
- `FR-029`: el servidor soporta cierre ordenado.

## Requisitos no funcionales

- `NFR-001`: no exponer secretos ni stack traces.
- `NFR-002`: logging estructurado sin datos sensibles.
- `NFR-003`: conservar stack y versiones existentes.
- `NFR-004`: mantener accesibilidad de tabla y filtros.
- `NFR-005`: conteo y datos usan filtros equivalentes.
- `NFR-006`: configuración productiva fuera del código desplegable.

## Brechas

- Modelo frontend incompleto frente al backend.
- CRUD declarado en frontend sin rutas backend.
- Año y orden visual sin contrato integral.
- Nuevo Video y Exportar sin comportamiento.
- Detalle sin ruta frontend.
- CORS abierto, errores con formatos distintos y sin healthcheck.
