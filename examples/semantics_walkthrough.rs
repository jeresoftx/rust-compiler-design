use rust_compiler_design::{ast::AstProgram, lexer::Span, parser::parse, semantics::analyze};

fn main() {
    let parsed = parse("let total = 2; total + 3;");
    let ast = AstProgram::from_parsed(parsed.program, Span::new(0, 25));
    println!("{:#?}", analyze(&ast));
}
