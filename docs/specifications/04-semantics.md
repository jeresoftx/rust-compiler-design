# Especificación: análisis semántico

**Estado:** draft  
**Dependencia:** [AST](03-ast.md)  
**Issue:** #11

## Concepto y problema

La sintaxis puede aceptar `total + missing`, aunque `missing` no exista. El
análisis semántico asigna significado: mantiene scopes, resuelve nombres y
comprueba que los operadores reciban valores compatibles.

## Contrato mínimo

El lenguaje inicial tiene un único tipo, `Integer`. Cada `let` introduce un
nombre después de analizar su valor; por tanto, un binding no puede leerse a sí
mismo en su inicializador. Los nombres se buscan en el scope actual y los
diagnósticos conservan el span del uso que falló.

## Invariantes

1. Un nombre usado debe existir en el scope antes de su uso.
2. Toda expresión aceptada tiene tipo `Integer`.
3. Un binding se inserta solo si su valor no tuvo error semántico.
4. El análisis no altera la estructura ni el orden del AST.

## Alternativas y límites

Se adopta un `HashMap<String, Type>` por scope porque hace visible la operación
de búsqueda. Una tabla de símbolos con identificadores internados se difiere:
ahorra asignaciones, pero introduce una optimización sin hipótesis ni datos.
No hay scopes anidados, inferencia, coerciones ni tipos booleanos todavía.
