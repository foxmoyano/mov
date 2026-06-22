# Spec-Driven Development de MOV

Esta carpeta implementa un flujo SDD centrado en features. La especificación define qué debe construirse; investigación, plan, contratos y tareas derivan de ella; código, release y operación conservan trazabilidad hacia la intención original.

## Flujo canónico

```text
Constitution
  -> Specify
  -> Clarify
  -> Plan + Research + Design
  -> Tasks
  -> Analyze
  -> Implement
  -> Testing documental
  -> Release
  -> Deployment
  -> Operations feedback
  -> actualización de la Specification
```

## Estructura

```text
docs/
├── governance/constitution.md
├── product/
│   ├── vision.md
│   ├── glossary.md
│   └── roadmap.md
├── features/NNN-nombre-feature/
├── adr/
├── manuals/
└── templates/
```

## Artefactos globales

- [Constitución](governance/constitution.md): principios obligatorios para todas las features.
- [Visión](product/vision.md): problema, actores, objetivos y límites del producto.
- [Glosario](product/glossary.md): lenguaje común del dominio.
- [Roadmap](product/roadmap.md): features candidatas, sin autorizar su implementación.
- [ADR](adr/README.md): decisiones transversales vigentes.
- [Manuales](manuals/README.md): comportamiento observable para usuarios y operación funcional.
- [Plantillas](templates/README.md): estructura reutilizable de nuevos paquetes.

## Paquete de una feature

Cada cambio usa `docs/features/NNN-nombre-feature/` y contiene:

| Orden | Artefacto | Propósito |
|---:|---|---|
| 1 | `spec.md` | Historias, requisitos y escenarios de aceptación |
| 2 | `clarifications.md` | Decisiones sobre ambigüedades |
| 3 | `checklists/requirements.md` | Calidad de la especificación |
| 4 | `research.md` | Contexto, alternativas y fundamentos |
| 5 | `plan.md` | Traducción técnica de la spec |
| 6 | `architecture.md`, `data-model.md`, `contracts/`, `ui/`, `configuration.md` | Diseño derivado |
| 7 | `tasks.md` | Trabajo ordenado y trazable |
| 8 | `analysis.md` | Cobertura y consistencia entre artefactos |
| 9 | `implementation.md` | Evidencia y desviaciones de implementación |
| 10 | `testing/` | Estrategia y casos exclusivamente documentales |
| 11 | `quickstart.md` | Recorrido funcional esperado, sin ejecución |
| 12 | `release.md`, `deployment.md`, `operations.md` | Entrega, reversión y feedback |

La feature actual es [001-video-catalog](features/001-video-catalog/README.md).

## Gates

1. No planificar con preguntas bloqueantes abiertas.
2. No crear tareas sin requisitos ni requisitos sin escenarios de aceptación.
3. No implementar sin plan, contratos y análisis aprobados.
4. No declarar implementación completa con desviaciones sin registrar.
5. No liberar sin trazabilidad, configuración, riesgos y rollback.
6. Los hallazgos operacionales actualizan la spec; no quedan como hotfix aislado.

## Testing documental

`testing/` especifica estrategia, casos, aceptación y requisitos no funcionales. Estos artefactos describen qué debería comprobar un proceso autorizado, pero no crean archivos de pruebas, fixtures, mocks o scripts ejecutables. El agente no ejecuta pruebas ni compilaciones.

## Estados

`Borrador → En revisión → Aprobado → Implementado → Desplegado → Obsoleto`

Cada documento indica estado, versión, fecha y responsable. Un artefacto obsoleto enlaza su reemplazo.

## Referencias

- GitHub Spec Kit: <https://github.com/github/spec-kit>
- Metodología SDD: <https://github.com/github/spec-kit/blob/main/spec-driven.md>
