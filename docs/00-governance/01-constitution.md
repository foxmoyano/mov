# Constitución SDD de MOV

1. La especificación gobierna la implementación.
2. Todo requisito debe ser trazable a contrato, tarea y evidencia.
3. El contrato frontend/backend se define antes de modificar código.
4. Se conserva Angular 21 y Rust/Axum/SQLx.
5. No se agregan dependencias ni refactors fuera de alcance.
6. Testing se documenta, pero no se crea ni ejecuta.
7. No se compila para validar cambios.
8. Todo despliegue debe definir configuración, observabilidad y rollback.
9. Incidentes y métricas retroalimentan la especificación.
