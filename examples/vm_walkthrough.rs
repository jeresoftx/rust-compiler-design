use rust_compiler_design::{
    bytecode::{Bytecode, OpCode},
    vm::execute,
};
fn main() {
    println!(
        "{:?}",
        execute(&Bytecode {
            instructions: vec![
                OpCode::PushConstant(2),
                OpCode::PushConstant(3),
                OpCode::Add,
                OpCode::Return
            ]
        })
    );
}
