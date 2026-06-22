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
├── 00-governance/01-constitution.md
├── 01-product/
│   ├── 01-vision.md
│   ├── 02-glossary.md
│   └── 03-roadmap.md
├── 02-features/NNN-nombre-feature/
├── 03-adr/
├── 04-manuals/
└── 05-templates/
```

## Artefactos globales

- [Constitución](00-governance/01-constitution.md): principios obligatorios para todas las features.
- [Visión](01-product/01-vision.md): problema, actores, objetivos y límites del producto.
- [Glosario](01-product/02-glossary.md): lenguaje común del dominio.
- [Roadmap](01-product/03-roadmap.md): features candidatas, sin autorizar su implementación.
- [Features](02-features/00-README.md): paquetes SDD del producto.
- [ADR](03-adr/00-README.md): decisiones transversales vigentes.
- [Manuales](04-manuals/00-README.md): comportamiento observable para usuarios y operación funcional.
- [Plantillas](05-templates/00-README.md): estructura reutilizable de nuevos paquetes.

## Paquete de una feature

Cada cambio usa `docs/02-features/NNN-nombre-feature/` y contiene:

| Orden | Artefacto | Propósito |
|---:|---|---|
| 1 | `01-spec.md` | Historias, requisitos y escenarios de aceptación |
| 2 | `02-clarifications.md` | Decisiones sobre ambigüedades |
| 3 | `03-checklists/01-requirements.md` | Calidad de la especificación |
| 4 | `04-research.md` | Contexto, alternativas y fundamentos |
| 5 | `05-plan.md` | Traducción técnica de la spec |
| 6 | `06-architecture.md` a `10-configuration.md` | Diseño derivado |
| 7 | `11-tasks.md` | Trabajo ordenado y trazable |
| 8 | `12-analysis.md` | Cobertura y consistencia entre artefactos |
| 9 | `13-implementation.md` | Evidencia y desviaciones de implementación |
| 10 | `14-testing/` | Estrategia y casos exclusivamente documentales |
| 11 | `15-quickstart.md` | Recorrido funcional esperado, sin ejecución |
| 12 | `16-release.md` a `18-operations.md` | Entrega, reversión y feedback |

La feature actual es [001-video-catalog](02-features/001-video-catalog/00-README.md).

## Gates

1. No planificar con preguntas bloqueantes abiertas.
2. No crear tareas sin requisitos ni requisitos sin escenarios de aceptación.
3. No implementar sin plan, contratos y análisis aprobados.
4. No declarar implementación completa con desviaciones sin registrar.
5. No liberar sin trazabilidad, configuración, riesgos y rollback.
6. Los hallazgos operacionales actualizan la spec; no quedan como hotfix aislado.

## Testing documental

`14-testing/` especifica estrategia, casos, aceptación y requisitos no funcionales. Estos artefactos describen qué debería comprobar un proceso autorizado, pero no crean archivos de pruebas, fixtures, mocks o scripts ejecutables. El agente no ejecuta pruebas ni compilaciones.

## Estados

`Borrador → En revisión → Aprobado → Implementado → Desplegado → Obsoleto`

Cada documento indica estado, versión, fecha y responsable. Un artefacto obsoleto enlaza su reemplazo.

## Referencias

- GitHub Spec Kit: <https://github.com/github/spec-kit>
- Metodología SDD: <https://github.com/github/spec-kit/blob/main/spec-driven.md>
