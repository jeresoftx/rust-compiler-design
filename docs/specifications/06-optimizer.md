# Especificación: optimizador

**Estado:** draft  
**Dependencia:** [IR](05-ir.md)  
**Issue:** #17

## Propósito

El optimizador transforma IR sin cambiar su resultado observable. El primer
pase aplica folding de constantes: si una operación binaria recibe dos `Const`
conocidas, puede reemplazarse por una sola constante.

## Invariantes

1. Para programas sin error, IR original y optimizada producen el mismo entero.
2. El pase no reordena instrucciones con efectos como `Store` o `Return`.
3. Solo se sustituye una operación cuando ambos operandos son constantes
   conocidas en ese punto.

## Alternativas y límites

Un peephole local se adopta por ser trazable. Propagación global, eliminación de
código muerto y reordenamientos se difieren hasta tener CFG y mediciones. La
división por cero no se pliega: conserva el comportamiento de ejecución.
