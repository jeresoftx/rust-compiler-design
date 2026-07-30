# Ruta de lectura y glosario

**Estado:** draft

## Ruta de lectura

1. Lee [Lexer](01-lexer.md) y ejecuta su ejemplo para entender tokens y spans.
2. Continúa con [Parser](02-parser.md) y [AST](03-ast.md): forma antes de
   significado.
3. Estudia [Semántica](04-semantics.md) y [IR](05-ir.md): nombres, tipos y
   lowering.
4. Recorre [Optimizador](06-optimizer.md), [Bytecode](07-bytecode.md) y
   [Máquina virtual](08-vm.md) para seguir el resultado ejecutable.

Ejecuta toda la evidencia con `cargo test --all-targets` y los ejemplos en
`examples/`. Los capítulos son material en revisión humana diferida: ninguno
está marcado como `reviewed` ni `published`.

## Glosario

| Término | Definición |
|---|---|
| AST | Árbol de sintaxis abstracta estable para fases posteriores. |
| Bytecode | Instrucciones de pila generadas desde IR. |
| Diagnóstico | Error con ubicación y mensaje, sin ocultar la causa. |
| IR | Representación intermedia lineal con temporales. |
| Lexer | Etapa que convierte fuente en tokens y spans. |
| Lowering | Conversión que preserva significado entre representaciones. |
| Span | Rango de bytes de una pieza de fuente. |
| VM | Máquina virtual que ejecuta bytecode. |
