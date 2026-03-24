//! 调试器
//!
//! 用于调试Ruby代码的执行过程，包括断点设置、单步执行等功能。

use crate::vm::{Context, Instruction};
use ruby_types::RubyValue;
use std::collections::HashSet;

/// 断点类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Breakpoint {
    /// 行号断点
    Line(usize),
    /// 函数断点
    Function(String),
    /// 指令断点
    Instruction(usize),
}

/// 调试器状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebuggerState {
    /// 运行中
    Running,
    /// 暂停
    Paused,
    /// 已停止
    Stopped,
}

/// 调试器
///
/// 用于调试Ruby代码的执行过程。
pub struct Debugger {
    /// 断点集合
    breakpoints: HashSet<Breakpoint>,
    /// 调试器状态
    state: DebuggerState,
    /// 上下文引用
    context: Option<*mut Context>,
}

impl Debugger {
    /// 创建新的调试器
    pub fn new() -> Self {
        Self { breakpoints: HashSet::new(), state: DebuggerState::Stopped, context: None }
    }

    /// 附加到上下文
    pub fn attach(&mut self, context: &mut Context) {
        self.context = Some(context as *mut Context);
        self.state = DebuggerState::Running;
    }

    /// 分离
    pub fn detach(&mut self) {
        self.context = None;
        self.state = DebuggerState::Stopped;
    }

    /// 添加断点
    pub fn add_breakpoint(&mut self, breakpoint: Breakpoint) {
        self.breakpoints.insert(breakpoint);
    }

    /// 移除断点
    pub fn remove_breakpoint(&mut self, breakpoint: &Breakpoint) {
        self.breakpoints.remove(breakpoint);
    }

    /// 清除所有断点
    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
    }

    /// 检查是否需要中断
    pub fn should_break(&self, line: usize, function: &str, instruction_index: usize) -> bool {
        if !self.is_attached() {
            return false;
        }

        // 检查行号断点
        if self.breakpoints.contains(&Breakpoint::Line(line)) {
            return true;
        }

        // 检查函数断点
        if self.breakpoints.contains(&Breakpoint::Function(function.to_string())) {
            return true;
        }

        // 检查指令断点
        if self.breakpoints.contains(&Breakpoint::Instruction(instruction_index)) {
            return true;
        }

        false
    }

    /// 暂停执行
    pub fn pause(&mut self) {
        self.state = DebuggerState::Paused;
    }

    /// 继续执行
    pub fn resume(&mut self) {
        self.state = DebuggerState::Running;
    }

    /// 单步执行
    pub fn step(&mut self) -> Option<Instruction> {
        if !self.is_attached() {
            return None;
        }

        // 暂时返回None，因为Context结构体中没有instructions和pc字段
        // 后续需要修改Context结构体，添加这些字段以支持调试功能
        None
    }

    /// 获取当前状态
    pub fn get_state(&self) -> DebuggerState {
        self.state.clone()
    }

    /// 检查是否已附加
    pub fn is_attached(&self) -> bool {
        self.context.is_some() && self.state != DebuggerState::Stopped
    }

    /// 获取变量值
    pub fn get_variable(&self, _name: &str) -> Option<RubyValue> {
        if !self.is_attached() {
            return None;
        }

        // 暂时返回None，因为Context结构体中的locals字段是私有的，而且使用usize作为键
        // 后续需要修改Context结构体，添加相应的方法来支持变量访问
        None
    }

    /// 设置变量值
    pub fn set_variable(&mut self, _name: &str, _value: RubyValue) -> bool {
        if !self.is_attached() {
            return false;
        }

        // 暂时返回false，因为Context结构体中的locals字段是私有的，而且使用usize作为键
        // 后续需要修改Context结构体，添加相应的方法来支持变量设置
        false
    }

    /// 获取调用栈
    pub fn get_call_stack(&self) -> Vec<String> {
        if !self.is_attached() {
            return Vec::new();
        }

        // 暂时返回空向量，因为Context结构体中没有call_stack字段
        // 后续需要修改Context结构体，添加call_stack字段以支持调试功能
        Vec::new()
    }
}
