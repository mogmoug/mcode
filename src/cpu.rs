pub struct CPU {
    pub pc: u64,
    pub ir: u32,
    pub sp: u64,
    pub mem: [u8; 0x10000],
    pub regs: [u8; 16],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            ir: 0,
            sp: 0,
            mem: [0; 0x10000],
            regs: [0; 16],
        }
    }
    pub fn load_program(self: CPU,_program: &u32){
        // TODO 加载程序的代码
    }
    pub fn run(self: CPU) {
        unsafe {
            // TODO 运行程序的代码
        }
    }
}