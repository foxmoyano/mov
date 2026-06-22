# AGENTS.md - Backend MOV

Este proyecto es una API REST escrita en Rust. Todos los comandos de este documento se ejecutan desde `backend`.

## Stack confirmado

- Rust edition 2021, con MSRV `1.91` declarado en `Cargo.toml`.
- Toolchain estable con `rustfmt` y `clippy` en `rust-toolchain.toml`.
- Axum 0.7 y Tokio 1 para HTTP y ejecución asíncrona.
- Serde/serde_json para serialización.
- SQLx 0.8 con PostgreSQL.
- tracing y tower-http para observabilidad y middleware.
- AWS SDK S3 para almacenamiento compatible con S3/MinIO.
- dotenv para configuración local.

Las versiones efectivas siempre se verifican en `Cargo.toml` y `Cargo.lock` antes de implementar.

## Reglas críticas

- **PROHIBIDO crear, modificar, eliminar o generar archivos o módulos de pruebas unitarias, de integración, end-to-end, de contrato, snapshots o cualquier otro tipo de prueba.**
- **PROHIBIDO ejecutar pruebas bajo cualquier circunstancia.** No ejecutar `cargo test`, ni siquiera con filtros, paquetes, targets o flags que limiten su alcance.
- No agregar bloques `#[cfg(test)]`, funciones `#[test]`, pruebas async ni fixtures como parte de una funcionalidad, corrección o refactor; tampoco sugerirlos como trabajo pendiente.
- **PROHIBIDO compilar o construir el backend para validar los cambios realizados.** No ejecutar `cargo build`, `cargo check`, `cargo clippy`, `cargo run` ni comandos equivalentes que compilen o inicien la API.
- No usar compilación, chequeo del compilador, Clippy ni ejecución del servidor como mecanismo de verificación, aunque el cambio parezca requerirlo.
- No leer ni modificar `target`.
- No agregar ni actualizar crates sin autorización explícita.
- No editar `Cargo.lock` manualmente.
- No bloquear el runtime de Tokio con I/O síncrono o trabajo intensivo dentro de handlers async.
- No usar `unwrap()` o `expect()` con datos de request, respuestas externas o resultados de base de datos.
- No exponer errores internos, stack traces, credenciales ni detalles de infraestructura en respuestas HTTP.
- No registrar secretos, tokens, URLs firmadas completas ni contenido sensible.
- No modificar `.env` ni incorporar sus valores a código o documentación.

## Análisis previo obligatorio

Antes de modificar el backend, revisar según el alcance:

- `Cargo.toml`, `Cargo.lock` y `rust-toolchain.toml`;
- `src/main.rs`, `src/config.rs` y `src/routes/mod.rs`;
- routes, handlers, services, DTOs y models similares;
- inicialización y uso del pool PostgreSQL en `src/db`;
- integración S3/MinIO en `src/storage` y services relacionados;
- estrategia de errores, códigos HTTP y logging del área;
- configuración de Docker y variables de entorno cuando corresponda;

## Arquitectura y responsabilidades

Mantener la separación actual:

- `routes`: composición del `Router`, paths y métodos HTTP;
- `handlers`: extractores de Axum, validación de entrada, coordinación y respuesta HTTP;
- `services`: lógica de negocio e integraciones;
- `dto`: contratos de entrada y salida;
- `models`: representación de datos persistidos;
- `db`: configuración y acceso compartido a PostgreSQL;
- `storage`: operaciones de bajo nivel con S3/MinIO;
- `config`: lectura y validación de configuración;
- `AppState`: dependencias compartidas por los handlers.

No mover lógica de negocio a routes ni duplicar acceso a infraestructura en handlers cuando exista un service apropiado.

## Endpoints y errores

- Definir rutas con extractores tipados de Axum (`Path`, `Query`, `State`, `Json`).
- Validar límites, formatos y valores de entrada antes de ejecutar operaciones costosas.
- Devolver el código HTTP correcto: `400` para entrada inválida, `404` para recurso ausente, `409` para conflictos y `500` para fallos internos inesperados.
- Mantener estable la forma de los DTOs públicos; revisar el impacto en frontend antes de cambiarla.
- No devolver mensajes de SQLx, AWS SDK u otros proveedores al cliente.
- Registrar el error técnico con contexto mediante `tracing` y devolver un mensaje público acotado.
- Si aumenta la complejidad del manejo de errores, seguir el patrón local o proponer una abstracción antes de incorporarla.

## Base de datos y SQLx

- Usar parámetros enlazados con `bind`, `query_as` o `QueryBuilder`; nunca interpolar entrada del usuario en SQL.
- Mantener consultas, filtros, orden y paginación consistentes entre `COUNT` y obtención de datos.
- Definir límites máximos razonables para paginación.
- Mantener models y DTOs separados cuando el contrato HTTP difiera de la fila persistida.
- Revisar nulabilidad y tipos PostgreSQL/Rust antes de alterar structs.
- No ejecutar cambios destructivos, migraciones ni escrituras masivas sin autorización explícita.
- Si se incorpora un sistema de migraciones, documentar primero su ubicación, orden y ejecución; actualmente no debe asumirse Liquibase ni otro mecanismo heredado.

## Asincronía y servicios externos

- Propagar errores con `Result` y `?`; convertirlos en la frontera adecuada.
- Evitar llamadas secuenciales cuando sean independientes y la concurrencia sea segura y acotada.
- Aplicar timeouts, límites y manejo de fallos parciales cuando el patrón del proyecto lo permita.
- Para S3/MinIO, centralizar operaciones de bajo nivel en `storage` y reglas de dominio en services.
- Generar URLs firmadas solo con expiración acotada y no persistirlas como datos permanentes.
- Distinguir recurso inexistente de error de infraestructura.

## Configuración y observabilidad

- Leer configuración desde variables de entorno mediante `config` o el patrón existente.
- Fallar al iniciar si falta configuración obligatoria; no usar valores inseguros por defecto.
- Usar logging estructurado con campos (`id`, `uuid`, conteos) en lugar de concatenar datos sensibles.
- Mantener logs útiles y evitar ruido por elemento dentro de colecciones grandes.
- Conservar el apagado ordenado y el estado compartido configurados en `main.rs`.

## Estilo y calidad

- Mantener el código compatible con Rust 1.91 y edition 2021.
- Seguir `rustfmt` y mantener el estilo observado en el código cercano.
- Preferir tipos explícitos de dominio y ownership claro frente a clones innecesarios.
- Agregar comentarios solo para decisiones no evidentes; usar documentación `///` en APIs públicas que lo necesiten.
- No mezclar refactors generales con cambios funcionales.

## Verificación permitida

El alias de formato del proyecto está definido en `.cargo/config.toml`. Es la única verificación por comando permitida porque no compila el proyecto.

```bash
cargo fmtc
```

- Revisar los archivos modificados y su diff.
- Comprobar manualmente tipos, imports, ownership, lifetimes, rutas, SQL y consistencia con módulos similares.
- No ejecutar pruebas, compilación, `cargo check`, Clippy, la API, migraciones ni verificaciones contra servicios externos.
- Informar que pruebas y compilación no se ejecutaron por prohibición expresa del proyecto.
