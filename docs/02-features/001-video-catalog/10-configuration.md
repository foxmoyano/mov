# Configuración de 001-video-catalog

| Componente | Clave | Obligatoria | Secreto |
|---|---|---:|---:|
| Frontend | API base URL | Sí | No |
| Backend | `HOST` | No | No |
| Backend | `PORT` | No | No |
| Backend | `DATABASE_URL` | Sí | Sí |
| Backend | `MINIO_ENDPOINT` | Sí | No |
| Backend | `MINIO_ACCESS_KEY` | Sí | Sí |
| Backend | `MINIO_SECRET_KEY` | Sí | Sí |
| Backend | `MINIO_REGION` | No | No |
| Backend | `MINIO_BUCKET` | Sí | No |
| Backend | `RUST_LOG` | No | No |

## Reglas

- No registrar valores reales en documentación.
- El frontend debe resolver URL por entorno antes de producción.
- CORS debe restringirse a orígenes aprobados antes de producción.
- La rotación de secretos pertenece a la plataforma de despliegue.
