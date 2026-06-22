# Operación y retroalimentación de 001-video-catalog

| Campo | Valor |
|---|---|
| Estado | Borrador |
| Versión | 0.1.0 |
| Fecha | 2026-06-21 |
| Responsable | Pendiente de asignación |

## Señales operacionales

- Disponibilidad y latencia por endpoint.
- Códigos HTTP por ruta.
- Saturación y errores del pool PostgreSQL.
- Errores, ausencias y latencia de MinIO.
- Cantidad de resultados y páginas solicitadas.
- Uso de filtros sin registrar valores sensibles.
- Reinicios y cierres ordenados del backend.

## Alertas por definir

- Tasa de errores `5xx`.
- Latencia p95.
- Falta de conexión PostgreSQL.
- Fallos sostenidos de MinIO.
- Ausencia de capacidad o disponibilidad del servicio.

Los umbrales requieren SLOs aprobados en `Q-007`.

## Incidentes

Cada incidente documenta:

```text
ID y fecha:
Impacto:
Versiones afectadas:
Síntoma y señales:
Contención:
Causa:
Requisito o supuesto incorrecto:
Cambio requerido en spec:
Acciones y responsables:
```

## Cierre del ciclo SDD

Un hallazgo operacional no termina en un hotfix aislado. Debe originar o modificar un requisito, actualizar aclaraciones y plan, generar tareas trazables y pasar nuevamente por análisis, implementación, release y despliegue.
