use byteorder::{ByteOrder, LittleEndian};
use hardware::instructions::execute_instruction;

mod hardware;

fn main() {
    let mut vm = hardware::VM::new();
    execute_instruction(&mut vm, 0b1110_001_000000010);
    execute_instruction(&mut vm, 0b0010_001_001_000_001);
    build_example_instruction_set();
}

fn build_example_instruction_set() {
    let mut buf = [0; 2];
    LittleEndian::write_u16(&mut buf, 0b1110_001_000000010);
    println!("{:b}", LittleEndian::read_u16(&buf));
}
