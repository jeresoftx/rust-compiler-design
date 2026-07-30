use rust_compiler_design::{
    bytecode::{OpCode, generate},
    ir::{Instruction, IrProgram, ValueId},
};
#[test]
fn emits_stack_instructions_from_ir() {
    let code = generate(&IrProgram {
        instructions: vec![
            Instruction::Const {
                destination: ValueId(0),
                value: 7,
            },
            Instruction::Return(ValueId(0)),
        ],
    });
    assert_eq!(
        code.instructions,
        vec![OpCode::PushConstant(7), OpCode::Return]
    );
}
