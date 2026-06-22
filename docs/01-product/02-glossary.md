# Glosario de MOV

| Término | Definición |
|---|---|
| Catálogo | Colección paginada de videos consultable por el usuario |
| Video | Registro identificado por UUID con metadatos en PostgreSQL |
| Detalle | Vista contractual ampliada de un video y sus imágenes |
| Imagen principal | Objeto `media/{uuid}/main.jpg` almacenado en MinIO |
| Escena | Imagen bajo `media/{uuid}/images/` |
| URL firmada | URL temporal de lectura generada por el backend |
| Página | Segmento del catálogo con índice base cero |
| Tamaño de página | Máximo de elementos solicitados; límite backend actual de 100 |
| Feature | Unidad versionada de intención, diseño, tareas e implementación SDD |
| Spec | Fuente de verdad funcional de una feature |
| Gate | Condición documental necesaria para avanzar de etapa |
| Brecha | Diferencia conocida entre intención, contrato o implementación |

Los términos nuevos se agregan aquí cuando afectan a más de una feature. Los términos locales permanecen en la spec correspondiente.
