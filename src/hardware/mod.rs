use self::registers::Registers;

pub mod instructions;
pub mod registers;

const MEMORY_SIZE: usize = 1 << 16;

pub struct VM {
    pub memory: [u16; MEMORY_SIZE],
    pub registers: Registers,
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0x0; MEMORY_SIZE],
            registers: Registers::new(),
        }
    }

    pub fn write_memory(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }

    pub fn read_register(&self, register: u8) -> u16 {
        self.registers.read(register)
    }

    pub fn load_register(&mut self, register: u8, value: u16) {
        self.registers.load(register, value)
    }
}
