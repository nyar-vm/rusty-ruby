//! 基线编译器
//!
//! 负责将指令序列编译为简单的机器码，编译速度快，适合执行热点代码。

use crate::vm::{Context, Instruction};
use ruby_types::{RubyError, RubyResult, RubyValue};

/// 基线编译器
///
/// 将指令序列编译为简单的机器码，编译速度快，执行速度比解释器快。
pub struct BaselineCompiler {
    // 编译器配置
}

impl BaselineCompiler {
    /// 创建新的基线编译器
    pub fn new() -> Self {
        Self {}
    }

    /// 编译指令序列
    ///
    /// # 参数
    /// - `instructions`: 指令序列
    ///
    /// # 返回值
    /// - `RubyResult<Box<dyn Fn(...)>>`: 编译后的函数
    pub fn compile(&self, instructions: &[Instruction]) -> RubyResult<Box<dyn Fn(&mut Context, &mut [RubyValue]) -> RubyResult<()>>> {
        // 为当前指令序列创建一个优化的执行闭包
        let instructions_copy = instructions.to_vec();

        Ok(Box::new(move |context, registers| {
            let mut pc = 0;
            while pc < instructions_copy.len() {
                let instr = &instructions_copy[pc];
                pc += 1;

                match instr {
                    // 加载指令
                    Instruction::LoadConst(value) => {
                        registers[0] = value.clone();
                    }
                    Instruction::LoadLocal(index) => {
                        if let Some(value) = context.get_local(*index) {
                            registers[0] = value.clone();
                        }
                        else {
                            registers[0] = RubyValue::Nil;
                        }
                    }
                    Instruction::LoadGlobal(name) => {
                        if let Some(value) = context.get_global(name.as_str()) {
                            registers[0] = value.clone();
                        }
                        else {
                            registers[0] = RubyValue::Nil;
                        }
                    }
                    Instruction::LoadInstance(name) => {
                        if let Some(value) = context.get_instance_variable(name.as_str()) {
                            registers[0] = value.clone();
                        }
                        else {
                            registers[0] = RubyValue::Nil;
                        }
                    }
                    Instruction::LoadClass(name) => {
                        if let Some(value) = context.get_class_variable(name.as_str()) {
                            registers[0] = value.clone();
                        }
                        else {
                            registers[0] = RubyValue::Nil;
                        }
                    }

                    // 存储指令
                    Instruction::StoreLocal(index) => {
                        context.set_local(*index, registers[0].clone());
                    }
                    Instruction::StoreGlobal(name) => {
                        context.set_global_string(name.clone(), registers[0].clone());
                    }
                    Instruction::StoreInstance(name) => {
                        context.set_instance_variable(name.as_str(), registers[0].clone());
                    }
                    Instruction::StoreClass(name) => {
                        context.set_class_variable(name.as_str(), registers[0].clone());
                    }

                    // 算术指令
                    Instruction::Add => match (&registers[0], &registers[1]) {
                        (RubyValue::Integer(a), RubyValue::Integer(b)) => {
                            registers[0] = RubyValue::Integer(a + b);
                        }
                        (RubyValue::String(_), RubyValue::String(_)) | (RubyValue::Array(_), RubyValue::Array(_)) => {
                            let result = registers[0].concat(&registers[1]);
                            registers[0] = result;
                        }
                        _ => {
                            let a = registers[0].to_f64();
                            let b = registers[1].to_f64();
                            registers[0] = RubyValue::Float(a + b);
                        }
                    },
                    Instruction::Sub => match (&registers[0], &registers[1]) {
                        (RubyValue::Integer(a), RubyValue::Integer(b)) => {
                            registers[0] = RubyValue::Integer(a - b);
                        }
                        _ => {
                            let a = registers[0].to_f64();
                            let b = registers[1].to_f64();
                            registers[0] = RubyValue::Float(a - b);
                        }
                    },
                    Instruction::Mul => match (&registers[0], &registers[1]) {
                        (RubyValue::Integer(a), RubyValue::Integer(b)) => {
                            registers[0] = RubyValue::Integer(a * b);
                        }
                        _ => {
                            let a = registers[0].to_f64();
                            let b = registers[1].to_f64();
                            registers[0] = RubyValue::Float(a * b);
                        }
                    },
                    Instruction::Div => {
                        let a = registers[0].to_f64();
                        let b = registers[1].to_f64();
                        if b == 0.0 {
                            return Err(RubyError::RuntimeError("Division by zero".to_string()));
                        }
                        registers[0] = RubyValue::Float(a / b);
                    }
                    Instruction::Mod => match (&registers[0], &registers[1]) {
                        (RubyValue::Integer(a), RubyValue::Integer(b)) => {
                            if *b == 0 {
                                return Err(RubyError::RuntimeError("Division by zero".to_string()));
                            }
                            registers[0] = RubyValue::Integer(a % b);
                        }
                        _ => {
                            let a = registers[0].to_f64();
                            let b = registers[1].to_f64();
                            if b == 0.0 {
                                return Err(RubyError::RuntimeError("Division by zero".to_string()));
                            }
                            registers[0] = RubyValue::Float(a % b);
                        }
                    },
                    Instruction::Exp => {
                        let a = registers[0].to_f64();
                        let b = registers[1].to_f64();
                        registers[0] = RubyValue::Float(a.powf(b));
                    }

                    // 比较指令
                    Instruction::Eq => {
                        let a = &registers[0];
                        let b = &registers[1];
                        registers[0] = RubyValue::Boolean(a == b);
                    }
                    Instruction::Neq => {
                        let a = &registers[0];
                        let b = &registers[1];
                        registers[0] = RubyValue::Boolean(a != b);
                    }
                    Instruction::Lt => {
                        let a = registers[0].to_f64();
                        let b = registers[1].to_f64();
                        registers[0] = RubyValue::Boolean(a < b);
                    }
                    Instruction::Lte => {
                        let a = registers[0].to_f64();
                        let b = registers[1].to_f64();
                        registers[0] = RubyValue::Boolean(a <= b);
                    }
                    Instruction::Gt => {
                        let a = registers[0].to_f64();
                        let b = registers[1].to_f64();
                        registers[0] = RubyValue::Boolean(a > b);
                    }
                    Instruction::Gte => {
                        let a = registers[0].to_f64();
                        let b = registers[1].to_f64();
                        registers[0] = RubyValue::Boolean(a >= b);
                    }

                    // 逻辑指令
                    Instruction::And => {
                        let a = registers[0].to_bool();
                        let b = registers[1].to_bool();
                        registers[0] = RubyValue::Boolean(a && b);
                    }
                    Instruction::Or => {
                        let a = registers[0].to_bool();
                        let b = registers[1].to_bool();
                        registers[0] = RubyValue::Boolean(a || b);
                    }
                    Instruction::Not => {
                        let a = registers[0].to_bool();
                        registers[0] = RubyValue::Boolean(!a);
                    }

                    // 控制流指令
                    Instruction::Jump(offset) => {
                        pc = (pc as i32 + offset - 1) as usize;
                    }
                    Instruction::JumpIfFalse(offset) => {
                        if !registers[0].to_bool() {
                            pc = (pc as i32 + offset - 1) as usize;
                        }
                    }
                    Instruction::JumpIfTrue(offset) => {
                        if registers[0].to_bool() {
                            pc = (pc as i32 + offset - 1) as usize;
                        }
                    }

                    // 方法调用指令
                    Instruction::CallMethod(name, arg_count) => {
                        let mut args = Vec::new();
                        for i in 0..*arg_count {
                            args.push(registers[i + 1].clone());
                        }
                        let result = context.call_method(name, args)?;
                        registers[0] = result;
                    }
                    Instruction::Return => {
                        return Ok(());
                    }

                    // 数组和哈希指令
                    Instruction::NewArray(size) => {
                        let mut array = Vec::new();
                        for i in 0..*size {
                            array.push(registers[i + 1].clone());
                        }
                        registers[0] = RubyValue::Array(array);
                    }
                    Instruction::NewHash(size) => {
                        let mut hash = std::collections::HashMap::new();
                        for i in 0..*size {
                            let key = registers[i * 2 + 1].to_string();
                            let value = registers[i * 2 + 2].clone();
                            hash.insert(key, value);
                        }
                        registers[0] = RubyValue::Hash(hash);
                    }

                    // 特殊指令
                    Instruction::Nil => {
                        registers[0] = RubyValue::Nil;
                    }
                    Instruction::True => {
                        registers[0] = RubyValue::Boolean(true);
                    }
                    Instruction::False => {
                        registers[0] = RubyValue::Boolean(false);
                    }
                    Instruction::Move(src, dst) => {
                        if *src < registers.len() && *dst < registers.len() {
                            registers[*dst] = registers[*src].clone();
                        }
                    }

                    // FFI 指令
                    Instruction::LoadLibrary(_path) => {
                        // 简化实现
                        registers[0] = RubyValue::Boolean(false);
                    }
                    Instruction::GetFunction(_lib_name, _func_name) => {
                        // 简化实现
                        registers[0] = RubyValue::Nil;
                    }
                    Instruction::CallFunction(_arg_count) => {
                        // 简化实现
                        registers[0] = RubyValue::Nil;
                    }
                    Instruction::Try => {
                        // 简化实现
                    }
                    Instruction::Catch(_offset) => {
                        // 简化实现
                    }
                    Instruction::Throw => {
                        // 简化实现
                    }
                }
            }

            Ok(())
        }))
    }
}
