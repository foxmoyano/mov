# Manual funcional

## Listado

El frontend solicita páginas al endpoint `GET /api/v1/videos`. El backend normaliza página y tamaño, filtra opcionalmente por título y devuelve elementos junto con el total.

## Detalle

`GET /api/v1/videos/{id}` obtiene metadatos PostgreSQL y busca imágenes en MinIO. La imagen principal y las escenas se entregan mediante URLs temporales. La ausencia de imágenes no impide devolver el detalle.

## Errores

- UUID inválido: rechazo de solicitud.
- Video inexistente: `404`.
- Error de persistencia: `500` con mensaje público acotado.
- Error de imágenes: degradación a ausencia de imagen cuando el video existe.

## Capacidades no soportadas

Crear, actualizar, eliminar y exportar videos no forman parte del contrato backend vigente.
