pub struct CPU {
    pub mem: [u8; 0x10000],
    pub regs: [u8; 16],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            mem: [0; 0x10000],
            regs: [0; 16],
        }
    }    
}