# AGENTS.md - Workspace MOV

Este workspace contiene una aplicación web formada por un frontend Angular y una API REST en Rust.

## Rol del agente

El agente debe actuar como coordinador técnico entre frontend, backend, base de datos, almacenamiento de objetos, verificación estática y documentación. Antes de modificar archivos debe identificar los proyectos afectados, leer sus instrucciones específicas y revisar implementaciones similares.

## Proyectos

| Alias | Proyecto | Tecnología | Ubicación |
|---|---|---|---|
| `@front` | Frontend web | Angular 21 | `frontend/platform` |
| `@backend` | API REST | Rust, Axum y SQLx | `backend` |

`frontend` es el contenedor del proyecto Angular; los comandos del frontend se ejecutan desde `frontend/platform`.

Si el usuario emplea un alias desconocido, se debe preguntar a qué proyecto se refiere antes de actuar.

## Regla principal

No asumir que frontend y backend comparten versiones, herramientas o convenciones. Para cualquier tarea:

1. Leer este archivo.
2. Resolver los proyectos afectados.
3. Leer el `AGENTS.md` de cada proyecto afectado.
4. Revisar manifiestos, configuración e implementaciones similares.
5. Informar el alcance y el plan antes de modificar archivos.

## Flujo para tareas frontend + backend

Cuando una tarea involucre ambos proyectos:

1. Revisar componentes, rutas, modelos, APIs y stores similares del frontend.
2. Revisar rutas, handlers, services, DTOs, models y consultas similares del backend.
3. Definir el contrato HTTP antes de implementar.
4. Separar el plan por capas: frontend, backend, datos/almacenamiento, verificación permitida y documentación.
5. Evaluar compatibilidad e impacto sobre consumidores existentes.
6. No modificar archivos hasta comunicar la salida previa obligatoria.

## Contrato frontend/backend

Antes de crear o cambiar una integración se debe definir:

- URL y método HTTP;
- path params y query params;
- headers relevantes;
- request body, si corresponde;
- DTO de respuesta y nombres/tipos de campos;
- paginación, orden y filtros, cuando apliquen;
- códigos de estado y formato de error;
- validaciones del frontend y del backend;
- tratamiento de errores, carga y ausencia de datos en el frontend.

No cambiar un contrato existente sin revisar todos sus consumidores.

## Restricciones generales

- **PROHIBIDO crear, modificar, eliminar o generar archivos de pruebas de cualquier tipo.**
- **PROHIBIDO ejecutar pruebas unitarias, de integración, end-to-end, de componentes, de contrato, snapshots o cualquier comando de testing.**
- La prohibición aplica a frontend y backend, incluso si las pruebas ya existen o si su ejecución parece necesaria para validar un cambio.
- No ejecutar `pnpm test`, `ng test`, `vitest`, `cargo test` ni comandos equivalentes, completos o parciales.
- **PROHIBIDO compilar o construir los proyectos para validar cambios.** No ejecutar `pnpm build`, `ng build`, `cargo build`, `cargo check`, `cargo clippy` ni comandos equivalentes.
- No modificar varios proyectos sin informarlo explícitamente.
- No agregar ni actualizar dependencias sin autorización.
- No modernizar arquitectura ni mezclar refactors ajenos con cambios funcionales.
- No duplicar componentes, stores, services, rutas o endpoints si existe un equivalente reutilizable.
- No modificar migraciones o datos históricos ya aplicados sin autorización.
- No ejecutar comandos destructivos ni operaciones masivas sobre datos.
- No exponer secretos de `.env`, credenciales, tokens, URLs firmadas ni datos sensibles.
- Ignorar artefactos generados como `node_modules`, `dist` y `target` durante búsquedas y revisiones.
- Respetar cambios locales preexistentes y no revertirlos.

## Salida previa obligatoria

Antes de modificar archivos, entregar:

1. proyectos involucrados;
2. instrucciones específicas leídas;
3. patrón detectado en frontend, si aplica;
4. patrón detectado en backend, si aplica;
5. contrato frontend/backend propuesto, si aplica;
6. archivos candidatos a modificar;
7. plan de implementación;
8. riesgos;
9. verificaciones permitidas necesarias, excluyendo expresamente cualquier prueba.

Para tareas exclusivas de un proyecto o de documentación, indicar los puntos no aplicables de forma breve.

## Verificación

- La verificación se limita a revisión de archivos, revisión de diferencias y formato que no compile ni construya el proyecto.
- Nunca usar compilación, build, type checking basado en compilador o Clippy como mecanismo de validación.
- Nunca crear pruebas como parte de una implementación ni sugerir su creación como trabajo pendiente.
- Usar los comandos definidos en el `AGENTS.md` del proyecto afectado.
- Ejecutar primero verificaciones específicas y luego las generales cuando el alcance lo justifique.
- No iniciar servidores salvo que sean necesarios para la tarea.
- No dejar procesos en ejecución al finalizar.
- Si una verificación no puede ejecutarse por red, servicios externos o configuración local, informar la causa y lo que sí se verificó.

## Salida final obligatoria

Al terminar, entregar:

1. resumen funcional;
2. resumen técnico por proyecto;
3. contrato frontend/backend implementado, si aplica;
4. archivos modificados o creados;
5. verificaciones permitidas ejecutadas y su resultado;
6. riesgos o pendientes;
7. mensaje de commit sugerido.
