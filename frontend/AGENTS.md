# AGENTS.md - Frontend MOV

El frontend es la aplicación Angular ubicada en `frontend/platform`. Todos los comandos de este documento se ejecutan desde esa carpeta.

## Stack confirmado

- Angular 21.1 con componentes standalone.
- TypeScript 5.9 en modo estricto y target ES2022.
- RxJS 7.8.
- PrimeNG 21, PrimeFlex 4, PrimeIcons 7 y temas PrimeUIX.
- `pnpm` 10.28.2 como gestor de paquetes.

Las versiones efectivas siempre se verifican en `package.json`, `pnpm-lock.yaml` y `angular.json` antes de usar una API.

## Reglas críticas

- **PROHIBIDO crear, modificar, eliminar o generar archivos de pruebas unitarias, de integración, end-to-end, de componentes, snapshots o cualquier otro tipo de prueba.**
- **PROHIBIDO ejecutar pruebas bajo cualquier circunstancia.** No ejecutar `pnpm test`, `ng test`, `vitest` ni comandos equivalentes, aunque las pruebas ya existan.
- No agregar pruebas como parte de una funcionalidad, corrección o refactor, ni sugerirlas como trabajo pendiente.
- **PROHIBIDO compilar o construir el frontend para validar los cambios realizados.** No ejecutar `pnpm build`, `ng build`, `pnpm watch`, `ng serve` ni comandos equivalentes que compilen o inicien la aplicación.
- No usar compilación de TypeScript, build de Angular ni servidor de desarrollo como mecanismo de verificación, aunque el cambio parezca requerirlo.
- No leer ni modificar `node_modules`.
- No editar manualmente `pnpm-lock.yaml`; solo puede cambiar como consecuencia justificada de una operación de `pnpm` autorizada.
- No agregar ni actualizar dependencias sin autorización explícita.
- No convertir patrones existentes ni hacer refactors ajenos a la tarea.
- No desactivar opciones strict ni usar `any` para eludir errores de tipos.
- No colocar secretos ni endpoints sensibles directamente en el código; usar la configuración de `src/environment` siguiendo el patrón existente.

## Análisis previo obligatorio

Antes de modificar el frontend, revisar según el alcance:

- `package.json`, `angular.json` y `tsconfig.json`;
- `src/main.ts`, `src/app/app.config.ts` y `src/app/app.routes.ts`;
- estructura de `src/app/core` y `src/app/features`;
- componentes, modelos, APIs y stores similares;
- interceptors, manejo de errores y configuración de `HttpClient`;
- estilos globales, estilos del componente y uso de PrimeNG;

## Arquitectura y convenciones

- Mantener componentes standalone; no introducir NgModules salvo una necesidad externa explícita.
- Preferir `inject()` cuando sea consistente con el área modificada.
- Usar signals, `computed` y el patrón de store local ya presente para estado síncrono.
- Mantener acceso HTTP en clases `*.api.ts` y estado/orquestación en `*.store.ts` cuando la feature siga ese patrón.
- Organizar funcionalidad de dominio en `src/app/features/<feature>` y elementos transversales en `src/app/core`.
- Definir rutas en `app.routes.ts` o en rutas de feature si el proyecto las incorpora.
- Reutilizar modelos compartidos de paginación y filtros antes de crear equivalentes.
- Mantener nombres de archivos en kebab-case y nombres TypeScript según las convenciones cercanas.
- Usar imports claros y evitar dependencias circulares.

## Angular 21

- Se permite el control flow moderno (`@if`, `@for`, `@switch`) y debe preferirse si coincide con las vistas existentes.
- Se permiten signals e inyección con `inject()` porque forman parte del patrón actual.
- Mantener templates accesibles: etiquetas, estados de foco, texto alternativo y asociaciones de formularios.
- En listas con `@for`, definir un `track` estable.
- Evitar suscripciones manuales cuando `async`, signals o una composición RxJS resuelvan el ciclo de vida.
- Si una suscripción manual es necesaria, asegurar su liberación mediante mecanismos compatibles con Angular 21.
- No introducir APIs experimentales sin autorización.

## Formularios y validación

- Confirmar primero si la feature usa formularios reactivos o template-driven.
- Mantener validaciones de experiencia de usuario en frontend, sin sustituir la validación autoritativa del backend.
- Mostrar mensajes y estados inválidos siguiendo el patrón visual existente.
- No duplicar validators ni normalización de datos.

## Integración HTTP

- Tipar request, response, paginación y errores; no consumir respuestas como `any`.
- Mantener URLs base en environment y rutas de recursos en las APIs de feature.
- Reutilizar el interceptor global de errores y no ocultar errores silenciosamente.
- Representar explícitamente estados de carga, éxito, vacío y error.
- No cambiar nombres o tipos de DTOs sin coordinar el contrato con backend.

## UI y estilos

- Reutilizar PrimeNG, PrimeFlex, PrimeIcons y los tokens/temas existentes.
- No agregar otro framework visual para resolver componentes ya cubiertos.
- Mantener estilos específicos junto al componente y estilos realmente globales en `src/styles.css`.
- Verificar comportamiento responsive y accesibilidad en cambios visuales.

## Verificación permitida

- Revisar los archivos modificados y su diff.
- Comprobar manualmente tipos, imports, templates, rutas y consistencia con componentes similares.
- Se permite formato que no compile, construya, sirva ni ejecute la aplicación.
- No ejecutar pruebas, compilación, build, type checking basado en el compilador ni servidor de desarrollo.
- Informar que pruebas y compilación no se ejecutaron por prohibición expresa del proyecto.
