# Constitución SDD de MOV

| Campo | Valor |
|---|---|
| Estado | Aprobado |
| Versión | 1.0.0 |
| Fecha | 2026-06-21 |
| Responsable | Equipo MOV |

## I. La especificación gobierna

Todo cambio funcional comienza en una especificación versionada. El código no introduce comportamiento, endpoint, campo o dependencia que no pueda rastrearse a un requisito aprobado.

## II. Trazabilidad obligatoria

Los identificadores siguen estos prefijos: `OBJ` objetivo, `US` historia, `FR` requisito funcional, `NFR` requisito no funcional, `AC` criterio de aceptación, `DEC` decisión, `CTR` contrato, `TASK` tarea y `RISK` riesgo. Cada tarea referencia al menos un requisito y cada requisito implementado referencia evidencia.

## III. Contrato antes que integración

Los cambios frontend/backend definen método, URL, parámetros, body, respuesta, errores y validaciones antes de modificar código. Un método declarado únicamente en el cliente no constituye un endpoint soportado.

## IV. Consistencia con el sistema

El frontend conserva Angular 21 standalone, PrimeNG, signals y separación API/store. El backend conserva Rust, Axum, SQLx y separación routes/handlers/services/DTO/models/storage. Una excepción requiere una decisión explícita en `research.md`.

## V. Datos e infraestructura verificables

Los documentos distinguen hechos observados, decisiones aprobadas y supuestos. La estructura PostgreSQL se consulta exclusivamente mediante `postgres-mov`. Los secretos nunca se documentan.

## VI. Simplicidad y alcance

La implementación resuelve solo el alcance aprobado. No se agregan dependencias, modernizaciones, abstracciones preventivas ni refactors incidentales sin un requisito propio.

## VII. Restricción de pruebas y compilación

Está prohibido crear, modificar, eliminar o ejecutar pruebas de cualquier tipo. También está prohibido compilar, construir, ejecutar o servir los proyectos para validar cambios. Los escenarios de aceptación, checklists y evidencias SDD son documentación, no tests ejecutables.

## VIII. Seguridad y privacidad

Ningún artefacto contiene credenciales, tokens, claves, cadenas de conexión completas o URLs firmadas. Los errores públicos no exponen detalles de PostgreSQL, MinIO, AWS SDK o stack traces.

## IX. Despliegue reversible

Todo release documenta configuración, compatibilidad, orden de despliegue, observabilidad, riesgos y rollback. La ausencia de infraestructura conocida se registra como brecha y no se completa con supuestos.

## X. Retroalimentación continua

Métricas, incidentes y aprendizaje operacional se traducen en nuevos requisitos o revisiones. Nunca se corrige producción sin actualizar la especificación que gobierna el comportamiento afectado.

## Enmiendas

Una enmienda debe indicar motivo, impacto sobre artefactos existentes, versión nueva y aprobación. Cambios incompatibles en principios incrementan versión mayor; nuevos principios, versión menor; aclaraciones, versión patch.
