//! 执行引擎
//!
//! 负责协调解释器、编译器和执行路径的切换，是整个编译架构的核心。

use crate::{
    architecture::{
        baseline_compiler::BaselineCompiler, hotness_detector::HotnessDetector, interpreter::Interpreter,
        optimizing_compiler::OptimizingCompiler,
    },
    vm::{Context, Instruction},
};
use ruby_types::{RubyResult, RubyValue};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// 编译状态
pub enum CompilationState {
    /// 未编译，使用解释器
    Interpreted,
    /// 已基线编译
    BaselineCompiled(Box<dyn Fn(&mut Context, &mut [RubyValue]) -> RubyResult<()>>),
    /// 已优化编译
    OptimizedCompiled(Box<dyn Fn(&mut Context, &mut [RubyValue]) -> RubyResult<()>>),
}

/// 执行引擎
///
/// 协调解释器、编译器和执行路径的切换。
pub struct ExecutionEngine {
    /// 解释器
    interpreter: Interpreter,
    /// 基线编译器
    baseline_compiler: BaselineCompiler,
    /// 优化编译器
    optimizing_compiler: OptimizingCompiler,
    /// 热点检测器
    hotness_detector: Arc<HotnessDetector>,
    /// 编译状态缓存
    compilation_cache: Mutex<HashMap<usize, CompilationState>>,
    /// JIT 缓存
    jit_cache: Mutex<crate::jit::JITCache>,
    /// 编译任务队列
    compilation_queue: Mutex<Vec<usize>>,
}

impl ExecutionEngine {
    /// 创建新的执行引擎
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            baseline_compiler: BaselineCompiler::new(),
            optimizing_compiler: OptimizingCompiler::new(),
            hotness_detector: Arc::new(HotnessDetector::new()),
            compilation_cache: Mutex::new(HashMap::new()),
            jit_cache: Mutex::new(crate::jit::JITCache::new()),
            compilation_queue: Mutex::new(Vec::new()),
        }
    }

    /// 分析指令序列的特性
    ///
    /// # 参数
    /// - `instructions`: 指令序列
    ///
    /// # 返回值
    /// - `CodeFeatures`: 代码特性
    fn analyze_code_features(&self, instructions: &[Instruction]) -> crate::architecture::hotness_detector::CodeFeatures {
        use crate::{architecture::hotness_detector::CodeFeatures, vm::Instruction};

        let mut features = CodeFeatures::default();
        features.total_instructions = instructions.len();

        // 分析指令类型
        for instr in instructions {
            match instr {
                Instruction::Add | Instruction::Sub | Instruction::Mul | Instruction::Div | Instruction::Mod | Instruction::Exp => {
                    features.arithmetic_ops += 1;
                }
                Instruction::Jump(_) | Instruction::JumpIfFalse(_) | Instruction::JumpIfTrue(_) => {
                    features.branch_ops += 1;
                }
                Instruction::CallMethod(_, _) => {
                    features.call_ops += 1;
                }
                _ => {}
            }
        }

        // 检测循环（简单实现：查找可能的循环结构）
        if features.branch_ops > 0 {
            // 检查是否有向后跳转（可能表示循环）
            for (i, instr) in instructions.iter().enumerate() {
                match instr {
                    Instruction::Jump(offset) if *offset < 0 => {
                        features.has_loop = true;
                        break;
                    }
                    Instruction::JumpIfFalse(offset) if *offset < 0 => {
                        features.has_loop = true;
                        break;
                    }
                    Instruction::JumpIfTrue(offset) if *offset < 0 => {
                        features.has_loop = true;
                        break;
                    }
                    _ => {}
                }
            }
        }

        features
    }

    /// 执行指令序列
    ///
    /// # 参数
    /// - `instructions`: 指令序列
    /// - `context`: 执行上下文
    /// - `registers`: 寄存器
    ///
    /// # 返回值
    /// - `RubyResult<()>`: 执行结果
    pub fn execute(&mut self, instructions: &[Instruction], context: &mut Context, registers: &mut [RubyValue]) -> RubyResult<()> {
        // 使用指令序列的内存地址作为唯一标识符
        let code_id = instructions.as_ptr() as *const u8 as usize;

        // 分析代码特性并缓存
        if self.hotness_detector.get_code_features(code_id).is_none() {
            let features = self.analyze_code_features(instructions);
            self.hotness_detector.cache_code_features(code_id, features);
        }

        // 检查编译状态
        let mut cache = self.compilation_cache.lock().unwrap();
        let compilation_state = cache.entry(code_id).or_insert(CompilationState::Interpreted);

        // 执行代码并检查是否需要编译
        let result = match compilation_state {
            CompilationState::Interpreted => {
                // 使用解释器执行
                let result = self.interpreter.execute(instructions, context, registers);

                // 增加执行计数并检查是否需要编译
                let (need_baseline, _) = self.hotness_detector.increment_count(code_id);

                // 获取代码特性
                if let Some(features) = self.hotness_detector.get_code_features(code_id) {
                    // 根据代码特性选择编译策略
                    if need_baseline || (features.has_loop && features.total_instructions > 5) {
                        // 对于有循环的代码或达到阈值的代码，直接进行基线编译
                        match self.baseline_compiler.compile(instructions) {
                            Ok(compiled_func) => {
                                *compilation_state = CompilationState::BaselineCompiled(compiled_func);
                                println!("Baseline compiled code with {} instructions", instructions.len());
                            }
                            Err(err) => {
                                println!("Baseline compilation failed: {:?}", err);
                            }
                        }
                    }

                    // 对于计算密集型代码，直接进行优化编译
                    if features.arithmetic_ops > features.total_instructions / 2 && features.total_instructions > 10 {
                        match self.optimizing_compiler.compile(instructions) {
                            Ok(compiled_func) => {
                                *compilation_state = CompilationState::OptimizedCompiled(compiled_func);
                                println!("Optimized compiled code with {} instructions (computation-intensive)", instructions.len());
                            }
                            Err(err) => {
                                println!("Optimizing compilation failed: {:?}", err);
                            }
                        }
                    }
                }
                else if need_baseline {
                    // 触发基线编译
                    match self.baseline_compiler.compile(instructions) {
                        Ok(compiled_func) => {
                            *compilation_state = CompilationState::BaselineCompiled(compiled_func);
                            println!("Baseline compiled code with {} instructions", instructions.len());
                        }
                        Err(err) => {
                            println!("Baseline compilation failed: {:?}", err);
                        }
                    }
                }
                result
            }
            CompilationState::BaselineCompiled(func) => {
                // 使用基线编译的代码执行
                let result = func(context, registers);

                // 增加执行计数并检查是否需要优化编译
                let (_, need_optimizing) = self.hotness_detector.increment_count(code_id);

                // 获取代码特性
                if let Some(features) = self.hotness_detector.get_code_features(code_id) {
                    // 根据代码特性选择优化策略
                    if need_optimizing
                        || (features.arithmetic_ops > features.total_instructions / 3)
                        || (features.has_loop && features.total_instructions > 10)
                    {
                        // 触发优化编译
                        match self.optimizing_compiler.compile(instructions) {
                            Ok(compiled_func) => {
                                *compilation_state = CompilationState::OptimizedCompiled(compiled_func);
                                println!("Optimized compiled code with {} instructions", instructions.len());
                            }
                            Err(err) => {
                                println!("Optimizing compilation failed: {:?}", err);
                            }
                        }
                    }
                }
                else if need_optimizing {
                    // 触发优化编译
                    match self.optimizing_compiler.compile(instructions) {
                        Ok(compiled_func) => {
                            *compilation_state = CompilationState::OptimizedCompiled(compiled_func);
                            println!("Optimized compiled code with {} instructions", instructions.len());
                        }
                        Err(err) => {
                            println!("Optimizing compilation failed: {:?}", err);
                        }
                    }
                }
                result
            }
            CompilationState::OptimizedCompiled(func) => {
                // 使用优化编译的代码执行
                func(context, registers)
            }
        };

        result
    }

    /// 执行后台编译任务
    ///
    /// 处理编译任务队列中的任务，减少编译对执行的影响
    pub fn process_compilation_queue(&mut self) {
        let mut queue = self.compilation_queue.lock().unwrap();
        let tasks = queue.drain(..).collect::<Vec<_>>();
        drop(queue);

        for code_id in tasks {
            // 处理编译任务
            // 这里可以实现后台编译逻辑
        }
    }

    /// 获取热点检测器
    pub fn hotness_detector(&self) -> Arc<HotnessDetector> {
        self.hotness_detector.clone()
    }
}
