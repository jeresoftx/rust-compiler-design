use rust_compiler_design::parser::parse;

fn main() {
    let result = parse("let total = 1 + 2 * 3; total;");
    println!("{:#?}", result.program);
    for error in result.errors {
        println!(
            "{} @ {}..{}",
            error.message, error.span.start, error.span.end
        );
    }
}
