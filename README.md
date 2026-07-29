# Rust Compiler Design

Curso-proyecto complementario de Jeresoft Academy para aprender diseño de
compiladores en Rust mediante la construcción gradual de un lenguaje pequeño.
Recorre lexer, parser, AST, análisis semántico, representación intermedia,
optimizador, bytecode y máquina virtual.

El curso enseña cada fase como una respuesta a un problema del pipeline: qué
información produce, qué invariante conserva, qué alternativa se descartó y
cómo se verifica su resultado (RFC-0001 §2 y §13).

## Lugar En El Camino

Es un curso complementario y un proyecto integrador. Reutiliza Rust, testing,
estructuras de datos y diseño de APIs sin competir con sus capítulos canónicos.

**Nivel:** intermedio-avanzado. **Prerequisitos:** Rust básico, ownership,
tipos algebraicos, pruebas y estructuras de datos elementales.

## Pipeline Del Curso

1. Lexer.
2. Parser.
3. AST.
4. Análisis semántico.
5. Representación intermedia.
6. Optimizador.
7. Bytecode.
8. Máquina virtual.

El alcance, las dependencias y los criterios de aceptación viven en el
[plan versionado](docs/superpowers/plans/2026-07-29-rust-compiler-design-course.md).
El avance operativo vive en el [GitHub Project #20](https://github.com/users/jeresoftx/projects/20).

## Estructura Prevista

```text
docs/       Capítulos compatibles con mdBook.
src/        Modelo educativo del compilador y la máquina virtual.
examples/   Programas fuente y recorridos del pipeline.
tests/      Pruebas de integración por fase.
diagrams/   Diagramas Mermaid y material de apoyo.
```

## Gobernanza

- La estructura sigue RFC-0001 §15; los capítulos aplicarán §14 y §16.
- El plan se convierte en milestones, issues y GitHub Project antes de código.
- Cada issue y PR debe pertenecer al mismo GitHub Project, conforme a RFC-0002.
- El código previsto usa `MIT OR Apache-2.0`; el contenido educativo usa
  `CC BY-SA 4.0`.
- Ningún capítulo se marcará como `reviewed` o `published` sin revisión humana.
