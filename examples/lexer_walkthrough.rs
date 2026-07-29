use rust_compiler_design::lexer::lex;

fn main() {
    let source = "let total = 12 + 3;";
    let result = lex(source);

    for (kind, span) in result.tokens {
        println!("{kind:?} @ {}..{}", span.start, span.end);
    }

    for error in result.errors {
        println!(
            "byte desconocido {:?} @ {}..{}",
            error.byte as char, error.span.start, error.span.end
        );
    }
}
