use rust_compiler_design::{
    ast::AstProgram,
    ir::{Instruction, ValueId, lower},
    lexer::Span,
    parser::parse,
};
#[test]
fn lowers_left_to_right_and_returns_expression_value() {
    let parsed = parse("2 + 3;");
    let ast = AstProgram::from_parsed(parsed.program, Span::new(0, 6));
    let ir = lower(&ast);
    assert_eq!(
        ir.instructions.last(),
        Some(&Instruction::Return(ValueId(2)))
    );
    assert_eq!(ir.instructions.len(), 4);
}
