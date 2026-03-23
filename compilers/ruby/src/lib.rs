//! Rusty Ruby 语言前端
//! 
//! 这个库提供了 Rusty Ruby 语言的词法分析、语法分析和运行时功能。

#![warn(missing_docs)]

use std::rc::Rc;
use oak_ruby::{RubyLanguage, RubyLexer, RubyParser};
use oak_core::{parser::Parser, lexer::Lexer, TextEdit, errors::ParseResult};
use ruby_types::{RubyValue, RubyError, RubyResult};

/// Ruby 运行时错误类型
type Result<T> = RubyResult<T>;

/// Trait for converting Rust types to Ruby values
pub trait ToRubyValue {
    /// Convert the Rust type to a Ruby Value
    fn to_ruby_value(&self) -> RubyValue;
}

/// 执行上下文，用于存储运行时的变量、方法等信息
pub struct Context {
    /// 全局变量
    globals: std::collections::HashMap<String, RubyValue>,
    /// 局部变量
    locals: std::collections::HashMap<String, RubyValue>,
    /// 实例变量
    instance_variables: std::collections::HashMap<String, RubyValue>,
    /// 类变量
    class_variables: std::collections::HashMap<String, RubyValue>,
    /// 方法定义
    methods: std::collections::HashMap<String, Rc<dyn Fn(&mut Context, Vec<RubyValue>) -> Result<RubyValue>>>,
    /// 类定义
    classes: std::collections::HashMap<String, std::collections::HashMap<String, Box<dyn Fn(&mut Context, Vec<RubyValue>) -> Result<RubyValue>>>>,
}

impl Context {
    /// 创建新的执行上下文
    pub fn new() -> Self {
        Self {
            globals: std::collections::HashMap::new(),
            locals: std::collections::HashMap::new(),
            instance_variables: std::collections::HashMap::new(),
            class_variables: std::collections::HashMap::new(),
            methods: std::collections::HashMap::new(),
            classes: std::collections::HashMap::new(),
        }
    }
    
    /// 获取全局变量
    pub fn get_global(&self, name: &str) -> Option<&RubyValue> {
        self.globals.get(name)
    }
    
    /// 设置全局变量
    pub fn set_global(&mut self, name: &str, value: RubyValue) {
        self.globals.insert(name.to_string(), value);
    }
    
    /// 获取局部变量
    pub fn get_local(&self, name: &str) -> Option<&RubyValue> {
        self.locals.get(name)
    }
    
    /// 设置局部变量
    pub fn set_local(&mut self, name: &str, value: RubyValue) {
        self.locals.insert(name.to_string(), value);
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
    pub fn define_method(&mut self, name: &str, func: Box<dyn Fn(&mut Context, Vec<RubyValue>) -> Result<RubyValue>>) {
        self.methods.insert(name.to_string(), Rc::new(func));
    }
    
    /// 调用方法
    pub fn call_method(&mut self, name: &str, args: Vec<RubyValue>) -> Result<RubyValue> {
        // Create a scope to limit the immutable borrow
        let method = {
            self.methods.get(name).cloned()
        };
        
        match method {
            Some(m) => m(self, args),
            None => Err(RubyError::MethodNotFound(name.to_string())),
        }
    }
}

/// Ruby 运行时环境
pub struct Ruby {
    language: RubyLanguage,
    context: Context,
}

impl Ruby {
    /// 创建新的 Ruby 运行时环境
    pub fn new() -> Result<Self> {
        Ok(Self {
            language: RubyLanguage::new(),
            context: Context::new(),
        })
    }
    
    /// 执行 Ruby 脚本
    pub fn execute_script(&mut self, script: &str) -> Result<()> {
        let lexer = RubyLexer::new(&self.language);
        let parser = RubyParser::new(&self.language);
        let edits: &[TextEdit] = &[];
        
        // 词法分析
        let mut lexer_cache = oak_core::parser::ParseSession::default();
        lexer.lex(script, edits, &mut lexer_cache);
        
        // 语法分析
        let mut parser_cache = oak_core::parser::ParseSession::default();
        let _parse_result = parser.parse(script, edits, &mut parser_cache);
        
        // 执行逻辑
        self.execute_ast(Ok(()))?;
        
        Ok(())
    }
    
    /// 执行 AST
    fn execute_ast(&mut self, _ast: ParseResult<()>) -> Result<()> {
        // 这里实现基本的 AST 执行逻辑
        // 由于我们没有具体的 AST 结构定义，这里实现一个简化版的执行器
        // 实际实现需要根据具体的 AST 结构来调整
        
        // 模拟执行一些基本操作
        println!("Executing AST...");
        
        // 示例：设置一个全局变量
        self.context.set_global("$global_var", RubyValue::Integer(42));
        println!("Set global variable $global_var = 42");
        
        // 示例：设置一个局部变量
        self.context.set_local("local_var", RubyValue::String("Hello, Ruby!".to_string()));
        println!("Set local variable local_var = 'Hello, Ruby!'");
        
        // 示例：设置一个实例变量
        self.context.set_instance_variable("@instance_var", RubyValue::Float(3.14));
        println!("Set instance variable @instance_var = 3.14");
        
        // 示例：设置一个类变量
        self.context.set_class_variable("@@class_var", RubyValue::Boolean(true));
        println!("Set class variable @@class_var = true");
        
        // 示例：定义一个带参数的方法
        self.context.define_method("add", Box::new(|_ctx, args| {
            if args.len() >= 2 {
                let sum = args[0].to_f64() + args[1].to_f64();
                Ok(RubyValue::Float(sum))
            } else {
                Err(RubyError::ArgumentError("Not enough arguments for add method".to_string()))
            }
        }));
        
        // 示例：调用带参数的方法
        let result = self.context.call_method("add", vec![RubyValue::Integer(10), RubyValue::Integer(20)])?;
        println!("Method call result: add(10, 20) = {:?}", result);
        
        // 示例：定义一个 say_hello 方法
        self.context.define_method("say_hello", Box::new(|_ctx, args| {
            let name = if args.len() > 0 {
                args[0].to_string()
            } else {
                "World".to_string()
            };
            println!("Hello, {}!", name);
            Ok(RubyValue::String(format!("Hello, {}", name)))
        }));
        
        // 示例：调用 say_hello 方法
        let result = self.context.call_method("say_hello", vec![RubyValue::String("Ruby".to_string())])?;
        println!("Method call result: {:?}", result);
        
        Ok(())
    }
    
    /// 定义 Ruby 类
    pub fn define_class(&mut self, name: &str) -> Result<()> {
        self.context.classes.insert(name.to_string(), std::collections::HashMap::new());
        Ok(())
    }
    
    /// 定义 Ruby 方法
    pub fn define_method(&mut self, class: &str, name: &str, _func: Box<dyn Fn()>) -> Result<()> {
        if let Some(class_methods) = self.context.classes.get_mut(class) {
            class_methods.insert(name.to_string(), Box::new(|_, _| Ok(RubyValue::Nil)));
        } else {
            return Err(RubyError::ClassNotFound(class.to_string()));
        }
        Ok(())
    }
    
    /// 定义 Ruby 模块
    pub fn define_module(&mut self, _name: &str) -> Result<()> {
        // TODO: 实现模块定义
        Ok(())
    }
    
    /// 定义 Ruby 模块函数
    pub fn define_module_function(&mut self, _module: &str, _name: &str, _func: Box<dyn Fn()>) -> Result<()> {
        // TODO: 实现模块函数定义
        Ok(())
    }
    
    /// 获取全局变量
    pub fn get_global(&self, name: &str) -> Result<RubyValue> {
        if let Some(value) = self.context.get_global(name) {
            Ok(value.clone())
        } else {
            Ok(RubyValue::Nil)
        }
    }
    
    /// 设置全局变量
    pub fn set_global(&mut self, name: &str, value: RubyValue) -> Result<()> {
        self.context.set_global(name, value);
        Ok(())
    }
}

/// Implement ToRubyValue for RubyValue itself
impl ToRubyValue for RubyValue {
    fn to_ruby_value(&self) -> RubyValue {
        self.clone()
    }
}

/// Implement ToRubyValue for common types
impl ToRubyValue for i32 {
    fn to_ruby_value(&self) -> RubyValue {
        RubyValue::Integer(*self)
    }
}

impl ToRubyValue for f64 {
    fn to_ruby_value(&self) -> RubyValue {
        RubyValue::Float(*self)
    }
}

impl ToRubyValue for bool {
    fn to_ruby_value(&self) -> RubyValue {
        RubyValue::Boolean(*self)
    }
}

impl ToRubyValue for String {
    fn to_ruby_value(&self) -> RubyValue {
        RubyValue::String(self.clone())
    }
}

impl ToRubyValue for &str {
    fn to_ruby_value(&self) -> RubyValue {
        RubyValue::String(self.to_string())
    }
}

impl<T: ToRubyValue> ToRubyValue for Vec<T> {
    fn to_ruby_value(&self) -> RubyValue {
        let values: Vec<_> = self.iter().map(|item| item.to_ruby_value()).collect();
        RubyValue::Array(values)
    }
}

impl<K: ToString, V: ToRubyValue> ToRubyValue for std::collections::HashMap<K, V> {
    fn to_ruby_value(&self) -> RubyValue {
        let mut hash = std::collections::HashMap::new();
        for (k, v) in self {
            hash.insert(k.to_string(), v.to_ruby_value());
        }
        RubyValue::Hash(hash)
    }
}
