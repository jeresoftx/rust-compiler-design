# Especificación: bytecode

**Estado:** draft  
**Dependencia:** [IR](05-ir.md)  
**Issue:** #20

## Propósito

El bytecode convierte instrucciones de IR en un formato compacto que una VM
puede recorrer. La primera versión usa instrucciones tipadas, no bytes crudos,
para hacer visibles pila, constantes y saltos antes de discutir serialización.

## Instrucciones

`PushConstant(i64)`, `Load(String)`, `Store(String)`, `Add`, `Subtract`,
`Multiply`, `Divide` y `Return`. Las aritméticas retiran dos valores de la pila
y empujan uno. `Return` consume el resultado final.

## Invariantes y límites

1. Cada operación aritmética exige dos valores en pila.
2. Un programa generado termina con `Return`.
3. El generador conserva el orden de IR.

No hay saltos ni tabla de constantes todavía: ambos se introducen cuando haya
control de flujo o repetición medible. Esta fase permanece segura y sin
dependencias externas.
