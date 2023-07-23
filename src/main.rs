use hardware::instructions::execute_instruction;

mod hardware;

fn main() {
    let mut vm = hardware::VM::new();
    execute_instruction(&mut vm, 0b1110_001_000000010);
    execute_instruction(&mut vm, 0b0010_001_001_000_001);
}
