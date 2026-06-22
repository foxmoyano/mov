# 001 - Catálogo de videos

| Campo | Valor |
|---|---|
| Estado | En revisión, baseline derivado del código |
| Versión | 0.2.0 |
| Fecha | 2026-06-21 |
| Responsable | Pendiente de validación de producto |

## Alcance

Esta especificación formaliza el comportamiento construido de lectura del catálogo y detalle. No convierte funciones parciales o declaradas únicamente en un requisito aprobado.

## US-001 Acceder a la aplicación

Como usuario, quiero ingresar a una ruta estable y ver la interfaz localizada para comenzar a consultar el catálogo.

### Requisitos

- `FR-001`: la ruta raíz debe presentar la página Gestión de Videos.
- `FR-002`: una ruta frontend desconocida debe redirigir a la raíz.
- `FR-003`: la aplicación debe usar locale `es-CL` para pipes y formatos localizables.
- `FR-004`: el encabezado debe mostrar el total conocido y distinguir entre estado cargando y sistema activo.

### Escenarios de aceptación documentales

- `AC-001`: dada la ruta raíz, cuando se abre la aplicación, entonces se presenta Gestión de Videos.
- `AC-002`: dada una ruta desconocida, cuando el router la resuelve, entonces redirige a la raíz.
- `AC-003`: dados valores de fecha y número, cuando se presentan, entonces usan los pipes configurados con locale `es-CL`.

## US-002 Consultar el catálogo

Como usuario, quiero ver los metadatos principales de los videos para compararlos sin abrir cada elemento.

### Requisitos

- `FR-005`: el frontend debe solicitar el listado mediante `GET /api/v1/videos`.
- `FR-006`: la respuesta debe contener `items`, `total`, `page` y `size`.
- `FR-007`: cada fila debe poder representar ID, título, extensión, tamaño MB, fecha de publicación, duración, resolución, altura e imagen persistida recibidos del backend.
- `FR-008`: la tabla debe presentar título, tipo, tamaño, duración, calidad, resolución y fecha de publicación.
- `FR-009`: la extensión se presenta sin punto y en mayúsculas; tamaño en MB; duración en segundos; fecha como `dd-MM-yyyy`.
- `FR-010`: durante una solicitud, la tabla y el encabezado deben reflejar estado de carga.
- `FR-011`: una respuesta sin elementos debe conservar total y paginación coherentes, sin inventar filas.

### Escenarios de aceptación documentales

- `AC-004`: dado un listado con datos completos, cuando se representa, entonces cada columna muestra el atributo correspondiente.
- `AC-005`: dada una solicitud pendiente, cuando la vista espera respuesta, entonces muestra estado de carga.
- `AC-006`: dada una página sin elementos, cuando se representa, entonces no se muestran filas y el total coincide con la respuesta.

## US-003 Navegar el catálogo

Como usuario, quiero cambiar de página y tamaño para recorrer el catálogo de manera controlada.

### Requisitos

- `FR-012`: la tabla debe trabajar con carga lazy y emitir página/tamaño en cada cambio.
- `FR-013`: el frontend debe calcular `page = floor(first / rows)` y conservar `first`, `rows` y `page` en el store.
- `FR-014`: la interfaz debe ofrecer tamaños de 10, 20 y 50 filas.
- `FR-015`: el backend debe interpretar páginas con índice base cero y convertir páginas negativas a cero.
- `FR-016`: el tamaño por defecto debe ser 10; valores no positivos usan 10 y valores mayores a 100 usan 100.
- `FR-017`: la respuesta debe devolver los valores efectivos de `page` y `size`.
- `FR-018`: conteo y consulta de elementos deben aplicar los mismos filtros.
- `FR-019`: el backend debe ordenar el listado por título ascendente antes de aplicar límite y offset.

### Escenarios de aceptación documentales

- `AC-007`: dado `first=20` y `rows=10`, cuando cambia el paginador, entonces el frontend solicita `page=2` y `size=10`.
- `AC-008`: dada una página negativa, cuando el backend la normaliza, entonces responde con página cero.
- `AC-009`: dado un tamaño mayor a 100, cuando se consulta, entonces el tamaño efectivo es 100.
- `AC-010`: dado un tamaño nulo o no positivo, cuando se consulta, entonces el tamaño efectivo es 10.

## US-004 Filtrar por título

Como usuario, quiero buscar por una parte del título para reducir el catálogo.

### Requisitos

- `FR-020`: la interfaz debe permitir ingresar título y año, y emitir ambos al seleccionar Buscar.
- `FR-021`: el store debe reiniciar `page` y `first` a cero cuando cambian los filtros.
- `FR-022`: el frontend debe enviar valores no nulos como query params junto con `page` y `size`.
- `FR-023`: el backend debe soportar únicamente `title` como filtro construido actualmente.
- `FR-024`: el backend debe recortar espacios y buscar por contenido con `ILIKE`; texto vacío equivale a no filtrar.
- `FR-025`: Limpiar debe vaciar título, establecer año en `null` y solicitar nuevamente desde la primera página.

### Escenarios de aceptación documentales

- `AC-011`: dado el texto `star`, cuando se busca, entonces se consideran títulos que lo contienen sin distinguir caso.
- `AC-012`: dado un título con espacios exteriores, cuando se filtra, entonces los espacios se eliminan antes de formar la coincidencia.
- `AC-013`: dado un filtro limpio, cuando se aplica, entonces se solicita la primera página sin filtro de título.
- `AC-014`: dado un año en la UI actual, cuando se envía, entonces no modifica resultados porque no forma parte del DTO backend; esta divergencia debe permanecer visible como brecha.

## US-005 Consultar detalle por API

Como cliente de API, quiero consultar un video por UUID para obtener metadatos e imágenes asociadas.

### Requisitos

- `FR-026`: el backend debe exponer `GET /api/v1/videos/{id}` con `id` UUID.
- `FR-027`: el detalle debe incluir `id`, `title`, `extension`, `size_mb`, `published_at`, `duration_seconds`, `resolution`, `video_height`, `image_url`, `main_image_url` y `scene_images`.
- `FR-028`: un UUID inexistente debe devolver `404`.
- `FR-029`: un path que no pueda extraerse como UUID debe ser rechazado antes de consultar datos.
- `FR-030`: la imagen principal debe buscarse en `media/{uuid}/main.jpg`.
- `FR-031`: las escenas deben listarse desde `media/{uuid}/images/`.
- `FR-032`: las URLs de imagen deben ser temporales con expiración actual de 3600 segundos.
- `FR-033`: la ausencia o fallo no crítico de imagen principal debe producir `main_image_url: null` sin perder el detalle.
- `FR-034`: la ausencia o fallo no crítico de escenas debe producir una lista vacía sin perder el detalle.
- `FR-035`: extensión nula se transforma en cadena vacía y tamaño nulo en `0` para el DTO de detalle.
- `FR-036`: `video_height` debe convertirse a entero; un valor no convertible produce `null`.

### Escenarios de aceptación documentales

- `AC-015`: dado un UUID existente con imágenes, cuando se consulta, entonces se devuelven metadatos y URLs temporales.
- `AC-016`: dado un UUID existente sin imagen principal, cuando se consulta, entonces el detalle contiene `main_image_url: null`.
- `AC-017`: dado un UUID existente sin escenas, cuando se consulta, entonces `scene_images` es una lista vacía.
- `AC-018`: dado un UUID inexistente, cuando se consulta, entonces se devuelve `404`.
- `AC-019`: dado un identificador con formato inválido, cuando se consulta, entonces la solicitud se rechaza sin consulta de negocio.

## US-006 Gestionar estados y errores

Como usuario u operador, quiero errores acotados y estado consistente para comprender fallos sin exponer detalles internos.

### Requisitos

- `FR-037`: el interceptor frontend debe transformar errores HTTP a `{message, status, code}`.
- `FR-038`: el código por defecto frontend debe ser `UNKNOWN_ERROR` y el mensaje fallback debe ser `Ocurrió un error inesperado`.
- `FR-039`: en modo desarrollo, el frontend debe registrar método, URL y payload normalizado.
- `FR-040`: el store debe desactivar loading tanto en error como al completar una solicitud.
- `FR-041`: un error de listado o detalle en PostgreSQL debe registrarse técnicamente y responder como error interno acotado.
- `FR-042`: los fallos de MinIO ya degradados no deben convertirse en error HTTP si los metadatos existen.

### Escenarios de aceptación documentales

- `AC-020`: dado un error HTTP sin código de aplicación, cuando se intercepta, entonces usa `UNKNOWN_ERROR`.
- `AC-021`: dado un error de listado, cuando finaliza el observable, entonces loading vuelve a `false`.
- `AC-022`: dado un fallo de MinIO con video existente, cuando se forma el detalle, entonces se responde con imágenes ausentes y no con `500`.

## US-007 Operar el backend

Como operador, quiero iniciar y detener la API con configuración externa para conectarla a sus dependencias.

### Requisitos

- `FR-043`: `DATABASE_URL`, `MINIO_ENDPOINT`, `MINIO_ACCESS_KEY`, `MINIO_SECRET_KEY` y `MINIO_BUCKET` son configuración obligatoria de inicio.
- `FR-044`: `HOST` debe usar `0.0.0.0` y `PORT` 3000 cuando no se proporcionan.
- `FR-045`: `MINIO_REGION` debe usar `us-east-1` cuando no se proporciona.
- `FR-046`: el pool PostgreSQL debe configurarse actualmente con máximo de cinco conexiones.
- `FR-047`: el backend debe atender señales de terminación y cierre de consola para detener Axum ordenadamente.
- `FR-048`: el filtro de logs debe aceptar `RUST_LOG` y usar un fallback informativo si no está definido.

### Escenarios de aceptación documentales

- `AC-023`: dada una configuración obligatoria ausente, cuando inicia el backend, entonces falla antes de servir solicitudes.
- `AC-024`: dados HOST y PORT ausentes, cuando se carga configuración, entonces usa `0.0.0.0:3000`.
- `AC-025`: dada una señal de terminación soportada, cuando se recibe, entonces el servidor inicia cierre ordenado.

## Requisitos no funcionales

- `NFR-001 Seguridad`: no exponer credenciales, stack traces ni errores de proveedores en respuestas públicas.
- `NFR-002 Observabilidad`: registrar inicio, consultas, UUID, conteos y fallos con `tracing`, sin secretos.
- `NFR-003 Compatibilidad`: conservar Angular 21, TypeScript 5.9, Rust 1.91, Axum 0.7 y SQLx 0.8.
- `NFR-004 Accesibilidad`: tabla y filtros deben conservar labels, `scope`, foco y semántica comprensible.
- `NFR-005 Integridad`: conteo, filtros, página y datos deben ser coherentes.
- `NFR-006 Configuración`: endpoints y credenciales de infraestructura deben resolverse por entorno y fuera del código desplegable.
- `NFR-007 Resiliencia`: fallos de imágenes no deben ocultar metadatos disponibles.
- `NFR-008 Localización`: contenido numérico y fechas del frontend deben usar `es-CL`.
- `NFR-009 Privacidad`: los logs no deben registrar credenciales ni URLs firmadas completas.

## Brechas observadas

- `GAP-001`: `Video` en TypeScript omite campos utilizados por la tabla y no representa nulabilidad backend.
- `GAP-002`: `VideosApi` declara POST, PUT y DELETE sin rutas backend equivalentes.
- `GAP-003`: el store declara alta y eliminación, pero no existe flujo integral soportado.
- `GAP-004`: el filtro `year` se envía, pero el backend no lo modela ni filtra.
- `GAP-005`: la tabla muestra ordenamiento por título, pero no envía sort y el backend usa orden fijo.
- `GAP-006`: Nuevo Video y Exportar no tienen handler ni contrato.
- `GAP-007`: el detalle está disponible en API, pero no existe ruta o vista frontend.
- `GAP-008`: el interceptor normaliza errores, pero no muestra notificación al usuario.
- `GAP-009`: CORS permite cualquier origen y métodos sin rutas implementadas.
- `GAP-010`: la URL frontend de API está codificada en environment de desarrollo.
- `GAP-011`: backend devuelve errores como texto y frontend espera opcionalmente `{message, code}`.
- `GAP-012`: no existe healthcheck para despliegue y operación.
- `GAP-013`: `provideHttpClient` se registra dos veces en la configuración frontend.
- `GAP-014`: la tabla invoca operaciones de string sobre extensión aunque backend permite `null`.

Las brechas describen divergencias; no autorizan implementarlas dentro de esta feature baseline.
