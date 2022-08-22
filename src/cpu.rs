pub struct Instruction{
    pub opcode: u8,
    pub arg: u8,
    pub arg2: u8,
}
pub struct Cpu {
    pub pc: usize, //程序计数器
    pub ir: Instruction, //指令寄存器
    pub sp: u64, //栈顶端的地址
    pub mem: [u8; 0x10000], //内存
    pub regs: [u8; 16], //寄存器组
    pub flags: [u8; 4],
    /*
    flags:
    0：为0则计算出是0，为1则计算出大于零，为2则计算出的小于零
    1：为0则上一次正常，为1则溢出，为2则负溢出
    2：TODO
    3：TODO
     */
}
impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0,
            ir: Instruction { opcode: 0, arg: 0, arg2: 0},
            sp: 0,
            mem: [0; 0x10000],
            regs: [0; 16],
            flags: [0; 4],
        }
    }
    //传入一个指令的数组，加载到内存中
    pub fn load_program(self: &mut Cpu,_program: Vec<u8>){
        self.mem[.._program.len()].copy_from_slice(&_program[..]);
    }
    pub fn run(self: &mut Cpu) {
        loop {
            //PC++
            self.pc = self.pc.wrapping_add(4);
            //解析指令
            self.ir = Instruction{
                opcode: self.mem[self.pc],
                arg: self.mem[self.pc+1],
                arg2: self.mem[self.pc+2],
            };
            match self.ir.opcode {
                0x0 => break, //退出程序
                0x1 => { //赋值加法
                    self.regs[self.ir.arg as usize] += self.ir.arg2;
                },
                0x2 => { //加法
                    self.regs[self.ir.arg as usize] += self.regs[self.ir.arg2 as usize];
                },
                0x3 => { //减法
                    self.regs[self.ir.arg as usize] -= self.regs[self.ir.arg2 as usize];
                }
                0x4 => { //乘法
                    self.regs[self.ir.arg as usize] *= self.regs[self.ir.arg2 as usize];                    
                }
                0x5 => { //除法
                    self.regs[self.ir.arg as usize] /= self.regs[self.ir.arg2 as usize];
                }
                _ => {}
            }
        }
    }
}