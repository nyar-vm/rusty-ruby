#![doc = include_str!("readme.md")]

use crate::{
    gc::{GC, GCType, create_gc},
    jit::{JITCache, JITCompiler, OptimizedJIT},
    profiler::{Debugger, Profiler, Visualizer},
};
use libloading::Library;
use ruby_types::{RubyError, RubyResult, RubyValue};
use std::sync::{Arc, Mutex};

/// VM 指令集
///
/// 定义了虚拟机执行的各种指令，包括加载/存储、算术、比较、逻辑、控制流、方法调用等。
#[derive(Debug, Clone)]
pub enum Instruction {
    // 加载指令
    /// 加载常量到寄存器
    ///
    /// 将指定的常量值加载到寄存器 0 中
    LoadConst(RubyValue),
    /// 加载局部变量到寄存器
    ///
    /// 将指定索引的局部变量加载到寄存器 0 中
    LoadLocal(usize),
    /// 加载全局变量到寄存器
    ///
    /// 将指定名称的全局变量加载到寄存器 0 中
    LoadGlobal(String),
    /// 加载实例变量到寄存器
    ///
    /// 将指定名称的实例变量加载到寄存器 0 中
    LoadInstance(String),
    /// 加载类变量到寄存器
    ///
    /// 将指定名称的类变量加载到寄存器 0 中
    LoadClass(String),

    // 存储指令
    /// 存储寄存器值到局部变量
    ///
    /// 将寄存器 0 的值存储到指定索引的局部变量
    StoreLocal(usize),
    /// 存储寄存器值到全局变量
    ///
    /// 将寄存器 0 的值存储到指定名称的全局变量
    StoreGlobal(String),
    /// 存储寄存器值到实例变量
    ///
    /// 将寄存器 0 的值存储到指定名称的实例变量
    StoreInstance(String),
    /// 存储寄存器值到类变量
    ///
    /// 将寄存器 0 的值存储到指定名称的类变量
    StoreClass(String),

    // 算术指令
    /// 加法
    ///
    /// 执行寄存器 0 和寄存器 1 的加法操作，结果存储在寄存器 0 中
    Add,
    /// 减法
    ///
    /// 执行寄存器 0 和寄存器 1 的减法操作，结果存储在寄存器 0 中
    Sub,
    /// 乘法
    ///
    /// 执行寄存器 0 和寄存器 1 的乘法操作，结果存储在寄存器 0 中
    Mul,
    /// 除法
    ///
    /// 执行寄存器 0 和寄存器 1 的除法操作，结果存储在寄存器 0 中
    Div,
    /// 取模
    ///
    /// 执行寄存器 0 和寄存器 1 的取模操作，结果存储在寄存器 0 中
    Mod,
    /// 幂运算
    ///
    /// 执行寄存器 0 的寄存器 1 次幂操作，结果存储在寄存器 0 中
    Exp,

    // 比较指令
    /// 等于
    ///
    /// 比较寄存器 0 和寄存器 1 的值是否相等，结果存储在寄存器 0 中
    Eq,
    /// 不等于
    ///
    /// 比较寄存器 0 和寄存器 1 的值是否不相等，结果存储在寄存器 0 中
    Neq,
    /// 小于
    ///
    /// 比较寄存器 0 的值是否小于寄存器 1 的值，结果存储在寄存器 0 中
    Lt,
    /// 小于等于
    ///
    /// 比较寄存器 0 的值是否小于等于寄存器 1 的值，结果存储在寄存器 0 中
    Lte,
    /// 大于
    ///
    /// 比较寄存器 0 的值是否大于寄存器 1 的值，结果存储在寄存器 0 中
    Gt,
    /// 大于等于
    ///
    /// 比较寄存器 0 的值是否大于等于寄存器 1 的值，结果存储在寄存器 0 中
    Gte,

    // 逻辑指令
    /// 逻辑与
    ///
    /// 执行寄存器 0 和寄存器 1 的逻辑与操作，结果存储在寄存器 0 中
    And,
    /// 逻辑或
    ///
    /// 执行寄存器 0 和寄存器 1 的逻辑或操作，结果存储在寄存器 0 中
    Or,
    /// 逻辑非
    ///
    /// 执行寄存器 0 的逻辑非操作，结果存储在寄存器 0 中
    Not,

    // 控制流指令
    /// 无条件跳转
    ///
    /// 无条件跳转到指定偏移量的指令
    Jump(i32),
    /// 条件跳转（如果为假）
    ///
    /// 如果寄存器 0 的值为假，则跳转到指定偏移量的指令
    JumpIfFalse(i32),
    /// 条件跳转（如果为真）
    ///
    /// 如果寄存器 0 的值为真，则跳转到指定偏移量的指令
    JumpIfTrue(i32),

    // 方法调用指令
    /// 调用方法
    ///
    /// 调用指定名称的方法，参数个数为指定值
    CallMethod(String, usize),
    /// 返回
    ///
    /// 从当前方法返回
    Return,

    // 数组和哈希指令
    /// 创建新数组
    ///
    /// 创建一个包含指定个数元素的新数组
    NewArray(usize),
    /// 创建新哈希
    ///
    /// 创建一个包含指定个数键值对的新哈希
    NewHash(usize),

    // 特殊指令
    /// 加载 nil
    ///
    /// 将 nil 加载到寄存器 0 中
    Nil,
    /// 加载 true
    ///
    /// 将 true 加载到寄存器 0 中
    True,
    /// 加载 false
    ///
    /// 将 false 加载到寄存器 0 中
    False,
    /// 移动寄存器值
    ///
    /// 将源寄存器的值移动到目标寄存器
    Move(usize, usize),

    // FFI 指令
    /// 加载动态库
    ///
    /// 加载指定路径的动态库
    LoadLibrary(String),
    /// 获取 C 函数
    ///
    /// 从指定的动态库中获取指定名称的 C 函数
    GetFunction(String, String),
    /// 调用 C 函数
    ///
    /// 调用指定的 C 函数，参数个数为指定值
    CallFunction(usize),

    // 异常处理指令
    /// 开始异常处理块
    ///
    /// 记录当前 PC 作为异常处理的起始点
    Try,
    /// 捕获异常
    ///
    /// 跳转到指定偏移量的指令来处理异常
    Catch(i32),
    /// 抛出异常
    ///
    /// 抛出当前寄存器 0 中的值作为异常
    Throw,
}

/// 闭包类型
///
/// 表示一个 Ruby 闭包，包含捕获的变量和执行逻辑
#[derive(Clone)]
pub struct Closure {
    /// 捕获的变量
    captured_vars: std::collections::HashMap<String, RubyValue>,
    /// 闭包执行逻辑
    func: Arc<Box<dyn Fn(&mut Context, Vec<RubyValue>) -> RubyResult<RubyValue>>>,
}

impl Closure {
    /// 创建新的闭包
    pub fn new(
        captured_vars: std::collections::HashMap<String, RubyValue>,
        func: Box<dyn Fn(&mut Context, Vec<RubyValue>) -> RubyResult<RubyValue>>,
    ) -> Self {
        Self { captured_vars, func: Arc::new(func) }
    }

    /// 执行闭包
    pub fn call(&self, context: &mut Context, args: Vec<RubyValue>) -> RubyResult<RubyValue> {
        // 保存当前上下文的变量
        let mut saved_vars = std::collections::HashMap::new();
        for (name, value) in &self.captured_vars {
            if let Some(existing) = context.globals.get(name) {
                saved_vars.insert(name.clone(), existing.clone());
            }
            context.set_global(name, value.clone());
        }

        // 执行闭包
        let result = (self.func)(context, args);

        // 恢复原来的变量
        let saved_vars_keys: Vec<String> = saved_vars.keys().cloned().collect();
        for (name, value) in saved_vars {
            context.set_global(name.as_str(), value);
        }
        for name in self.captured_vars.keys() {
            if !saved_vars_keys.contains(name) {
                context.globals.remove(name);
            }
        }

        result
    }
}

/// 执行上下文，用于存储运行时的变量、方法等信息
///
/// 管理运行时的各种状态，包括变量、方法、类和 FFI 相关资源。
pub struct Context {
    /// 全局变量
    ///
    /// 存储全局作用域的变量
    pub globals: std::collections::HashMap<String, RubyValue>,
    /// 局部变量
    ///
    /// 存储局部作用域的变量，使用索引作为键
    locals: std::collections::HashMap<usize, RubyValue>,
    /// 实例变量
    ///
    /// 存储实例作用域的变量
    instance_variables: std::collections::HashMap<String, RubyValue>,
    /// 类变量
    ///
    /// 存储类作用域的变量
    class_variables: std::collections::HashMap<String, RubyValue>,
    /// 方法定义
    ///
    /// 存储全局方法的定义
    methods: std::collections::HashMap<String, std::sync::Arc<Box<dyn Fn(&mut Context, Vec<RubyValue>) -> RubyResult<RubyValue>>>>,
    /// 类定义
    ///
    /// 存储类的定义及其方法
    classes: std::collections::HashMap<
        String,
        (Option<String>, std::collections::HashMap<String, std::sync::Arc<Box<dyn Fn(&mut Context, Vec<RubyValue>) -> RubyResult<RubyValue>>>>),
    >,

    /// FFI 相关
    ///
    /// 存储加载的动态库
    libraries: std::collections::HashMap<String, libloading::Library>,
    /// 存储获取的 C 函数
    functions: std::collections::HashMap<String, libloading::Symbol<'static, unsafe extern "C" fn() -> i32>>,
    /// 存储闭包
    closures: std::collections::HashMap<usize, Closure>,
    /// 下一个闭包 ID
    next_closure_id: usize,

    /// 性能分析器
    ///
    /// 用于收集和分析代码执行的性能数据
    profiler: Profiler,
    /// 调试器
    ///
    /// 用于调试Ruby代码的执行过程
    debugger: Debugger,
    /// 可视化工具
    ///
    /// 用于可视化编译和优化过程
    visualizer: Visualizer,
}

impl Context {
    /// 创建新的执行上下文
    pub fn new() -> Self {
        let mut context = Self {
            globals: std::collections::HashMap::new(),
            locals: std::collections::HashMap::new(),
            instance_variables: std::collections::HashMap::new(),
            class_variables: std::collections::HashMap::new(),
            methods: std::collections::HashMap::new(),
            classes: std::collections::HashMap::new(),
            libraries: std::collections::HashMap::new(),
            functions: std::collections::HashMap::new(),
            closures: std::collections::HashMap::new(),
            next_closure_id: 0,
            profiler: Profiler::new(),
            debugger: Debugger::new(),
            visualizer: Visualizer::new(),
        };

        // 添加内置方法
        context.define_builtin_methods();

        context
    }

    /// 定义内置方法
    fn define_builtin_methods(&mut self) {
        // 数组方法
        self.define_method(
            "length",
            Box::new(|_, args| {
                if let Some(array) = args.get(0) {
                    if let Some(length) = array.array_length() {
                        Ok(RubyValue::Integer(length as i32))
                    }
                    else {
                        Err(RubyError::TypeError("Expected array".to_string()))
                    }
                }
                else {
                    Err(RubyError::ArgumentError("Expected array argument".to_string()))
                }
            }),
        );

        // 字符串方法
        self.define_method(
            "length",
            Box::new(|_, args| {
                if let Some(string) = args.get(0) {
                    match string {
                        RubyValue::String(s) => Ok(RubyValue::Integer(s.len() as i32)),
                        _ => Err(RubyError::TypeError("Expected string".to_string())),
                    }
                }
                else {
                    Err(RubyError::ArgumentError("Expected string argument".to_string()))
                }
            }),
        );

        // 哈希方法
        self.define_method(
            "length",
            Box::new(|_, args| {
                if let Some(hash) = args.get(0) {
                    match hash {
                        RubyValue::Hash(map) => Ok(RubyValue::Integer(map.len() as i32)),
                        _ => Err(RubyError::TypeError("Expected hash".to_string())),
                    }
                }
                else {
                    Err(RubyError::ArgumentError("Expected hash argument".to_string()))
                }
            }),
        );
    }

    /// 获取全局变量
    pub fn get_global(&self, name: &str) -> Option<&RubyValue> {
        self.globals.get(name)
    }

    /// 设置全局变量
    pub fn set_global(&mut self, name: &str, value: RubyValue) {
        self.globals.insert(name.to_string(), value);
    }

    /// 设置全局变量（使用 String 类型的名称）
    pub fn set_global_string(&mut self, name: String, value: RubyValue) {
        self.globals.insert(name, value);
    }

    /// 获取局部变量
    pub fn get_local(&self, index: usize) -> Option<&RubyValue> {
        self.locals.get(&index)
    }

    /// 设置局部变量
    pub fn set_local(&mut self, index: usize, value: RubyValue) {
        self.locals.insert(index, value);
    }

    /// 获取实例变量
    pub fn get_instance_variable(&self, name: &str) -> Option<&RubyValue> {
        self.instance_variables.get(name)
    }

    /// 设置实例变量
    pub fn set_instance_variable(&mut self, name: &str, value: RubyValue) {
        self.instance_variables.insert(name.to_string(), value);
    }

    /// 获取类变量
    pub fn get_class_variable(&self, name: &str) -> Option<&RubyValue> {
        self.class_variables.get(name)
    }

    /// 设置类变量
    pub fn set_class_variable(&mut self, name: &str, value: RubyValue) {
        self.class_variables.insert(name.to_string(), value);
    }

    /// 定义方法
    pub fn define_method(&mut self, name: &str, func: Box<dyn Fn(&mut Context, Vec<RubyValue>) -> RubyResult<RubyValue>>) {
        self.methods.insert(name.to_string(), std::sync::Arc::new(func));
    }

    /// 调用方法
    pub fn call_method(&mut self, name: &str, args: Vec<RubyValue>) -> RubyResult<RubyValue> {
        // 记录函数调用开始
        self.profiler.record_function_start(name);

        // Create a scope to limit the immutable borrow
        let method = { self.methods.get(name).cloned() };

        let result = match method {
            Some(m) => m(self, args),
            None => Err(RubyError::MethodNotFound(name.to_string())),
        };

        // 记录函数调用结束
        self.profiler.record_function_end(name);

        result
    }

    /// 调用对象方法
    pub fn call_object_method(&mut self, obj: &RubyValue, method_name: &str, args: Vec<RubyValue>) -> RubyResult<RubyValue> {
        match obj {
            RubyValue::Object(class_name, _) => {
                // 查找类方法
                if let Some(method) = self.find_method(class_name, method_name) {
                    // 克隆 Arc 来避免借用冲突
                    let method = std::sync::Arc::clone(method);
                    method(self, args)
                }
                else {
                    Err(RubyError::MethodNotFound(format!("{}.{}", class_name, method_name)))
                }
            }
            _ => {
                // 对于基本类型，使用全局方法
                self.call_method(method_name, args)
            }
        }
    }

    /// 创建对象实例
    pub fn new_object(&mut self, class_name: &str) -> RubyResult<RubyValue> {
        if self.classes.contains_key(class_name) {
            Ok(RubyValue::Object(class_name.to_string(), std::collections::HashMap::new()))
        }
        else {
            Err(RubyError::ClassNotFound(class_name.to_string()))
        }
    }

    /// 定义类
    pub fn define_class(&mut self, name: &str) {
        self.classes.insert(name.to_string(), (None, std::collections::HashMap::new()));
    }

    /// 获取性能分析器
    pub fn profiler(&mut self) -> &mut Profiler {
        &mut self.profiler
    }

    /// 获取调试器
    pub fn debugger(&mut self) -> &mut Debugger {
        &mut self.debugger
    }

    /// 获取可视化工具
    pub fn visualizer(&mut self) -> &mut Visualizer {
        &mut self.visualizer
    }

    /// 启用性能分析
    pub fn enable_profiling(&mut self) {
        self.profiler.enable();
    }

    /// 禁用性能分析
    pub fn disable_profiling(&mut self) {
        self.profiler.disable();
    }

    /// 生成性能分析报告
    pub fn generate_profile_report(&self) -> String {
        self.profiler.generate_report()
    }

    /// 生成可视化报告
    pub fn generate_visualization_report(&self) -> String {
        self.visualizer.generate_full_report()
    }

    /// 定义继承自其他类的类
    pub fn define_class_with_super(&mut self, name: &str, super_class: &str) {
        self.classes.insert(name.to_string(), (Some(super_class.to_string()), std::collections::HashMap::new()));
    }

    /// 获取类的方法集合
    pub fn get_class_methods(&mut self, name: &str) -> Result<RubyValue, RubyError> {
        // self.classes.get_mut(name).map(|(_, methods)| methods)
        Err(RubyError::RuntimeError("unimplement".to_string()))
    }

    /// 查找方法，支持继承
    pub fn find_method(
        &self,
        class_name: &str,
        method_name: &str,
    ) -> Option<&std::sync::Arc<Box<dyn Fn(&mut Context, Vec<RubyValue>) -> RubyResult<RubyValue>>>> {
        // 首先在当前类中查找
        if let Some((_, methods)) = self.classes.get(class_name) {
            if let Some(method) = methods.get(method_name) {
                return Some(method);
            }
        }

        // 如果当前类中没有，查找父类
        if let Some((Some(super_class), _)) = self.classes.get(class_name) {
            return self.find_method(super_class, method_name);
        }

        // 如果所有父类都没有，返回 None
        None
    }

    /// 动态定义方法
    pub fn define_dynamic_method(
        &mut self,
        class_name: &str,
        method_name: &str,
        func: Box<dyn Fn(&mut Context, Vec<RubyValue>) -> RubyResult<RubyValue>>,
    ) {
        todo!()
    }

    /// 动态调用方法
    pub fn send(&mut self, obj: &RubyValue, method_name: &str, args: Vec<RubyValue>) -> RubyResult<RubyValue> {
        self.call_object_method(obj, method_name, args)
    }

    /// FFI 相关方法
    pub fn add_library(&mut self, path: &str, lib: Library) {
        self.libraries.insert(path.to_string(), lib);
    }

    pub fn get_library(&self, name: &str) -> Option<&Library> {
        self.libraries.get(name)
    }

    pub fn add_function(&mut self, key: &str, func: libloading::Symbol<'static, unsafe extern "C" fn() -> i32>) {
        self.functions.insert(key.to_string(), func);
    }

    pub fn get_function(&self, key: &str) -> Option<&libloading::Symbol<'static, unsafe extern "C" fn() -> i32>> {
        self.functions.get(key)
    }

    /// 创建闭包
    pub fn create_closure(
        &mut self,
        captured_vars: std::collections::HashMap<String, RubyValue>,
        func: Box<dyn Fn(&mut Context, Vec<RubyValue>) -> RubyResult<RubyValue>>,
    ) -> RubyValue {
        let closure_id = self.next_closure_id;
        self.next_closure_id += 1;

        let closure = Closure::new(captured_vars, func);
        self.closures.insert(closure_id, closure);

        RubyValue::Closure(closure_id)
    }

    /// 执行闭包
    pub fn call_closure(&mut self, closure: &RubyValue, args: Vec<RubyValue>) -> RubyResult<RubyValue> {
        match closure {
            RubyValue::Closure(closure_id) => {
                // 首先检查闭包是否存在
                if let Some(closure) = self.closures.get(closure_id) {
                    // 创建闭包的副本
                    let closure = closure.clone();
                    // 调用闭包
                    closure.call(self, args)
                }
                else {
                    Err(RubyError::RuntimeError("Closure not found".to_string()))
                }
            }
            _ => Err(RubyError::TypeError("Expected closure".to_string())),
        }
    }
}

/// 虚拟机状态
struct VMState {
    /// 指令指针
    pc: usize,
    /// 寄存器
    registers: Vec<RubyValue>,
    /// 栈
    stack: Vec<RubyValue>,
    /// 执行上下文
    context: Arc<Mutex<Context>>,
    /// 垃圾收集器
    gc: Arc<Mutex<dyn GC>>,
    /// JIT编译器
    jit_compiler: Box<dyn JITCompiler>,
    /// JIT缓存
    jit_cache: JITCache,
    /// 异常处理栈
    exception_stack: Vec<usize>,
}

impl VMState {
    /// 创建新的虚拟机状态
    fn new(context: Arc<Mutex<Context>>, gc: Arc<Mutex<dyn GC>>) -> Self {
        Self {
            pc: 0,
            registers: vec![RubyValue::Nil; 16], // 16个通用寄存器
            stack: Vec::new(),
            context,
            gc,
            jit_compiler: Box::new(OptimizedJIT::new()),
            jit_cache: JITCache::new(),
            exception_stack: Vec::new(),
        }
    }

    /// 执行指令
    fn execute(&mut self, instructions: &[Instruction]) -> RubyResult<()> {
        // 为指令序列生成唯一键（使用指令序列的内存地址作为简单实现）
        let instructions_key = instructions.as_ptr() as *const u8 as usize;

        // 检查是否已编译
        if self.jit_cache.is_compiled(instructions_key) {
            // 使用JIT编译后的函数执行
            if let Some(compiled_func) = self.jit_cache.get_compiled(instructions_key) {
                let mut context = self.context.lock().unwrap();
                return compiled_func(&mut context, &mut self.registers);
            }
        }

        // 增加执行计数
        let execution_count = self.jit_cache.increment_count(instructions_key);

        // 检查是否需要编译
        if execution_count >= 10 && self.jit_compiler.should_compile(instructions) {
            // 编译指令序列
            match self.jit_compiler.compile(instructions) {
                Ok(compiled_func) => {
                    self.jit_cache.cache_compiled(instructions_key, compiled_func, crate::jit::CompilationLevel::Baseline);
                    println!("JIT compiled instructions with {} instructions", instructions.len());
                }
                Err(err) => {
                    println!("JIT compilation failed: {:?}", err);
                }
            }
        }

        // 解释执行
        while self.pc < instructions.len() {
            let instr = &instructions[self.pc];
            self.pc += 1;

            match instr {
                // 加载指令
                Instruction::LoadConst(value) => {
                    self.registers[0] = value.clone();
                }
                Instruction::LoadLocal(index) => {
                    let context = self.context.lock().unwrap();
                    if let Some(value) = context.get_local(*index) {
                        self.registers[0] = value.clone();
                    }
                    else {
                        self.registers[0] = RubyValue::Nil;
                    }
                }
                Instruction::LoadGlobal(name) => {
                    let context = self.context.lock().unwrap();
                    if let Some(value) = context.get_global(name.as_str()) {
                        self.registers[0] = value.clone();
                    }
                    else {
                        self.registers[0] = RubyValue::Nil;
                    }
                }
                Instruction::LoadInstance(name) => {
                    let context = self.context.lock().unwrap();
                    if let Some(value) = context.get_instance_variable(name.as_str()) {
                        self.registers[0] = value.clone();
                    }
                    else {
                        self.registers[0] = RubyValue::Nil;
                    }
                }
                Instruction::LoadClass(name) => {
                    let context = self.context.lock().unwrap();
                    if let Some(value) = context.get_class_variable(name.as_str()) {
                        self.registers[0] = value.clone();
                    }
                    else {
                        self.registers[0] = RubyValue::Nil;
                    }
                }

                // 存储指令
                Instruction::StoreLocal(index) => {
                    let mut context = self.context.lock().unwrap();
                    context.set_local(*index, self.registers[0].clone());
                }
                Instruction::StoreGlobal(name) => {
                    let mut context = self.context.lock().unwrap();
                    context.set_global_string(name.clone(), self.registers[0].clone());
                }
                Instruction::StoreInstance(name) => {
                    let mut context = self.context.lock().unwrap();
                    context.set_instance_variable(name.as_str(), self.registers[0].clone());
                }
                Instruction::StoreClass(name) => {
                    let mut context = self.context.lock().unwrap();
                    context.set_class_variable(name.as_str(), self.registers[0].clone());
                }

                // 算术指令
                Instruction::Add => {
                    match (&self.registers[0], &self.registers[1]) {
                        (RubyValue::Integer(a), RubyValue::Integer(b)) => {
                            self.registers[0] = RubyValue::Integer(a + b);
                        }
                        (RubyValue::String(_), RubyValue::String(_)) | (RubyValue::Array(_), RubyValue::Array(_)) => {
                            // 字符串或数组拼接
                            let result = self.registers[0].concat(&self.registers[1]);
                            self.registers[0] = result;
                        }
                        _ => {
                            let a = self.registers[0].to_f64();
                            let b = self.registers[1].to_f64();
                            self.registers[0] = RubyValue::Float(a + b);
                        }
                    }
                }
                Instruction::Sub => match (&self.registers[0], &self.registers[1]) {
                    (RubyValue::Integer(a), RubyValue::Integer(b)) => {
                        self.registers[0] = RubyValue::Integer(a - b);
                    }
                    _ => {
                        let a = self.registers[0].to_f64();
                        let b = self.registers[1].to_f64();
                        self.registers[0] = RubyValue::Float(a - b);
                    }
                },
                Instruction::Mul => match (&self.registers[0], &self.registers[1]) {
                    (RubyValue::Integer(a), RubyValue::Integer(b)) => {
                        self.registers[0] = RubyValue::Integer(a * b);
                    }
                    _ => {
                        let a = self.registers[0].to_f64();
                        let b = self.registers[1].to_f64();
                        self.registers[0] = RubyValue::Float(a * b);
                    }
                },
                Instruction::Div => {
                    let a = self.registers[0].to_f64();
                    let b = self.registers[1].to_f64();
                    if b == 0.0 {
                        return Err(RubyError::RuntimeError("Division by zero".to_string()));
                    }
                    self.registers[0] = RubyValue::Float(a / b);
                }
                Instruction::Mod => match (&self.registers[0], &self.registers[1]) {
                    (RubyValue::Integer(a), RubyValue::Integer(b)) => {
                        if *b == 0 {
                            return Err(RubyError::RuntimeError("Division by zero".to_string()));
                        }
                        self.registers[0] = RubyValue::Integer(a % b);
                    }
                    _ => {
                        let a = self.registers[0].to_f64();
                        let b = self.registers[1].to_f64();
                        if b == 0.0 {
                            return Err(RubyError::RuntimeError("Division by zero".to_string()));
                        }
                        self.registers[0] = RubyValue::Float(a % b);
                    }
                },
                Instruction::Exp => {
                    let a = self.registers[0].to_f64();
                    let b = self.registers[1].to_f64();
                    self.registers[0] = RubyValue::Float(a.powf(b));
                }

                // 比较指令
                Instruction::Eq => {
                    let a = &self.registers[0];
                    let b = &self.registers[1];
                    self.registers[0] = RubyValue::Boolean(a == b);
                }
                Instruction::Neq => {
                    let a = &self.registers[0];
                    let b = &self.registers[1];
                    self.registers[0] = RubyValue::Boolean(a != b);
                }
                Instruction::Lt => {
                    let a = self.registers[0].to_f64();
                    let b = self.registers[1].to_f64();
                    self.registers[0] = RubyValue::Boolean(a < b);
                }
                Instruction::Lte => {
                    let a = self.registers[0].to_f64();
                    let b = self.registers[1].to_f64();
                    self.registers[0] = RubyValue::Boolean(a <= b);
                }
                Instruction::Gt => {
                    let a = self.registers[0].to_f64();
                    let b = self.registers[1].to_f64();
                    self.registers[0] = RubyValue::Boolean(a > b);
                }
                Instruction::Gte => {
                    let a = self.registers[0].to_f64();
                    let b = self.registers[1].to_f64();
                    self.registers[0] = RubyValue::Boolean(a >= b);
                }

                // 逻辑指令
                Instruction::And => {
                    let a = self.registers[0].to_bool();
                    let b = self.registers[1].to_bool();
                    self.registers[0] = RubyValue::Boolean(a && b);
                }
                Instruction::Or => {
                    let a = self.registers[0].to_bool();
                    let b = self.registers[1].to_bool();
                    self.registers[0] = RubyValue::Boolean(a || b);
                }
                Instruction::Not => {
                    let a = self.registers[0].to_bool();
                    self.registers[0] = RubyValue::Boolean(!a);
                }

                // 控制流指令
                Instruction::Jump(offset) => {
                    self.pc = (self.pc as i32 + offset - 1) as usize;
                }
                Instruction::JumpIfFalse(offset) => {
                    if !self.registers[0].to_bool() {
                        self.pc = (self.pc as i32 + offset - 1) as usize;
                    }
                }
                Instruction::JumpIfTrue(offset) => {
                    if self.registers[0].to_bool() {
                        self.pc = (self.pc as i32 + offset - 1) as usize;
                    }
                }

                // 方法调用指令
                Instruction::CallMethod(name, arg_count) => {
                    let mut args = Vec::new();
                    for i in 0..*arg_count {
                        args.push(self.registers[i + 1].clone());
                    }

                    let mut context = self.context.lock().unwrap();
                    let result = context.call_method(name, args)?;
                    self.registers[0] = result;
                }
                Instruction::Return => {
                    // 从栈中恢复 PC 和寄存器状态
                    // 简化实现，实际需要更复杂的栈管理
                    return Ok(());
                }

                // 数组和哈希指令
                Instruction::NewArray(size) => {
                    let mut array = Vec::new();
                    for i in 0..*size {
                        array.push(self.registers[i + 1].clone());
                    }
                    let value = RubyValue::Array(array);
                    let mut gc = self.gc.lock().unwrap();
                    let allocated_value = gc.allocate(value);
                    self.registers[0] = allocated_value;
                }
                Instruction::NewHash(size) => {
                    let mut hash = std::collections::HashMap::new();
                    for i in 0..*size {
                        let key = self.registers[i * 2 + 1].to_string();
                        let value = self.registers[i * 2 + 2].clone();
                        hash.insert(key, value);
                    }
                    let value = RubyValue::Hash(hash);
                    let mut gc = self.gc.lock().unwrap();
                    let allocated_value = gc.allocate(value);
                    self.registers[0] = allocated_value;
                }

                // 特殊指令
                Instruction::Nil => {
                    self.registers[0] = RubyValue::Nil;
                }
                Instruction::True => {
                    self.registers[0] = RubyValue::Boolean(true);
                }
                Instruction::False => {
                    self.registers[0] = RubyValue::Boolean(false);
                }
                Instruction::Move(src, dst) => {
                    if *src < self.registers.len() && *dst < self.registers.len() {
                        self.registers[*dst] = self.registers[*src].clone();
                    }
                }

                // FFI 指令
                Instruction::LoadLibrary(path) => {
                    let mut context = self.context.lock().unwrap();
                    match unsafe { Library::new(path) } {
                        Ok(lib) => {
                            context.add_library(path, lib);
                            self.registers[0] = RubyValue::Boolean(true);
                        }
                        Err(err) => {
                            println!("Failed to load library: {:?}", err);
                            self.registers[0] = RubyValue::Boolean(false);
                        }
                    }
                }
                Instruction::GetFunction(lib_name, func_name) => {
                    let mut context = self.context.lock().unwrap();
                    if let Some(lib) = context.get_library(lib_name) {
                        match unsafe { lib.get::<unsafe extern "C" fn() -> i32>(func_name.as_bytes()) } {
                            Ok(func) => {
                                let func_key = format!("{}_{}", lib_name, func_name);
                                // 注意：这里使用了 unsafe 来延长生命周期，实际生产环境需要更安全的处理
                                let static_func = unsafe {
                                    std::mem::transmute::<
                                        libloading::Symbol<'_, unsafe extern "C" fn() -> i32>,
                                        libloading::Symbol<'static, unsafe extern "C" fn() -> i32>,
                                    >(func)
                                };
                                context.add_function(&func_key, static_func);
                                self.registers[0] = RubyValue::String(func_key);
                            }
                            Err(err) => {
                                println!("Failed to get function: {:?}", err);
                                self.registers[0] = RubyValue::Nil;
                            }
                        }
                    }
                    else {
                        self.registers[0] = RubyValue::Nil;
                    }
                }
                Instruction::CallFunction(_arg_count) => {
                    let context = self.context.lock().unwrap();
                    if let RubyValue::String(func_key) = &self.registers[0] {
                        if let Some(func) = context.get_function(func_key) {
                            // 简单实现：只支持无参数函数，返回 i32
                            let result = unsafe { func() };
                            self.registers[0] = RubyValue::Integer(result);
                        }
                        else {
                            self.registers[0] = RubyValue::Nil;
                        }
                    }
                    else {
                        self.registers[0] = RubyValue::Nil;
                    }
                }

                // 异常处理指令
                Instruction::Try => {
                    // 记录当前 PC 作为异常处理的起始点
                    self.exception_stack.push(self.pc);
                }
                Instruction::Catch(offset) => {
                    // 移除异常处理栈顶元素
                    self.exception_stack.pop();
                    // 跳转到指定偏移量的指令
                    self.pc = (self.pc as i32 + offset - 1) as usize;
                }
                Instruction::Throw => {
                    // 抛出异常
                    let exception = self.registers[0].clone();
                    // 检查是否有异常处理块
                    if let Some(catch_pc) = self.exception_stack.pop() {
                        // 跳转到异常处理块
                        self.pc = catch_pc;
                        // 将异常值存储在寄存器 0 中
                        self.registers[0] = exception;
                    }
                    else {
                        // 如果没有异常处理块，返回错误
                        return Err(RubyError::RuntimeError(format!("Unhandled exception: {:?}", exception)));
                    }
                }
            }
        }

        Ok(())
    }
}

/// Ruby 虚拟机
///
/// 基于寄存器的虚拟机，用于执行 Ruby 代码。
/// 负责指令执行、垃圾收集等核心功能。
pub struct VM {
    /// 执行上下文
    ///
    /// 管理运行时的变量、方法等信息
    context: Arc<Mutex<Context>>,
    /// 垃圾收集器
    ///
    /// 负责内存管理和垃圾回收
    gc: Arc<Mutex<Box<dyn crate::gc::GC>>>,
    /// 执行引擎
    ///
    /// 负责协调解释器、编译器和执行路径的切换
    execution_engine: crate::architecture::execution_engine::ExecutionEngine,
}

impl VM {
    /// 创建新的虚拟机
    ///
    /// # 返回值
    /// - `Self`：新创建的虚拟机实例
    pub fn new() -> Self {
        // 创建分代垃圾收集器
        let gc = create_gc(GCType::Generational);

        Self {
            context: Arc::new(Mutex::new(Context::new())),
            gc: Arc::new(Mutex::new(gc)),
            execution_engine: crate::architecture::execution_engine::ExecutionEngine::new(),
        }
    }

    /// 创建使用指定垃圾收集器的虚拟机
    ///
    /// # 参数
    /// - `gc_type`：垃圾收集器类型
    ///
    /// # 返回值
    /// - `Self`：新创建的虚拟机实例
    pub fn new_with_gc(gc_type: GCType) -> Self {
        let gc = create_gc(gc_type);

        Self {
            context: Arc::new(Mutex::new(Context::new())),
            gc: Arc::new(Mutex::new(gc)),
            execution_engine: crate::architecture::execution_engine::ExecutionEngine::new(),
        }
    }

    /// 执行指令
    ///
    /// # 参数
    /// - `instructions`：要执行的指令序列
    ///
    /// # 返回值
    /// - `Ok(())`：指令执行成功
    /// - `Err(RubyError)`：指令执行失败
    pub fn execute(&mut self, instructions: &[Instruction]) -> RubyResult<()> {
        let mut context = self.context.lock().unwrap();
        let mut registers = vec![RubyValue::Nil; 16]; // 16个通用寄存器
        let result = self.execution_engine.execute(instructions, &mut context, &mut registers);

        // 执行完成后触发GC
        self.collect_garbage();

        result
    }

    /// 执行垃圾收集
    ///
    /// 从上下文中获取根对象，执行标记-清除算法回收未使用的内存。
    pub fn collect_garbage(&self) {
        // 从上下文中获取根对象
        let mut roots: Vec<&RubyValue> = Vec::new();

        // 获取上下文中的全局变量作为根对象
        let context = self.context.lock().unwrap();
        for (_, value) in &context.globals {
            roots.push(value);
        }

        // 获取局部变量作为根对象
        for (_, value) in &context.locals {
            roots.push(value);
        }

        // 获取实例变量作为根对象
        for (_, value) in &context.instance_variables {
            roots.push(value);
        }

        // 获取类变量作为根对象
        for (_, value) in &context.class_variables {
            roots.push(value);
        }

        // 触发GC
        let mut gc = self.gc.lock().unwrap();
        gc.collect(&roots);
    }

    /// 获取执行上下文
    ///
    /// # 返回值
    /// - `Arc<Mutex<Context>>`：执行上下文的共享引用
    pub fn context(&self) -> Arc<Mutex<Context>> {
        self.context.clone()
    }
}
