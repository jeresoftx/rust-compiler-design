//! Resolución de nombres y comprobación del único tipo inicial.

use std::collections::HashMap;

use crate::{
    ast::{AstExpression, AstProgram, AstStatement},
    lexer::Span,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Integer,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticError {
    pub span: Span,
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Analysis {
    pub types: Vec<Type>,
    pub errors: Vec<SemanticError>,
}

#[must_use]
pub fn analyze(program: &AstProgram) -> Analysis {
    let mut analyzer = Analyzer {
        symbols: HashMap::new(),
        result: Analysis::default(),
    };
    for statement in &program.statements {
        analyzer.statement(statement);
    }
    analyzer.result
}

struct Analyzer {
    symbols: HashMap<String, Type>,
    result: Analysis,
}

impl Analyzer {
    fn statement(&mut self, statement: &AstStatement) {
        match statement {
            AstStatement::Let { name, value, .. } => {
                if self.expression(value).is_some() {
                    self.symbols.insert(name.clone(), Type::Integer);
                }
            }
            AstStatement::Expression { expression, .. } => {
                self.expression(expression);
            }
        }
    }

    fn expression(&mut self, expression: &AstExpression) -> Option<Type> {
        match expression {
            AstExpression::Integer { .. } => self.record(),
            AstExpression::Name { value, span } => {
                if self.symbols.contains_key(value) {
                    self.record()
                } else {
                    self.result.errors.push(SemanticError {
                        span: *span,
                        message: format!("nombre no declarado: {value}"),
                    });
                    None
                }
            }
            AstExpression::Prefix { operand, .. } => {
                self.expression(operand).and_then(|_| self.record())
            }
            AstExpression::Binary { left, right, .. } => {
                let left = self.expression(left);
                let right = self.expression(right);
                if left.is_some() && right.is_some() {
                    self.record()
                } else {
                    None
                }
            }
        }
    }

    fn record(&mut self) -> Option<Type> {
        self.result.types.push(Type::Integer);
        Some(Type::Integer)
    }
}
