/*
Copyright 2022 Mogmoug

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"),
 to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
  and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
  WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
mod cpu;
use cpu::Cpu;

fn main() {
    let mut _cpu = Cpu::new();
    //就输出了一个hello，也太长了吧
    _cpu.load_program(vec![
        1, 1, 'h' as u8, //写入h
        8, 1, 0, //输出
        1, 2, 'e' as u8, //写入e
        8, 2, 0, //输出
        1, 3, 'l' as u8, //写入l
        8, 3, 0, //输出
        1, 4, 'l' as u8, //写入l
        8, 4, 0, //输出
        1, 5, 'o' as u8, //写入o
        8, 5, 0, //输出
    ]);
    _cpu.run();
    if 1 == 0 {
        {
            _cpu.print_debug_info();
        };
    }
}
