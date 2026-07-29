//! Descenso recursivo por niveles para la gramática inicial.
//!
//! Este módulo produce un árbol sintáctico transitorio. El AST estable se
//! introduce como una fase explícita del curso después de fijar la gramática.

use crate::lexer::{Span, TokenKind, lex};

/// Operadores binarios admitidos por la gramática inicial.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Expresión sintáctica previa al AST canónico.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParsedExpr {
    Integer(i64),
    Identifier(String),
    PrefixMinus(Box<Self>),
    Binary {
        left: Box<Self>,
        operator: BinaryOperator,
        right: Box<Self>,
    },
}

/// Sentencia del lenguaje inicial.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParsedStatement {
    Let { name: String, value: ParsedExpr },
    Expression(ParsedExpr),
}

/// Programa sintáctico completo.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParsedProgram {
    pub statements: Vec<ParsedStatement>,
}

/// Diagnóstico producido mientras se conserva una ruta de recuperación.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub span: Span,
    pub message: String,
}

/// Resultado de analizar una fuente, incluidos los errores recuperables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseResult {
    pub program: ParsedProgram,
    pub errors: Vec<ParseError>,
}

/// Analiza una entrada usando la gramática declarada en `02-parser.md`.
#[must_use]
pub fn parse(source: &str) -> ParseResult {
    let lexed = lex(source);
    let mut parser = Parser {
        tokens: lexed.tokens,
        cursor: 0,
        errors: lexed
            .errors
            .into_iter()
            .map(|error| ParseError {
                span: error.span,
                message: format!("byte desconocido {:?}", error.byte as char),
            })
            .collect(),
    };
    let program = parser.parse_program();

    ParseResult {
        program,
        errors: parser.errors,
    }
}

struct Parser {
    tokens: Vec<(TokenKind, Span)>,
    cursor: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    fn parse_program(&mut self) -> ParsedProgram {
        let mut statements = Vec::new();
        while !self.at_eof() {
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(()) => self.synchronize(),
            }
        }
        ParsedProgram { statements }
    }

    fn parse_statement(&mut self) -> Result<ParsedStatement, ()> {
        if self.matches(|kind| matches!(kind, TokenKind::Let)) {
            let name = match self.advance().0 {
                TokenKind::Identifier(name) => name,
                _ => return self.error_here("se esperaba un identificador después de let"),
            };
            self.expect(
                |kind| matches!(kind, TokenKind::Equal),
                "se esperaba = después del nombre",
            )?;
            let value = self.parse_expression()?;
            self.expect(
                |kind| matches!(kind, TokenKind::Semicolon),
                "se esperaba ; después de la expresión",
            )?;
            Ok(ParsedStatement::Let { name, value })
        } else {
            let expression = self.parse_expression()?;
            self.expect(
                |kind| matches!(kind, TokenKind::Semicolon),
                "se esperaba ; después de la expresión",
            )?;
            Ok(ParsedStatement::Expression(expression))
        }
    }

    fn parse_expression(&mut self) -> Result<ParsedExpr, ()> {
        self.parse_sum()
    }

    fn parse_sum(&mut self) -> Result<ParsedExpr, ()> {
        let mut expression = self.parse_product()?;
        loop {
            let operator = if self.matches(|kind| matches!(kind, TokenKind::Plus)) {
                Some(BinaryOperator::Add)
            } else if self.matches(|kind| matches!(kind, TokenKind::Minus)) {
                Some(BinaryOperator::Subtract)
            } else {
                None
            };
            let Some(operator) = operator else { break };
            expression = ParsedExpr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(self.parse_product()?),
            };
        }
        Ok(expression)
    }

    fn parse_product(&mut self) -> Result<ParsedExpr, ()> {
        let mut expression = self.parse_prefix()?;
        loop {
            let operator = if self.matches(|kind| matches!(kind, TokenKind::Star)) {
                Some(BinaryOperator::Multiply)
            } else if self.matches(|kind| matches!(kind, TokenKind::Slash)) {
                Some(BinaryOperator::Divide)
            } else {
                None
            };
            let Some(operator) = operator else { break };
            expression = ParsedExpr::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(self.parse_prefix()?),
            };
        }
        Ok(expression)
    }

    fn parse_prefix(&mut self) -> Result<ParsedExpr, ()> {
        if self.matches(|kind| matches!(kind, TokenKind::Minus)) {
            return Ok(ParsedExpr::PrefixMinus(Box::new(self.parse_prefix()?)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<ParsedExpr, ()> {
        let (kind, _) = self.advance();
        match kind {
            TokenKind::Integer(value) => Ok(ParsedExpr::Integer(value)),
            TokenKind::Identifier(name) => Ok(ParsedExpr::Identifier(name)),
            TokenKind::LeftParen => {
                let expression = self.parse_expression()?;
                self.expect(
                    |kind| matches!(kind, TokenKind::RightParen),
                    "se esperaba ) después de la expresión",
                )?;
                Ok(expression)
            }
            _ => self.error_previous("se esperaba una expresión"),
        }
    }

    fn expect(
        &mut self,
        predicate: impl FnOnce(&TokenKind) -> bool,
        message: &str,
    ) -> Result<(), ()> {
        if predicate(&self.current().0) {
            self.advance();
            Ok(())
        } else {
            self.error_here(message)
        }
    }

    fn matches(&mut self, predicate: impl FnOnce(&TokenKind) -> bool) -> bool {
        if predicate(&self.current().0) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn synchronize(&mut self) {
        while !self.at_eof() {
            if matches!(self.current().0, TokenKind::Semicolon) {
                self.advance();
                break;
            }
            self.advance();
        }
    }

    fn error_here<T>(&mut self, message: &str) -> Result<T, ()> {
        let span = self.current().1;
        self.errors.push(ParseError {
            span,
            message: message.to_owned(),
        });
        Err(())
    }

    fn error_previous<T>(&mut self, message: &str) -> Result<T, ()> {
        let span = self.previous().1;
        self.errors.push(ParseError {
            span,
            message: message.to_owned(),
        });
        Err(())
    }

    fn at_eof(&self) -> bool {
        matches!(self.current().0, TokenKind::Eof)
    }

    fn current(&self) -> &(TokenKind, Span) {
        &self.tokens[self.cursor]
    }

    fn previous(&self) -> &(TokenKind, Span) {
        &self.tokens[self.cursor.saturating_sub(1)]
    }

    fn advance(&mut self) -> (TokenKind, Span) {
        let token = self.current().clone();
        if !matches!(token.0, TokenKind::Eof) {
            self.cursor += 1;
        }
        token
    }
}
