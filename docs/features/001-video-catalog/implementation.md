# Implementación de 001-video-catalog

| Campo | Valor |
|---|---|
| Estado | Implementado como baseline observado |
| Versión | 1.1.0 |
| Fecha | 2026-06-21 |
| Responsable | Equipo técnico |

## Capacidades observadas

| Capacidad | Frontend | Backend | Estado integral |
|---|---|---|---|
| Ruta raíz y fallback | Implementado | No aplica | Implementado |
| Localización `es-CL` | Implementado | No aplica | Implementado |
| Listado paginado | Implementado | Implementado | Implementado |
| Filtro por título | Implementado | Implementado | Implementado |
| Filtro por año | Implementado en UI/request | No implementado | Incompleto |
| Orden por título | Control visual | Orden fijo ascendente | No contratado |
| Detalle por UUID | Cliente API sin vista | Implementado | Parcial |
| Imagen principal y escenas | No visible | Implementado con MinIO | Parcial |
| Normalización de errores | Implementado sin notificación | Texto plano/logs | Parcial |
| Configuración externa | API URL de desarrollo | PostgreSQL/MinIO por env | Parcial |
| Cierre ordenado | No aplica | Implementado | Implementado |
| Crear video | Cliente declarado/botón visual | No implementado | No soportado |
| Actualizar video | Cliente declarado | No implementado | No soportado |
| Eliminar video | Cliente/store declarado | No implementado | No soportado |
| Exportar | Botón visual | No implementado | No soportado |

## Regla para futuras entradas

Cada implementación agrega:

- feature y versión de spec;
- tareas completadas;
- archivos modificados;
- decisiones o desviaciones;
- contratos y datos afectados;
- verificaciones documentales realizadas;
- riesgos y rollback.

No registrar pruebas o compilaciones porque están prohibidas. Si el código contradice la spec, marcar `Desviación` y no declarar la feature completada.
