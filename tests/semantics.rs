use rust_compiler_design::{
    ast::AstProgram,
    lexer::Span,
    parser::parse,
    semantics::{Type, analyze},
};

#[test]
fn resolves_a_binding_before_its_later_use() {
    let parsed = parse("let total = 2; total + 3;");
    let ast = AstProgram::from_parsed(parsed.program, Span::new(0, 25));
    let result = analyze(&ast);

    assert!(result.errors.is_empty());
    assert_eq!(result.types.len(), 4);
    assert!(result.types.iter().all(|kind| *kind == Type::Integer));
}

#[test]
fn reports_an_unknown_name_without_inserting_it() {
    let parsed = parse("missing;");
    let ast = AstProgram::from_parsed(parsed.program, Span::new(0, 8));
    let result = analyze(&ast);

    assert_eq!(result.errors.len(), 1);
    assert!(result.errors[0].message.contains("missing"));
}
