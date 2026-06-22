# Plan técnico de 001-video-catalog

| Campo | Valor |
|---|---|
| Estado | En revisión |
| Versión | 0.2.0 |
| Fecha | 2026-06-21 |
| Responsable | Pendiente de asignación |

## Objetivo

Formalizar y mantener el baseline construido bajo `FR-001` a `FR-048`, sin incorporar las brechas como alcance implícito.

## Plan por capa

### Frontend

- Mantener la composición página → filtros/dashboard → store → API.
- Conservar ruta raíz, fallback, locale `es-CL`, estado de carga y paginación lazy.
- Tratar la alineación del modelo `Video` y su nulabilidad como una feature posterior.
- No habilitar acciones visuales sin especificación y contrato backend.
- Resolver configuración de API por entorno antes de despliegue productivo.

### Backend

- Mantener rutas GET y validación de paginación.
- Conservar filtro por título y orden ascendente fijo mientras el contrato no cambie.
- Consolidar errores públicos sin filtrar detalles internos.
- Mantener fallos de imágenes como degradación parcial del detalle.
- Definir CORS por entorno antes de producción.

### Datos

- Conservar `videos.id` como UUID.
- No alterar schema sin una especificación de migración y rollback.
- Alinear nulabilidad y conversiones entre PostgreSQL, Rust y TypeScript.

### Almacenamiento

- Conservar prefijos de objetos mientras formen parte del contrato operativo.
- Mantener URLs firmadas temporales y secretos solo en variables de entorno.

### Despliegue

- Publicar artefactos externos ya construidos por un proceso autorizado.
- Desplegar dependencias antes que backend y frontend.
- Definir plataforma, frontend hosting, networking, healthchecks y rollback antes de ejecutar.

## Secuencia de features recomendada

1. `002-contract-alignment`: alinear DTO frontend/backend y retirar métodos no soportados.
2. `003-video-detail-ui`: especificar y presentar detalle.
3. `004-filter-and-sort`: decidir año y orden remoto.
4. `005-runtime-configuration`: externalizar configuración frontend y CORS.
5. `006-deployment-baseline`: definir infraestructura, health y observabilidad.

Cada feature debe completar su propio paquete en `docs/02-features/` antes de modificar código.
