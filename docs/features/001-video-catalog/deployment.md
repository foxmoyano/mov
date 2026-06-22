# Despliegue de 001-video-catalog

| Campo | Valor |
|---|---|
| Estado | Borrador bloqueado |
| Versión | 0.1.0 |
| Fecha | 2026-06-21 |
| Responsable | Pendiente de asignación |

## Topología esperada

```text
Navegador -> Hosting frontend -> API backend -> PostgreSQL movie_db
                                          \-> MinIO/S3
```

La plataforma concreta, DNS, TLS, red, réplicas y hosting frontend están pendientes (`Q-002`, `Q-008`).

## Configuración backend

Variables conocidas, sin valores en documentación:

- `HOST`
- `PORT`
- `DATABASE_URL`
- `MINIO_ENDPOINT`
- `MINIO_ACCESS_KEY`
- `MINIO_SECRET_KEY`
- `MINIO_REGION`
- `MINIO_BUCKET`
- `RUST_LOG`, si se habilita configuración de tracing

## Configuración frontend

- URL base de API por entorno.
- Política de publicación de assets estáticos.
- Fallback de rutas hacia `index.html`.
- TLS y headers de seguridad.

Actualmente la URL de API está incorporada en `environment.ts`; producción requiere una decisión de configuración antes de desplegar.

## Orden operativo

1. Confirmar release aprobado y artefactos externos disponibles.
2. Confirmar PostgreSQL y schema compatibles.
3. Confirmar bucket, permisos y conectividad MinIO.
4. Aplicar secretos mediante el mecanismo de la plataforma.
5. Desplegar backend sin exponer credenciales.
6. Publicar frontend apuntando al endpoint aprobado.
7. Observar logs y métricas definidos por operación.
8. Registrar versión y resultado, o activar rollback.

## Rollback

- Restaurar versión previa de frontend y backend.
- Revertir configuración a la versión anterior.
- Las migraciones requieren procedimiento propio de reversión o compatibilidad hacia atrás.
- No eliminar objetos MinIO ni datos PostgreSQL durante rollback sin autorización explícita.

## Restricción de ejecución

Este documento describe responsabilidades operativas. El agente no compila, construye imágenes, inicia servicios, ejecuta pruebas ni realiza el despliegue como validación de cambios.
