# Aclaraciones de 001-video-catalog

| Campo | Valor |
|---|---|
| Estado | En revisión |
| Versión | 0.2.0 |
| Fecha | 2026-06-21 |
| Responsable | Pendiente de validación de producto |

## Decisiones confirmadas por el sistema construido

| ID | Pregunta | Respuesta observada | Impacto |
|---|---|---|---|
| `CLR-001` | ¿Cuál es la base MOV? | `movie_db`, consultada exclusivamente con `postgres-mov` | Datos |
| `CLR-002` | ¿Cuál es el recurso persistido? | `public.videos` | Datos y contrato |
| `CLR-003` | ¿Qué operaciones backend existen? | Listado y detalle mediante GET | Alcance |
| `CLR-004` | ¿Cuál es la base de rutas? | `/api/v1/videos` | Contrato |
| `CLR-005` | ¿Cómo se pagina? | Página base cero; default 10; máximo 100 | Contrato |
| `CLR-006` | ¿Cómo se filtra? | Solo título, `ILIKE` por contenido | Contrato |
| `CLR-007` | ¿Cómo se ordena? | Título ascendente fijo | Contrato |
| `CLR-008` | ¿Cómo se identifican detalles? | UUID | Contrato |
| `CLR-009` | ¿Dónde están las imágenes? | `media/{uuid}/main.jpg` y `media/{uuid}/images/` | MinIO |
| `CLR-010` | ¿Cuánto dura una URL firmada? | 3600 segundos | Seguridad/operación |
| `CLR-011` | ¿Qué ocurre si faltan imágenes? | El detalle se conserva con `null` o lista vacía | Resiliencia |
| `CLR-012` | ¿Se pueden crear o ejecutar pruebas? | No | Constitución |
| `CLR-013` | ¿Se puede compilar para validar? | No | Constitución |

## Preguntas abiertas bloqueantes para nuevas capacidades

| ID | Pregunta | Afecta | Estado |
|---|---|---|---|
| `Q-001` | ¿Qué usuarios o roles pueden consultar el catálogo? | Seguridad | Abierta |
| `Q-002` | ¿Cuál es la plataforma objetivo de despliegue? | Despliegue | Abierta |
| `Q-003` | ¿Qué orígenes CORS se permiten por entorno? | Seguridad | Abierta |
| `Q-004` | ¿El filtro por año debe implementarse o retirarse? | Contrato/UI | Abierta |
| `Q-005` | ¿El ordenamiento debe ser remoto y qué campos admite? | Contrato/UI | Abierta |
| `Q-006` | ¿Nuevo Video y Exportar pertenecen al roadmap? | Alcance | Abierta |
| `Q-007` | ¿Qué SLOs de disponibilidad y latencia se requieren? | Operación | Abierta |
| `Q-008` | ¿Cómo se publica y configura el frontend en producción? | Despliegue | Abierta |
| `Q-009` | ¿Debe existir una vista frontend de detalle? | UX | Abierta |
| `Q-010` | ¿Cuál será el formato de error HTTP común? | Contrato | Abierta |
| `Q-011` | ¿Se requiere healthcheck y qué dependencias debe reportar? | Operación | Abierta |
| `Q-012` | ¿Los valores nulos se muestran vacíos, con guion o valor por defecto? | UX/contrato | Abierta |

Las respuestas que cambien alcance requieren una nueva feature; no se incorporan silenciosamente al baseline.
