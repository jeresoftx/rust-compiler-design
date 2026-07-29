use rust_compiler_design::{
    ast::{AstExpression, AstProgram, AstStatement},
    lexer::Span,
    parser::parse,
};

#[test]
fn converts_a_parsed_binding_without_resolving_its_name() {
    let parsed = parse("let total = 2 + 3;");
    let ast = AstProgram::from_parsed(parsed.program, Span::new(0, 18));

    assert_eq!(ast.span, Span::new(0, 18));
    assert!(
        matches!(ast.statements[0], AstStatement::Let { ref name, value: AstExpression::Binary { .. }, .. } if name == "total")
    );
}
