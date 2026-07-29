# Especificación: parser

**Estado:** draft  
**Dependencia:** [lexer](01-lexer.md)  
**Issue:** #5

## Concepto y problema

El parser transforma tokens en estructura. El lexer ya sabe que `+` es un
símbolo, pero no si `1 + 2 * 3` significa `(1 + 2) * 3` o `1 + (2 * 3)`.
Centralizar esa decisión evita que cada consumidor invente su propia
precedencia.

## Gramática inicial

```text
programa      := sentencia* EOF
sentencia     := "let" identificador "=" expresión ";" | expresión ";"
expresión     := suma
suma          := producto (("+" | "-") producto)*
producto      := prefijo (("*" | "/") prefijo)*
prefijo       := ("-" prefijo) | primario
primario      := entero | identificador | "(" expresión ")"
```

La precedencia asciende de suma a producto y prefijo. Todos los operadores
binarios iniciales son asociativos por la izquierda; por ejemplo, `8 - 3 - 1`
se agrupa como `(8 - 3) - 1`.

## Invariantes

1. Cada nodo sintáctico procede de tokens contiguos y conserva el span que los
   cubre.
2. Una expresión binaria no pierde ni reordena sus operandos.
3. Un programa aceptado termina en `Eof`; no quedan tokens ignorados.
4. Un diagnóstico sintáctico no convierte una entrada incompleta en un árbol
   aparentemente válido.

## Recuperación

Después de un error, el parser avanza hasta un punto de sincronización: `;` o
`Eof`. Esto permite informar más de un error sin intentar reconstruir una
expresión arbitraria. La primera versión prioriza diagnósticos honestos sobre
una recuperación sofisticada.

## Alternativas

| Alternativa | Decisión | Motivo |
|---|---|---|
| Descenso recursivo por niveles | Adoptada | Muestra precedencia y recuperación de forma directa. |
| Pratt parser | Diferida | Es extensible, pero añade un protocolo que distrae del primer AST. |
| Generador de parsers | Fuera de alcance | Oculta el control de errores y la relación token-árbol. |

## Límites

No hay llamadas, bloques, tipos, booleanos ni asignación como expresión. La
única vinculación es `let`; las decisiones de nombres y tipos pertenecen al
análisis semántico. La implementación será estable, segura y sin dependencias
externas.
