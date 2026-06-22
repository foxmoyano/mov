# Wireframes documentales de 001-video-catalog

## Catálogo actual

```text
+-------------------------------------------------------------+
| Gestión de Videos                     [Nuevo] [Exportar]     |
+-------------------------------------------------------------+
| Filtros                                                     |
| Título [________________] Año [____] [Buscar] [Limpiar]      |
+-------------------------------------------------------------+
| Título | Tipo | Tamaño | Duración | Calidad | Resol. | Fecha |
| ...                                                         |
+-------------------------------------------------------------+
| Mostrando X a Y de Z                         < 1 2 3 >       |
+-------------------------------------------------------------+
```

## Estados requeridos

- Carga: indicador de tabla activo y texto `Cargando...`.
- Con datos: filas y total coherentes.
- Vacío: tabla sin filas y total cero; falta mensaje específico.
- Error: interceptor global; falta definición UX completa.

## Brechas visuales

- Año, Nuevo y Exportar se muestran sin soporte contractual completo.
- El control de orden no define interacción con backend.
- Falta wireframe aprobado para detalle.
