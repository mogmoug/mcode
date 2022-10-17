# MCODE
MCODE是一个可以用来学习的编程语言  
它有一个模拟的CPU，能够让你时刻查看它的状态  
MCODE尚未完成开发，您可以参与进来，也可以fork自己的版本  
![](img/mogxin.png) ![](img/xinry.png)

## TODO List
1.  图灵完备 [ ]
2.  小屏幕 [ ]

## LICENSE
Copyright 2022 Mogmoug

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

## 理念
创建一个模拟CPU，用于各种测试  

## 指令
操作码（一个字节）与两个参数字节的结合体  
为：opcode: u8;arg: u8;arg1: u8  

## CPU
有：程序计数器，指令寄存器，内存栈区域，内存，寄存器组，标志位  
实现：程序的执行，各种指令，四则运算，调试CPU，加载程序，图灵完备  

### 程序计数器
记录当前执行指令在内存中的地址  
每次执行完后通常增加3，因为一个指令有三个字节  
usize类型，32位系统为32位二进制正整数，64位系统为64位二进制正整数  
♪(´▽｀)（。＾▽＾）

### 指令寄存器
在解析指令时，将解析结果暂存到指令寄存器中  
是指令类型，同样有三个字节  
在执行指令时提高速度

### 内存
内存（RAM，随机存储器），是一种存储器。在本程序中是一个u8数组，长度为65535。在重启程序后内存里的数据会丢失。

### 内存栈区域
内存栈区域是从内存256地址到栈顶地址的一片内存区。  
每进行一次call都会增加一次栈顶（stack_top），每进行一次ret就减少一次栈顶  

### 寄存器组
寄存器组是一些小型存储器的集合，有利于CPU的运行
可以存储一些暂时的数据，很多指令都需要用到它

### 标志位
标志位可以记录一些CPU的状态，有一些指令需要用到这些状态，
以便进行更加复杂的操作
flags:
0：为0则计算出是0，为1则计算出大于零，为2则计算出的小于零
1：为0则上一次正常，为1则溢出，为2则负溢出
2：TODO
3：TODO