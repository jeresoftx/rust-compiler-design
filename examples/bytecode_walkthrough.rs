use rust_compiler_design::{
    bytecode::generate,
    ir::{Instruction, IrProgram, ValueId},
};
fn main() {
    println!(
        "{:#?}",
        generate(&IrProgram {
            instructions: vec![
                Instruction::Const {
                    destination: ValueId(0),
                    value: 7
                },
                Instruction::Return(ValueId(0))
            ]
        })
    );
}
