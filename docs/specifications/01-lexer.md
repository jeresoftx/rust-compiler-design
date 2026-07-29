# Especificación: lexer

**Estado:** draft  
**Issue:** #2

## Concepto y problema

El lexer convierte caracteres en tokens con ubicación. Separa el reconocimiento
local de símbolos del parser: una gramática puede razonar sobre `Identificador`,
`Entero` o `Mas` sin volver a inspeccionar bytes y espacios.

## Contrato mínimo

La primera versión reconoce identificadores ASCII, enteros decimales, palabras
reservadas, paréntesis, operadores aritméticos, asignación, punto y coma y fin
de archivo. Cada token conserva un `Span` de inicio y fin en offsets de bytes.
Los espacios se ignoran; un carácter desconocido genera un diagnóstico léxico
con su span y el lexer continúa cuando sea posible.

## Invariantes

1. Los spans avanzan de izquierda a derecha y no cubren bytes fuera de la
   entrada.
2. Concatenar los lexemas de tokens no trivia reproduce los caracteres fuente
   reconocidos en el mismo orden.
3. El lexer siempre termina con un token de fin de archivo, incluso si hubo
   errores.
4. Un error no se convierte silenciosamente en un token válido.

## Alternativas

| Alternativa | Decisión | Límite |
|---|---|---|
| Lexer manual por bytes | Adoptada | Es explícito y suficiente para el lenguaje pequeño. |
| Expresiones regulares | No por ahora | Ocultan los spans y recuperación que el curso debe enseñar. |
| Unicode completo | Fuera de alcance inicial | Requiere definir identificación y offsets de forma más amplia. |

## Límites y decisión educativa

El lexer no normaliza Unicode ni implementa comentarios, cadenas o números de
punto flotante en esta fase. La sintaxis se ampliará solo cuando una fase
posterior necesite demostrar un nuevo invariante. La implementación será segura,
estable y sin dependencias externas.
