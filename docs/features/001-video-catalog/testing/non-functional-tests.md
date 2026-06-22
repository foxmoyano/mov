# Pruebas no funcionales documentales de 001-video-catalog

| ID | Requisito | Verificación esperada | Criterio | Estado |
|---|---|---|---|---|
| `NFT-001` | `NFR-001` | Revisar exposición de errores | Sin credenciales, stack traces o errores de proveedor | No ejecutado |
| `NFT-002` | `NFR-002` | Revisar eventos operacionales | Inicio, UUID, conteos y errores con logging estructurado | Revisión documental |
| `NFT-003` | `NFR-003` | Revisar compatibilidad declarada | Stack coincide con manifiestos | Revisión documental |
| `NFT-004` | `NFR-004` | Revisar semántica y foco | Labels, scope y navegación comprensible | No ejecutado |
| `NFT-005` | `NFR-005` | Revisar consistencia de consultas | COUNT y datos aplican filtro equivalente | Revisión documental |
| `NFT-006` | `NFR-006` | Revisar configuración | Secretos externos; API URL por entorno | Parcial |
| `NFT-007` | `NFR-007` | Revisar degradación de MinIO | Metadatos disponibles pese a imágenes ausentes | Revisión documental |
| `NFT-008` | `NFR-008` | Revisar localización | Locale `es-CL` registrado y usado por pipes | Revisión documental |
| `NFT-009` | `NFR-009` | Revisar privacidad de logs | Sin secretos ni URLs firmadas completas | No ejecutado |
| `NFT-010` | `NFR-001` | Revisar CORS | Solo orígenes y métodos aprobados | Bloqueado por `Q-003` |
| `NFT-011` | SLO pendiente | Evaluar latencia p95 | Umbral pendiente de `Q-007` | Bloqueado |

No se ejecutan herramientas de carga, escáneres, compiladores o servidores desde este flujo.
