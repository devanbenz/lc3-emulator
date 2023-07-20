use super::VM;

enum OpCode {
    HALT = 0b1111,
    ADD = 0b0010
}

pub fn decode_operation(instruction: u16) -> Option<OpCode> {
    match instruction >> 12 {
        0b1111 => Some(OpCode::HALT),
        0b0010 => Some(OpCode::ADD),
        _ => unimplemented!()
    }
}

pub fn execute_instruction(vm: &mut VM, instruction: u16) {
    let opcode = decode_operation(instruction);

    match opcode {
        Some(OpCode::ADD) => add(instruction),
        Some(OpCode::HALT) => halt(instruction),
        None => panic!("ERR: Out of bounds error")
    }
}

fn halt(opcode: u16) {
    println!("opcode: {:b} (HALT)", opcode);
}

fn add(opcode: u16) {
    println!("opcode: {:b} (HALT)", opcode);
}
