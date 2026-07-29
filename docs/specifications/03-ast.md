# Especificación: AST

**Estado:** draft  
**Dependencia:** [parser](02-parser.md)  
**Issue:** #8

## Concepto y problema

El árbol de sintaxis abstracta (AST) elimina detalles de recorrido del parser y
conserva la estructura que necesita el análisis semántico. El parser transitorio
demuestra cómo reconocer la gramática; el AST fija una API estable para nombres,
tipos, lowering y diagnósticos.

## Representación

```text
Program { statements, span }
Statement::Let { name, value, span }
Statement::Expression { expression, span }
Expression::Integer { value, span }
Expression::Name { value, span }
Expression::Prefix { operator, operand, span }
Expression::Binary { left, operator, right, span }
```

Cada nodo conserva el `Span` que cubre exactamente los tokens que lo
originaron. Los nombres se almacenan como texto; su resolución no pertenece al
AST.

## Invariantes

1. El span de un nodo padre cubre los spans de todos sus hijos.
2. Los hijos de un binario permanecen en orden fuente.
3. Un `let` tiene nombre, valor y punto y coma reconocidos por el parser.
4. El AST no asigna tipos ni resuelve nombres.

## Alternativas

| Alternativa | Decisión | Límite |
|---|---|---|
| Enum recursivo con `Box` | Adoptada | Claro y suficiente para el lenguaje mínimo. |
| Arena de nodos e identificadores | Diferida | Añade gestión de índices antes de necesitar análisis compartido. |
| Reutilizar el árbol del parser | Rechazada | Mezcla la API educativa de reconocimiento con la representación estable. |

## Conversión y límites

La conversión desde `ParsedProgram` será total solo cuando el parser no tenga
diagnósticos. Los errores ya recuperados no se transforman en nodos falsamente
válidos. No hay nodos para llamadas, bloques, booleanos ni tipos en esta fase;
la representación es segura, estable y sin dependencias externas.
