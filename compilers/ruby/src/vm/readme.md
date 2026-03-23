# Ruby 基于寄存器的虚拟机实现

本模块实现了一个基于寄存器的虚拟机，用于执行 Ruby 代码。

## 核心组件

- **指令集**：定义了虚拟机执行的各种指令
- **执行上下文**：管理运行时的变量、方法等信息
- **虚拟机状态**：维护执行过程中的寄存器、栈等状态
- **虚拟机**：提供指令执行和垃圾收集等功能

## 指令集概览

- **加载指令**：LoadConst、LoadLocal、LoadGlobal、LoadInstance、LoadClass
- **存储指令**：StoreLocal、StoreGlobal、StoreInstance、StoreClass
- **算术指令**：Add、Sub、Mul、Div、Mod、Exp
- **比较指令**：Eq、Neq、Lt、Lte、Gt、Gte
- **逻辑指令**：And、Or、Not
- **控制流指令**：Jump、JumpIfFalse、JumpIfTrue
- **方法调用指令**：CallMethod、Return
- **数组和哈希指令**：NewArray、NewHash
- **特殊指令**：Nil、True、False、Move
- **FFI 指令**：LoadLibrary、GetFunction、CallFunction