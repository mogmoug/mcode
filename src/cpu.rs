/*
Copyright 2022 Mogmoug

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
#[derive(Debug)]
pub struct Instruction{
    pub opcode: u8,
    pub arg: u8,
    pub arg2: u8,
}
pub struct Cpu {
    pub pc: usize, //程序计数器
    pub ir: Instruction, //指令寄存器
    pub sp: u64, //栈顶端的地址
    pub mem: [u8; 256], //内存
    pub regs: [u8; 256], //寄存器组
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
            mem: [0; 256],
            regs: [0; 256],
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
            self.pc = self.pc.wrapping_add(3);
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
                0x6 => { //条件跳转
                    if self.flags[0] == 0 {
                        self.pc = (self.ir.arg <<8 + self.ir.arg2) as usize;
                    }
                }
                0x7 => { //直接跳转
                    self.pc = (self.ir.arg <<8 + self.ir.arg2) as usize;
                }
                0x8 => { //输出
                    print!("{}",(self.regs[self.ir.arg as usize] as char));
                }
                _ => {
                    println!("unknown instruction:{:#?}",self.ir);
                }
            }
        }
    }
    pub fn print_debug_info(self: &mut Cpu){
        println!("PC:{} ",self.pc);
        println!("instruction reg:{:#?}",self.ir);
        println!("SP:{}",self.sp);
        println!("mem:{:?}",self.mem);
        println!("regs:{:?}",self.regs);
        println!("flags:{:?}",self.flags);
    }
}