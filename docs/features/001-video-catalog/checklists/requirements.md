# Checklist de requisitos de 001-video-catalog

| Campo | Valor |
|---|---|
| Estado | En revisión |
| Fecha | 2026-06-21 |

## Cobertura del baseline

- [x] Ruta principal y fallback documentados.
- [x] Localización y formatos visibles documentados.
- [x] Campos de tabla y estados de carga/vacío documentados.
- [x] Paginación frontend y normalización backend documentadas.
- [x] Filtro por título, espacios y limpieza documentados.
- [x] Divergencia del filtro por año documentada.
- [x] Contrato de detalle y conversiones nulas documentados.
- [x] Rutas MinIO, expiración y degradación parcial documentadas.
- [x] Normalización de errores frontend documentada.
- [x] Configuración, pool, logs y cierre backend documentados.
- [x] Capacidades declaradas pero no soportadas separadas como brechas.

## Calidad SDD

- [x] Historias, requisitos y escenarios tienen IDs únicos.
- [x] Los requisitos describen comportamiento observable.
- [x] Las decisiones técnicas reflejan el baseline existente.
- [x] Las brechas no se presentan como capacidades aprobadas.
- [x] Contratos y datos tienen artefactos derivados.
- [x] Casos documentales cubren todos los `AC-*`.
- [ ] Producto valida que el baseline representa la intención deseada.

## Preguntas que bloquean ampliaciones

- [ ] Roles y autenticación (`Q-001`).
- [ ] CORS por entorno (`Q-003`).
- [ ] Año y ordenamiento (`Q-004`, `Q-005`).
- [ ] Acciones de escritura/exportación (`Q-006`).
- [ ] Vista de detalle (`Q-009`).
- [ ] Contrato de errores y nulos (`Q-010`, `Q-012`).
- [ ] SLO y healthcheck (`Q-007`, `Q-011`).

El baseline puede permanecer documentado. Ninguna ampliación afectada por estas preguntas avanza a implementación.
