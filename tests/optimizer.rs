use rust_compiler_design::{
    ir::{Instruction, IrProgram, ValueId},
    optimizer::fold_constants,
    parser::BinaryOperator,
};
#[test]
fn folds_a_binary_operation_with_known_constants() {
    let result = fold_constants(&IrProgram {
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
    });
    assert!(matches!(
        result.instructions[2],
        Instruction::Const { value: 5, .. }
    ));
}
