const INIT_PC: u16 = 0x3000;

#[derive(Clone, Copy)]
pub struct Registers {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,
    pc: u16,
    cd: u16
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            r0: 0x0,
            r1: 0x0,
            r2: 0x0,
            r3: 0x0,
            r4: 0x0,
            r5: 0x0,
            r6: 0x0,
            r7: 0x0,
            pc: INIT_PC, // program counter  
            cd: 0x0,     // condition flag
        }
    }

    pub fn load(mut self, register: u8, value: u16) {
        match register {
            0x1 => self.r0 = value,
            0x2 => self.r1 = value,
            0x3 => self.r2 = value,
            0x4 => self.r3 = value,
            0x5 => self.r4 = value,
            0x6 => self.r5 = value,
            0x7 => self.r6 = value,
            0x8 => self.r7 = value,
            0x9 => self.pc = value,
            0x10 => self.cd = value,
            _ => panic!("ERR 0: out of bounds index") 
        }
    }

    pub fn read(self, register: u8) -> u16 {
        match register {
            0x1 => self.r0,
            0x2 => self.r1,
            0x3 => self.r2,
            0x4 => self.r3,
            0x5 => self.r4,
            0x6 => self.r5,
            0x7 => self.r6,
            0x8 => self.r7,
            0x9 => self.pc,
            0x10 => self.cd,
            _ => unimplemented!() 

        }
    }

    pub fn update_cd_flag(&mut self, r: u8) {
        if self.read(r) == 0 {
            self.load(0x10, ConditionFlag::Z as u16);
        } else if self.read(r) >> 15 != 0 {
            self.load(0x10, ConditionFlag::N as u16);
        } else {
            self.load(0x10, ConditionFlag::P as u16);
        }
    }
}

enum ConditionFlag {
    P = 1 << 0,
    Z = 1 << 1,
    N = 1 << 2,
}
