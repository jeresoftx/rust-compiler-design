use rust_compiler_design::{
    bytecode::{Bytecode, OpCode},
    vm::{VmError, execute},
};
#[test]
fn executes_arithmetic_program() {
    assert_eq!(
        execute(&Bytecode {
            instructions: vec![
                OpCode::PushConstant(2),
                OpCode::PushConstant(3),
                OpCode::Add,
                OpCode::Return
            ]
        }),
        Ok(5)
    );
}
#[test]
fn reports_division_by_zero() {
    assert_eq!(
        execute(&Bytecode {
            instructions: vec![
                OpCode::PushConstant(1),
                OpCode::PushConstant(0),
                OpCode::Divide,
                OpCode::Return
            ]
        }),
        Err(VmError::DivisionByZero)
    );
}
