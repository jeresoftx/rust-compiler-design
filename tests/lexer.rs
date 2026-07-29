use rust_compiler_design::lexer::{Span, TokenKind, lex};

#[test]
fn tokenizes_a_binding_expression_with_byte_spans() {
    let result = lex("let total = 12 + 3;");

    assert!(result.errors.is_empty());
    assert_eq!(
        result.tokens,
        vec![
            (TokenKind::Let, Span::new(0, 3)),
            (TokenKind::Identifier("total".into()), Span::new(4, 9)),
            (TokenKind::Equal, Span::new(10, 11)),
            (TokenKind::Integer(12), Span::new(12, 14)),
            (TokenKind::Plus, Span::new(15, 16)),
            (TokenKind::Integer(3), Span::new(17, 18)),
            (TokenKind::Semicolon, Span::new(18, 19)),
            (TokenKind::Eof, Span::new(19, 19)),
        ]
    );
}

#[test]
fn reports_unknown_bytes_and_recovers_to_the_next_token() {
    let result = lex("x @ 1");

    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].span, Span::new(2, 3));
    assert_eq!(result.errors[0].byte, b'@');
    assert_eq!(
        result.tokens,
        vec![
            (TokenKind::Identifier("x".into()), Span::new(0, 1)),
            (TokenKind::Integer(1), Span::new(4, 5)),
            (TokenKind::Eof, Span::new(5, 5)),
        ]
    );
}

#[test]
fn always_emits_eof_for_an_empty_input() {
    let result = lex("");

    assert!(result.errors.is_empty());
    assert_eq!(result.tokens, vec![(TokenKind::Eof, Span::new(0, 0))]);
}
