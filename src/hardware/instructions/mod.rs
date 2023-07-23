use super::{registers::Registers, VM};

enum OpCode {
    HALT = 0b1111,
    ADD = 0b0010,
    LEA = 0b1110,
}

pub fn execute_instruction(vm: &mut VM, instruction: u16) {
    let opcode = decode_operation(instruction);

    match opcode {
        Some(OpCode::ADD) => add(instruction, vm.registers),
        Some(OpCode::HALT) => halt(instruction),
        Some(OpCode::LEA) => lea(instruction, vm),
        None => panic!("ERR: Out of bounds error"),
    }
}

fn decode_operation(instruction: u16) -> Option<OpCode> {
    match instruction >> 12 {
        0b1111 => Some(OpCode::HALT),
        0b0010 => Some(OpCode::ADD),
        0b1110 => Some(OpCode::LEA),
        _ => unimplemented!(),
    }
}

fn get_rd(instruction: u16) -> u8 {
    ((instruction >> 9) & 0b0000_0111) as u8
}

fn get_rs1(instruction: u16) -> u8 {
    ((instruction >> 6) & 0b0000_0111) as u8
}

fn get_rs2(instruction: u16) -> u8 {
    (instruction & 0b0000_0111) as u8
}

fn get_mode(instruction: u16) -> u8 {
    ((instruction >> 5) & 0b0000_0001) as u8
}

fn halt(opcode: u16) {
    println!("opcode: {:b} (HALT)", opcode);
}

fn sext(instruction: u16, offset: usize) -> i16 {
    let sign = instruction >> offset - 1;
    if sign == 0 {
        instruction as i16
    } else {
        (((!instruction) | 0b1111_1111_0000_0000) + 1) as i16
    }
}

fn lea(instruction: u16, vm: &mut VM) {
    let rd: u8 = get_rd(instruction);
    let imm = instruction & 0b0000_0001_1111_1111;
    let sext_imm = sext(imm, 9);

    vm.load_register(rd, sext_imm as u16);
}

fn add(instruction: u16, registers: Registers) {
    let rd = get_rd(instruction);
    let rs1 = get_rs1(instruction);
    let rs2 = get_rs2(instruction);
    let m: u8 = get_mode(instruction);

    let result = registers.read(rs1) + registers.read(rs2);

    println!(
        "opcode: {:b} (ADD) rd: {}, rs1: {}, rs2: {}, mode: {}",
        instruction, rd, rs1, rs2, m
    );
    println!("result: {}", result);
}
