//! 执行引擎
//! 
//! 负责协调解释器、编译器和执行路径的切换，是整个编译架构的核心。

use crate::architecture::{interpreter::Interpreter, baseline_compiler::BaselineCompiler, optimizing_compiler::OptimizingCompiler, hotness_detector::HotnessDetector};
use crate::vm::{Context, Instruction};
use ruby_types::{RubyResult, RubyValue};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

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
        }
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
                if need_baseline {
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
                if need_optimizing {
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

    /// 获取热点检测器
    pub fn hotness_detector(&self) -> Arc<HotnessDetector> {
        self.hotness_detector.clone()
    }
}