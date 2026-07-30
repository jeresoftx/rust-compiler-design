use rust_compiler_design::{
    ir::{Instruction, IrProgram, ValueId},
    optimizer::fold_constants,
    parser::BinaryOperator,
};
fn main() {
    let ir = IrProgram {
        instructions: vec![
            Instruction::Const {
                destination: ValueId(0),
                value: 2,
            },
            Instruction::Const {
                destination: ValueId(1),
                value: 3,
            },
            Instruction::Binary {
                destination: ValueId(2),
                operator: BinaryOperator::Add,
                left: ValueId(0),
                right: ValueId(1),
            },
        ],
    };
    println!("{:#?}", fold_constants(&ir));
}
