use rust_compiler_design::{ast::AstProgram, lexer::Span, parser::parse};

fn main() {
    let parsed = parse("let total = 2 + 3;");
    let ast = AstProgram::from_parsed(parsed.program, Span::new(0, 18));
    println!("{ast:#?}");
}
