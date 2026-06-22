# ADR-002: Testing exclusivamente documental

| Campo | Valor |
|---|---|
| Estado | Aceptado |
| Fecha | 2026-06-21 |

## Contexto

SDD requiere escenarios verificables y trazabilidad hacia criterios de aceptación. El workspace prohíbe crear, modificar o ejecutar pruebas, y prohíbe compilar para validar cambios.

## Decisión

Cada feature incluye estrategia, casos, aceptación y pruebas no funcionales en Markdown. Se documentan precondiciones, pasos, resultados esperados y cobertura, pero no se generan tests, fixtures, mocks, scripts o comandos ejecutables.

## Consecuencias

- Los requisitos siguen siendo verificables a nivel de especificación.
- La ejecución queda fuera del alcance del agente y del flujo documental.
- Un caso se marca `No ejecutado` salvo evidencia externa autorizada.
- La documentación nunca afirma resultados obtenidos por inferencia.
