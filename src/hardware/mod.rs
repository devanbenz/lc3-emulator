use self::registers::Registers;

pub mod registers;
pub mod instructions;

const MEMORY_SIZE: usize = 1 << 16;

pub struct VM {
    memory: [u16; MEMORY_SIZE],
    registers: Registers
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0x0; MEMORY_SIZE],
            registers: Registers::new()
        }
    }

    pub fn write_memory(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }
}


