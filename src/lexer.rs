//! Tokenización determinista para el lenguaje mínimo del curso.
//!
//! El lexer trabaja con bytes ASCII para que el vínculo entre fuente y `Span`
//! sea directo. La fase inicial no intenta normalizar Unicode ni reconocer
//! comentarios o cadenas; esos límites son una decisión explícita del capítulo.

/// Rango de bytes de un elemento dentro del código fuente, con límite final exclusivo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// Offset del primer byte incluido.
    pub start: usize,
    /// Offset inmediatamente posterior al último byte incluido.
    pub end: usize,
}

impl Span {
    /// Crea un rango de bytes con límite final exclusivo.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// Clase de token reconocida por el lexer inicial.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// Palabra reservada para una vinculación de nombre.
    Let,
    /// Nombre ASCII que no es palabra reservada.
    Identifier(String),
    /// Literal entero decimal que cabe en `i64`.
    Integer(i64),
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    LeftParen,
    RightParen,
    Semicolon,
    /// Marca el final de la entrada, incluso después de un diagnóstico.
    Eof,
}

/// Diagnóstico mínimo de un byte que no pertenece al lenguaje inicial.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexError {
    pub span: Span,
    pub byte: u8,
}

/// Resultado de una tokenización que puede conservar errores recuperables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexResult {
    /// Tokens y spans en el orden en que aparecen en el código fuente.
    pub tokens: Vec<(TokenKind, Span)>,
    /// Diagnósticos producidos sin interrumpir el reconocimiento posterior.
    pub errors: Vec<LexError>,
}

/// Convierte una entrada ASCII del lenguaje mínimo en tokens y diagnósticos.
///
/// ```
/// use rust_compiler_design::lexer::{lex, TokenKind};
///
/// let result = lex("let count = 2;");
/// assert!(matches!(result.tokens[0].0, TokenKind::Let));
/// assert!(result.errors.is_empty());
/// ```
#[must_use]
pub fn lex(source: &str) -> LexResult {
    let bytes = source.as_bytes();
    let mut offset = 0;
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    while offset < bytes.len() {
        let start = offset;
        let byte = bytes[offset];

        if byte.is_ascii_whitespace() {
            offset += 1;
            continue;
        }

        let kind = match byte {
            b'+' => Some(TokenKind::Plus),
            b'-' => Some(TokenKind::Minus),
            b'*' => Some(TokenKind::Star),
            b'/' => Some(TokenKind::Slash),
            b'=' => Some(TokenKind::Equal),
            b'(' => Some(TokenKind::LeftParen),
            b')' => Some(TokenKind::RightParen),
            b';' => Some(TokenKind::Semicolon),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                offset += 1;
                while offset < bytes.len() && is_identifier_continue(bytes[offset]) {
                    offset += 1;
                }

                let name = &source[start..offset];
                tokens.push((keyword_or_identifier(name), Span::new(start, offset)));
                continue;
            }
            b'0'..=b'9' => {
                offset += 1;
                while offset < bytes.len() && bytes[offset].is_ascii_digit() {
                    offset += 1;
                }

                let digits = &source[start..offset];
                match digits.parse::<i64>() {
                    Ok(value) => tokens.push((TokenKind::Integer(value), Span::new(start, offset))),
                    Err(_) => errors.push(LexError {
                        span: Span::new(start, offset),
                        byte,
                    }),
                }
                continue;
            }
            _ => None,
        };

        offset += 1;
        match kind {
            Some(kind) => tokens.push((kind, Span::new(start, offset))),
            None => errors.push(LexError {
                span: Span::new(start, offset),
                byte,
            }),
        }
    }

    tokens.push((TokenKind::Eof, Span::new(bytes.len(), bytes.len())));
    LexResult { tokens, errors }
}

fn is_identifier_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn keyword_or_identifier(name: &str) -> TokenKind {
    match name {
        "let" => TokenKind::Let,
        _ => TokenKind::Identifier(name.to_owned()),
    }
}
