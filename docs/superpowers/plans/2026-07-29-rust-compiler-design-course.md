# Plan de implementación de Rust Compiler Design

**Estado:** activo · **Actualizado:** 2026-07-29 · **Repositorio:**
[`jeresoftx/rust-compiler-design`](https://github.com/jeresoftx/rust-compiler-design)

## Fuente de verdad

Este documento traduce el currículo de diseño de compiladores de RFC-0001 §10
en trabajo operativo. Su autoridad sigue este orden:

1. la petición directa de Joel para este repositorio;
2. RFC-0001 §2, §10, §13–§17 y §20;
3. RFC-0002 para trazabilidad issue–PR–Project;
4. `AGENTS.md` de este repositorio;
5. los estándares de fuente de verdad, GitHub Delivery Workflow y bloque
   autónomo de Jeresoft Academy.

El [GitHub Project #20](https://github.com/users/jeresoftx/projects/20) es la
representación operativa de este plan.

## Estado actual

- [x] Project #20 creado desde la plantilla oficial, vinculado al repositorio y
  con las vistas Backlog, Roadmap, Critical Path, In Progress, Review / PRs y
  Blocked.
- [x] Cinco milestones, etiquetas y 26 issues accionables creados y asignados.
- [x] Cada item tiene prioridad, estimación humana, fechas, dependencia, ruta
  crítica, área, tipo, responsable y validación esperada.

## Objetivo

Entregar un libro de ingeniería y crate educativo que construya, en Rust y de
punta a punta, un lenguaje pequeño. El lector debe poder seguir una entrada
desde tokens hasta ejecución en una máquina virtual, entendiendo en cada fase
la representación, los invariantes, los diagnósticos y los trade-offs.

## Arquitectura educativa

Cada capítulo sigue RFC-0001 §2: concepto, problema, alternativas,
justificación e implementación. Cada fase se divide en tres slices trazables:

1. especificación: contrato, gramática o invariantes y alternativas;
2. modelo Rust: TDD, implementación mínima y pruebas de borde;
3. capítulo: diagrama Mermaid, ejemplos, ejercicios, soluciones y límites.

El lenguaje educativo tendrá expresiones, bindings y control mínimo solo cuando
una fase lo requiera. No habrá `unsafe`, nightly ni dependencias externas
directas durante el plan. Una ampliación de sintaxis, optimización o dependencia
se decide en el issue correspondiente antes de implementarse.

## Criterio de cierre global

El plan queda completo en `draft` cuando el coordinador, los 24 slices de las
ocho fases y el cierre editorial estén resueltos por PRs trazables. Cada PR debe
tener assignee, milestone, labels y asociación verificada al mismo Project que
su issue; las verificaciones aplicables deben pasar. Los ocho capítulos, sus
modelos, ejemplos, ejercicios, diagramas, ruta de lectura y glosario deben
existir sin marcarse como `reviewed` ni `published`.

## Milestones y roadmap estimado

Las fechas son pronósticos del Roadmap, no compromisos de publicación. Los
milestones no tendrán fechas de vencimiento artificiales.

| Milestone | Ventana estimada | Propósito |
|---|---|---|
| 0. Planeación y fundación | 2026-07-29 a 2026-08-01 | Plan, Project, taxonomía y coordinación. |
| 1. Frontend del lenguaje | 2026-08-03 a 2026-09-04 | Lexer, parser y AST. |
| 2. Semántica e IR | 2026-09-07 a 2026-09-25 | Análisis semántico y representación intermedia. |
| 3. Ejecución y optimización | 2026-09-28 a 2026-10-30 | Optimizador, bytecode y máquina virtual. |
| 4. Integración y cierre editorial | 2026-11-02 a 2026-11-13 | Programas integradores, ruta y auditoría. |

## Ruta crítica

`#1 → #2 → #3 → #4 → #5 → #6 → #7 → #8 → #9 → #10 → #11 → #12 → #13 → #14 → #15 → #16 → #17 → #18 → #19 → #20 → #21 → #22 → #23 → #24 → #25 → #26`

La secuencia conserva el pipeline del compilador. No se cierra una fase antes
de que el contrato y el modelo de su predecesora estén disponibles.

## Issue coordinador

- [x] #1 Coordinar plan, Project y trazabilidad de `rust-compiler-design`.
  - Prioridad: P1. Estimación humana: 1d. Ruta crítica: sí.
  - Cierre: plan en `main`, Project enlazado, campos y vistas verificados,
    issues creados y cada item con fechas, prioridad, duración y dependencia.

## Milestone 1: Frontend del lenguaje

### Capítulo 01: lexer

- [x] #2 Especificar tokens, spans, errores léxicos y contrato de entrada.
- [x] #3 Implementar y probar lexer determinista con diagnósticos mínimos.
- [x] #4 Escribir capítulo, diagrama, ejemplos, ejercicios y soluciones.

### Capítulo 02: parser

- [x] #5 Especificar gramática, precedencia, recuperación y alternativas.
- [x] #6 Implementar y probar parser con precedencia y diagnósticos.
- [x] #7 Escribir capítulo, diagrama, ejemplos, ejercicios y soluciones.

### Capítulo 03: AST

- [x] #8 Especificar nodos, spans, invariantes y alternativas de representación.
- [x] #9 Implementar y probar AST construido desde el parser.
- [x] #10 Escribir capítulo, diagrama, ejemplos, ejercicios y soluciones.

## Milestone 2: Semántica e IR

### Capítulo 04: análisis semántico

- [x] #11 Especificar símbolos, scopes, tipos y diagnósticos semánticos.
- [x] #12 Implementar y probar análisis de nombres y tipos mínimos.
- [x] #13 Escribir capítulo, diagrama, ejemplos, ejercicios y soluciones.

### Capítulo 05: representación intermedia

- [x] #14 Especificar IR, bloques, instrucciones y contrato de lowering.
- [x] #15 Implementar y probar lowering de AST validado a IR.
- [x] #16 Escribir capítulo, diagrama, ejemplos, ejercicios y soluciones.

## Milestone 3: Ejecución y optimización

### Capítulo 06: optimizador

- [ ] #17 Especificar optimizaciones locales, preservación semántica y límites.
- [ ] #18 Implementar y probar optimizador mínimo con equivalencia observable.
- [ ] #19 Escribir capítulo, diagrama, ejemplos, ejercicios y soluciones.

### Capítulo 07: bytecode

- [ ] #20 Especificar instrucciones, constantes, saltos y formato de bytecode.
- [ ] #21 Implementar y probar generación de bytecode desde IR.
- [ ] #22 Escribir capítulo, diagrama, ejemplos, ejercicios y soluciones.

### Capítulo 08: máquina virtual

- [ ] #23 Especificar pila, frames, ejecución, errores y límites de la VM.
- [ ] #24 Implementar y probar máquina virtual y programas integradores.
- [ ] #25 Escribir capítulo, diagrama, ejemplos, ejercicios y soluciones.

## Milestone 4: Integración y cierre editorial

- [ ] #26 Completar ruta de lectura, glosario, referencias cruzadas y auditoría
  editorial sin cambiar capítulos a `reviewed` ni `published`.

## Dependencias y bloqueadores

| Issues | Dependencia | Bloqueador |
|---|---|---|
| #3, #6, #9, #12, #15, #18, #21, #24 | Especificación de su fase | Ninguno adicional. |
| #4, #7, #10, #13, #16, #19, #22, #25 | Modelo de su fase | Ninguno adicional. |
| #26 | #25 | Ninguno; no altera estados editoriales. |
| Cualquier issue de dependencia o `unsafe` | Decisión humana explícita | No se implementa hasta autorización. |

## Contrato de issues y validación

Cada issue debe enlazar este plan e incluir alcance, criterios de aceptación,
prioridad, estimación humana, fechas de roadmap, dependencias, ruta crítica,
bloqueadores, validación y definición de terminado. Se asigna a `jeresoftx`,
tiene milestone, labels y se agrega al Project del curso.

| Tipo | Validación mínima |
|---|---|
| Especificación | Invariantes, gramática o contrato, alternativas, límites, enlaces y `git diff --check`. |
| Modelo | TDD, `cargo fmt --check`, Clippy, pruebas, doctests y diff limpio. |
| Capítulo | Modelo verde, ejemplos ejecutables, Mermaid, ejercicios, soluciones y declaración honesta sobre complejidad o benchmark. |
| Cierre | Índice, enlaces, estados, glosario, ruta de lectura y suite completa. |

## Siguiente bloque recomendado

`#1 → #2 → #3 → #4`: fundación, contrato del lexer, modelo probado y primer
capítulo. No requiere dependencias externas ni `unsafe`.
