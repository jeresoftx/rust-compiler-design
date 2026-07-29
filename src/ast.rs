//! Representación sintáctica estable construida desde el parser.

use crate::lexer::Span;
use crate::parser::{BinaryOperator, ParsedExpr, ParsedProgram, ParsedStatement};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstProgram {
    pub statements: Vec<AstStatement>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AstStatement {
    Let {
        name: String,
        value: AstExpression,
        span: Span,
    },
    Expression {
        expression: AstExpression,
        span: Span,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AstExpression {
    Integer {
        value: i64,
        span: Span,
    },
    Name {
        value: String,
        span: Span,
    },
    Prefix {
        operand: Box<Self>,
        span: Span,
    },
    Binary {
        left: Box<Self>,
        operator: BinaryOperator,
        right: Box<Self>,
        span: Span,
    },
}

impl AstProgram {
    /// Convierte un árbol sintáctico ya aceptado en el AST estable del curso.
    #[must_use]
    pub fn from_parsed(program: ParsedProgram, span: Span) -> Self {
        let statements = program
            .statements
            .into_iter()
            .map(|statement| match statement {
                ParsedStatement::Let { name, value } => AstStatement::Let {
                    name,
                    value: AstExpression::from_parsed(value, span),
                    span,
                },
                ParsedStatement::Expression(expression) => AstStatement::Expression {
                    expression: AstExpression::from_parsed(expression, span),
                    span,
                },
            })
            .collect();
        Self { statements, span }
    }
}

impl AstExpression {
    fn from_parsed(expression: ParsedExpr, span: Span) -> Self {
        match expression {
            ParsedExpr::Integer(value) => Self::Integer { value, span },
            ParsedExpr::Identifier(value) => Self::Name { value, span },
            ParsedExpr::PrefixMinus(operand) => Self::Prefix {
                operand: Box::new(Self::from_parsed(*operand, span)),
                span,
            },
            ParsedExpr::Binary {
                left,
                operator,
                right,
            } => Self::Binary {
                left: Box::new(Self::from_parsed(*left, span)),
                operator,
                right: Box::new(Self::from_parsed(*right, span)),
                span,
            },
        }
    }
}
