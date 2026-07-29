use rust_compiler_design::parser::{BinaryOperator, ParsedExpr, ParsedStatement, parse};

#[test]
fn gives_product_precedence_over_sum() {
    let result = parse("1 + 2 * 3;");

    assert!(result.errors.is_empty());
    assert_eq!(
        result.program.statements,
        vec![ParsedStatement::Expression(ParsedExpr::Binary {
            left: Box::new(ParsedExpr::Integer(1)),
            operator: BinaryOperator::Add,
            right: Box::new(ParsedExpr::Binary {
                left: Box::new(ParsedExpr::Integer(2)),
                operator: BinaryOperator::Multiply,
                right: Box::new(ParsedExpr::Integer(3)),
            }),
        })]
    );
}

#[test]
fn parses_a_binding_and_a_following_expression() {
    let result = parse("let total = 8 - 3 - 1; total;");

    assert!(result.errors.is_empty());
    assert_eq!(result.program.statements.len(), 2);
    assert!(matches!(
        result.program.statements[0],
        ParsedStatement::Let { .. }
    ));
    assert!(
        matches!(result.program.statements[1], ParsedStatement::Expression(ParsedExpr::Identifier(ref name)) if name == "total")
    );
}

#[test]
fn synchronizes_at_semicolon_after_a_syntax_error() {
    let result = parse("let = 1; 2;");

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
        result.program.statements,
        vec![ParsedStatement::Expression(ParsedExpr::Integer(2))]
    );
}
