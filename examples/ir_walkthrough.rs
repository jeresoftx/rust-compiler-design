use rust_compiler_design::{ast::AstProgram, ir::lower, lexer::Span, parser::parse};

fn main() {
    let parsed = parse("let x = 4; x * 2;");
    let ast = AstProgram::from_parsed(parsed.program, Span::new(0, 17));
    println!("{:#?}", lower(&ast));
}
