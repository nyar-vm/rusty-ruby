//! 性能分析器
//!
//! 用于收集和分析代码执行的性能数据，包括执行时间、调用次数等。

use crate::vm::{Context, Instruction};
use std::{collections::HashMap, time::Instant};

/// 性能分析事件
#[derive(Debug, Clone)]
pub enum ProfileEvent {
    /// 函数调用开始
    FunctionStart(String),
    /// 函数调用结束
    FunctionEnd(String),
    /// 指令执行
    InstructionExecuted(usize),
    /// 编译事件
    Compilation(String),
}

/// 性能分析数据
#[derive(Debug, Clone)]
pub struct ProfileData {
    /// 函数调用次数
    pub function_calls: HashMap<String, usize>,
    /// 函数执行时间
    pub function_times: HashMap<String, u128>,
    /// 指令执行次数
    pub instruction_counts: HashMap<usize, usize>,
    /// 编译事件
    pub compilation_events: Vec<String>,
}

/// 性能分析器
///
/// 用于收集和分析代码执行的性能数据。
pub struct Profiler {
    /// 性能分析数据
    data: ProfileData,
    /// 函数调用栈
    call_stack: Vec<String>,
    /// 函数开始时间
    function_start_times: HashMap<String, Instant>,
    /// 是否启用
    enabled: bool,
}

impl Profiler {
    /// 创建新的性能分析器
    pub fn new() -> Self {
        Self {
            data: ProfileData {
                function_calls: HashMap::new(),
                function_times: HashMap::new(),
                instruction_counts: HashMap::new(),
                compilation_events: Vec::new(),
            },
            call_stack: Vec::new(),
            function_start_times: HashMap::new(),
            enabled: false,
        }
    }

    /// 启用性能分析
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// 禁用性能分析
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// 记录函数调用开始
    pub fn record_function_start(&mut self, function_name: &str) {
        if !self.enabled {
            return;
        }

        let function_name_str = function_name.to_string();
        self.call_stack.push(function_name_str.clone());
        self.function_start_times.insert(function_name_str.clone(), Instant::now());
        *self.data.function_calls.entry(function_name_str).or_insert(0) += 1;
    }

    /// 记录函数调用结束
    pub fn record_function_end(&mut self, function_name: &str) {
        if !self.enabled {
            return;
        }

        let function_name_str = function_name.to_string();
        let function_name_clone = function_name_str.clone();
        if let Some(start_time) = self.function_start_times.remove(&function_name_str) {
            let duration = start_time.elapsed().as_nanos();
            *self.data.function_times.entry(function_name_str).or_insert(0) += duration;
        }
        if let Some(last) = self.call_stack.last() {
            if last == &function_name_clone {
                self.call_stack.pop();
            }
        }
    }

    /// 记录指令执行
    pub fn record_instruction(&mut self, instruction_index: usize) {
        if !self.enabled {
            return;
        }

        *self.data.instruction_counts.entry(instruction_index).or_insert(0) += 1;
    }

    /// 记录编译事件
    pub fn record_compilation(&mut self, event: &str) {
        if !self.enabled {
            return;
        }

        self.data.compilation_events.push(event.to_string());
    }

    /// 获取性能分析数据
    pub fn get_data(&self) -> &ProfileData {
        &self.data
    }

    /// 重置性能分析数据
    pub fn reset(&mut self) {
        self.data = ProfileData {
            function_calls: HashMap::new(),
            function_times: HashMap::new(),
            instruction_counts: HashMap::new(),
            compilation_events: Vec::new(),
        };
        self.call_stack.clear();
        self.function_start_times.clear();
    }

    /// 生成性能分析报告
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Performance Profile Report ===\n\n");

        // 函数调用统计
        report.push_str("Function Calls:\n");
        for (function, count) in &self.data.function_calls {
            report.push_str(&format!("  {}: {} calls\n", function, count));
        }
        report.push_str("\n");

        // 函数执行时间
        report.push_str("Function Execution Times (nanoseconds):\n");
        for (function, time) in &self.data.function_times {
            report.push_str(&format!("  {}: {} ns\n", function, time));
        }
        report.push_str("\n");

        // 指令执行次数
        report.push_str("Instruction Execution Counts:\n");
        for (instruction, count) in &self.data.instruction_counts {
            report.push_str(&format!("  Instruction {}: {} executions\n", instruction, count));
        }
        report.push_str("\n");

        // 编译事件
        report.push_str("Compilation Events:\n");
        for event in &self.data.compilation_events {
            report.push_str(&format!("  {}\n", event));
        }
        report.push_str("\n");

        report
    }
}
