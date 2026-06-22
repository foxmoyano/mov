# Casos de prueba documentales de 001-video-catalog

| ID | Requisitos/aceptación | Caso | Resultado esperado | Estado |
|---|---|---|---|---|
| `TC-001` | `FR-001`, `FR-004`, `AC-001` | Acceder a raíz | Se presenta Gestión de Videos con total/estado | No ejecutado |
| `TC-002` | `FR-002`, `AC-002` | Acceder a ruta desconocida | Redirección a raíz | No ejecutado |
| `TC-003` | `FR-003`, `AC-003` | Presentar fecha y número | Formatos configurados con `es-CL` | No ejecutado |
| `TC-004` | `FR-005`, `FR-006`, `FR-007`, `FR-008`, `FR-009`, `AC-004` | Listado con datos completos | Columnas y formatos corresponden al contrato | No ejecutado |
| `TC-005` | `FR-010`, `AC-005` | Solicitud pendiente | Tabla y encabezado indican carga | No ejecutado |
| `TC-006` | `FR-011`, `AC-006` | Página sin elementos | Sin filas; total y página coherentes | No ejecutado |
| `TC-007` | `FR-012`, `FR-013`, `FR-014`, `AC-007` | `first=20`, `rows=10` | Se solicita página 2 y tamaño 10 | No ejecutado |
| `TC-008` | `FR-015`, `AC-008` | Página negativa | Página efectiva cero | No ejecutado |
| `TC-009` | `FR-016`, `AC-009` | Tamaño mayor a 100 | Tamaño efectivo 100 | No ejecutado |
| `TC-010` | `FR-016`, `FR-017`, `AC-010` | Tamaño nulo o no positivo | Tamaño efectivo 10 informado en respuesta | No ejecutado |
| `TC-011` | `FR-020`, `FR-021`, `FR-022`, `FR-023`, `FR-024`, `AC-011` | Filtrar título con distinto caso | Coincidencias parciales sin distinguir caso | No ejecutado |
| `TC-012` | `FR-024`, `AC-012` | Título con espacios exteriores | Filtro usa el texto recortado | No ejecutado |
| `TC-013` | `FR-021`, `FR-025`, `AC-013` | Limpiar filtros | Valores limpios y primera página | No ejecutado |
| `TC-014` | `FR-020`, `FR-023`, `AC-014` | Enviar año | Resultados no cambian; brecha visible | No ejecutado |
| `TC-015` | `FR-026`, `FR-027`, `FR-030`, `FR-031`, `FR-032`, `AC-015` | Detalle existente con imágenes | Metadatos y URLs temporales | No ejecutado |
| `TC-016` | `FR-033`, `AC-016` | Imagen principal ausente | `main_image_url: null` | No ejecutado |
| `TC-017` | `FR-034`, `AC-017` | Escenas ausentes | `scene_images: []` | No ejecutado |
| `TC-018` | `FR-028`, `AC-018` | UUID inexistente | `404` | No ejecutado |
| `TC-019` | `FR-029`, `AC-019` | Identificador inválido | Rechazo antes de lógica de negocio | No ejecutado |
| `TC-020` | `FR-035`, `FR-036` | Campos persistidos nulos/no convertibles | Defaults y `null` según DTO | No ejecutado |
| `TC-021` | `FR-037`, `FR-038`, `AC-020` | Error sin código de aplicación | Payload usa `UNKNOWN_ERROR` | No ejecutado |
| `TC-022` | `FR-040`, `AC-021` | Error de listado | Loading vuelve a `false` | No ejecutado |
| `TC-023` | `FR-042`, `AC-022` | Fallo MinIO con video existente | Detalle sin imágenes, no error HTTP | No ejecutado |
| `TC-024` | `FR-043`, `AC-023` | Falta configuración obligatoria | Backend no comienza a servir | No ejecutado |
| `TC-025` | `FR-044`, `AC-024` | HOST/PORT ausentes | Defaults `0.0.0.0:3000` | No ejecutado |
| `TC-026` | `FR-047`, `AC-025` | Señal de terminación | Cierre ordenado iniciado | No ejecutado |
| `TC-027` | `FR-018`, `FR-019` | Listar con filtro y varias páginas | Conteo/datos usan el mismo filtro y orden por título | No ejecutado |
| `TC-028` | `FR-039` | Error HTTP en modo desarrollo | Se registra método, URL y payload normalizado | No ejecutado |
| `TC-029` | `FR-041` | Fallo PostgreSQL | Log técnico y respuesta interna acotada | No ejecutado |
| `TC-030` | `FR-045` | Región MinIO ausente | Se usa `us-east-1` | No ejecutado |
| `TC-031` | `FR-046` | Inicializar pool | Máximo configurado de cinco conexiones | No ejecutado |
| `TC-032` | `FR-048` | `RUST_LOG` ausente/presente | Se usa valor recibido o fallback configurado | No ejecutado |

Estos casos especifican verificación esperada; no son tests implementados ni autorizan su ejecución.
