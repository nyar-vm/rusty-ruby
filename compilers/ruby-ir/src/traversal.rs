//! IR traversal and manipulation utilities

use crate::{BasicBlock, BlockId, Class, ClassId, Expression, Function, FunctionId, Module, ModuleId, Program, Statement};

/// Trait for visiting IR nodes
pub trait Visitor {
    /// Visit an expression
    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(_) => {}
            Expression::Variable(_) => {}
            Expression::GlobalVariable(_) => {}
            Expression::InstanceVariable(_) => {}
            Expression::ClassVariable(_) => {}
            Expression::MethodCall { receiver, method: _, arguments } => {
                self.visit_expression(receiver);
                for arg in arguments {
                    self.visit_expression(arg);
                }
            }
            Expression::BinaryOp { left, op: _, right } => {
                self.visit_expression(left);
                self.visit_expression(right);
            }
            Expression::UnaryOp { op: _, operand } => {
                self.visit_expression(operand);
            }
            Expression::ArrayLiteral(elements) => {
                for elem in elements {
                    self.visit_expression(elem);
                }
            }
            Expression::HashLiteral(pairs) => {
                for (_, value) in pairs {
                    self.visit_expression(value);
                }
            }
            Expression::Block { parameters: _, body } => {
                for stmt in body {
                    self.visit_statement(stmt);
                }
            }
            Expression::SelfRef => {}
            Expression::SuperCall { arguments } => {
                for arg in arguments {
                    self.visit_expression(arg);
                }
            }
        }
    }

    /// Visit a statement
    fn visit_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Expression(expr) => {
                self.visit_expression(expr);
            }
            Statement::Assignment { name: _, value } => {
                self.visit_expression(value);
            }
            Statement::GlobalAssignment { name: _, value } => {
                self.visit_expression(value);
            }
            Statement::InstanceAssignment { name: _, value } => {
                self.visit_expression(value);
            }
            Statement::ClassAssignment { name: _, value } => {
                self.visit_expression(value);
            }
            Statement::If { condition, then_branch, else_branch } => {
                self.visit_expression(condition);
                for stmt in then_branch {
                    self.visit_statement(stmt);
                }
                for stmt in else_branch {
                    self.visit_statement(stmt);
                }
            }
            Statement::While { condition, body } => {
                self.visit_expression(condition);
                for stmt in body {
                    self.visit_statement(stmt);
                }
            }
            Statement::Until { condition, body } => {
                self.visit_expression(condition);
                for stmt in body {
                    self.visit_statement(stmt);
                }
            }
            Statement::Case { value, when_clauses, else_clause } => {
                self.visit_expression(value);
                for (cond, body) in when_clauses {
                    self.visit_expression(cond);
                    for stmt in body {
                        self.visit_statement(stmt);
                    }
                }
                for stmt in else_clause {
                    self.visit_statement(stmt);
                }
            }
            Statement::For { variable: _, iterator, body } => {
                self.visit_expression(iterator);
                for stmt in body {
                    self.visit_statement(stmt);
                }
            }
            Statement::Break => {}
            Statement::Next => {}
            Statement::Redo => {}
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    self.visit_expression(expr);
                }
            }
            Statement::MethodDefinition { name: _, parameters: _, body } => {
                for stmt in body {
                    self.visit_statement(stmt);
                }
            }
            Statement::ClassDefinition { name: _, superclass: _, body } => {
                for stmt in body {
                    self.visit_statement(stmt);
                }
            }
            Statement::ModuleDefinition { name: _, body } => {
                for stmt in body {
                    self.visit_statement(stmt);
                }
            }
            Statement::Require(expr) => {
                self.visit_expression(expr);
            }
            Statement::Load(expr) => {
                self.visit_expression(expr);
            }
        }
    }

    /// Visit a basic block
    fn visit_basic_block(&mut self, block: &BasicBlock) {
        for stmt in &block.statements {
            self.visit_statement(stmt);
        }
    }

    /// Visit a function
    fn visit_function(&mut self, function: &Function) {
        for block in function.blocks.values() {
            self.visit_basic_block(block);
        }
    }

    /// Visit a class
    fn visit_class(&mut self, _class: &Class) {
        // Classes themselves don't have direct statements, but their methods do
    }

    /// Visit a module
    fn visit_module(&mut self, _module: &Module) {
        // Modules themselves don't have direct statements, but their methods do
    }

    /// Visit a program
    fn visit_program(&mut self, program: &Program) {
        for stmt in &program.global_statements {
            self.visit_statement(stmt);
        }
        for function in program.functions.values() {
            self.visit_function(function);
        }
        for class in program.classes.values() {
            self.visit_class(class);
        }
        for module in program.modules.values() {
            self.visit_module(module);
        }
    }
}

/// Trait for mutating IR nodes
pub trait Mutator {
    /// Mutate an expression
    fn mutate_expression(&mut self, expr: &mut Expression) {
        match expr {
            Expression::Literal(_) => {}
            Expression::Variable(_) => {}
            Expression::GlobalVariable(_) => {}
            Expression::InstanceVariable(_) => {}
            Expression::ClassVariable(_) => {}
            Expression::MethodCall { receiver, method: _, arguments } => {
                self.mutate_expression(receiver);
                for arg in arguments {
                    self.mutate_expression(arg);
                }
            }
            Expression::BinaryOp { left, op: _, right } => {
                self.mutate_expression(left);
                self.mutate_expression(right);
            }
            Expression::UnaryOp { op: _, operand } => {
                self.mutate_expression(operand);
            }
            Expression::ArrayLiteral(elements) => {
                for elem in elements {
                    self.mutate_expression(elem);
                }
            }
            Expression::HashLiteral(pairs) => {
                for (_, value) in pairs {
                    self.mutate_expression(value);
                }
            }
            Expression::Block { parameters: _, body } => {
                for stmt in body {
                    self.mutate_statement(stmt);
                }
            }
            Expression::SelfRef => {}
            Expression::SuperCall { arguments } => {
                for arg in arguments {
                    self.mutate_expression(arg);
                }
            }
        }
    }

    /// Mutate a statement
    fn mutate_statement(&mut self, stmt: &mut Statement) {
        match stmt {
            Statement::Expression(expr) => {
                self.mutate_expression(expr);
            }
            Statement::Assignment { name: _, value } => {
                self.mutate_expression(value);
            }
            Statement::GlobalAssignment { name: _, value } => {
                self.mutate_expression(value);
            }
            Statement::InstanceAssignment { name: _, value } => {
                self.mutate_expression(value);
            }
            Statement::ClassAssignment { name: _, value } => {
                self.mutate_expression(value);
            }
            Statement::If { condition, then_branch, else_branch } => {
                self.mutate_expression(condition);
                for stmt in then_branch {
                    self.mutate_statement(stmt);
                }
                for stmt in else_branch {
                    self.mutate_statement(stmt);
                }
            }
            Statement::While { condition, body } => {
                self.mutate_expression(condition);
                for stmt in body {
                    self.mutate_statement(stmt);
                }
            }
            Statement::Until { condition, body } => {
                self.mutate_expression(condition);
                for stmt in body {
                    self.mutate_statement(stmt);
                }
            }
            Statement::Case { value, when_clauses, else_clause } => {
                self.mutate_expression(value);
                for (cond, body) in when_clauses {
                    self.mutate_expression(cond);
                    for stmt in body {
                        self.mutate_statement(stmt);
                    }
                }
                for stmt in else_clause {
                    self.mutate_statement(stmt);
                }
            }
            Statement::For { variable: _, iterator, body } => {
                self.mutate_expression(iterator);
                for stmt in body {
                    self.mutate_statement(stmt);
                }
            }
            Statement::Break => {}
            Statement::Next => {}
            Statement::Redo => {}
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    self.mutate_expression(expr);
                }
            }
            Statement::MethodDefinition { name: _, parameters: _, body } => {
                for stmt in body {
                    self.mutate_statement(stmt);
                }
            }
            Statement::ClassDefinition { name: _, superclass: _, body } => {
                for stmt in body {
                    self.mutate_statement(stmt);
                }
            }
            Statement::ModuleDefinition { name: _, body } => {
                for stmt in body {
                    self.mutate_statement(stmt);
                }
            }
            Statement::Require(expr) => {
                self.mutate_expression(expr);
            }
            Statement::Load(expr) => {
                self.mutate_expression(expr);
            }
        }
    }

    /// Mutate a basic block
    fn mutate_basic_block(&mut self, block: &mut BasicBlock) {
        for stmt in &mut block.statements {
            self.mutate_statement(stmt);
        }
    }

    /// Mutate a function
    fn mutate_function(&mut self, function: &mut Function) {
        for block in function.blocks.values_mut() {
            self.mutate_basic_block(block);
        }
    }

    /// Mutate a class
    fn mutate_class(&mut self, _class: &mut Class) {
        // Classes themselves don't have direct statements, but their methods do
    }

    /// Mutate a module
    fn mutate_module(&mut self, _module: &mut Module) {
        // Modules themselves don't have direct statements, but their methods do
    }

    /// Mutate a program
    fn mutate_program(&mut self, program: &mut Program) {
        for stmt in &mut program.global_statements {
            self.mutate_statement(stmt);
        }
        for function in program.functions.values_mut() {
            self.mutate_function(function);
        }
        for class in program.classes.values_mut() {
            self.mutate_class(class);
        }
        for module in program.modules.values_mut() {
            self.mutate_module(module);
        }
    }
}

/// Builder for creating IR programs
pub struct ProgramBuilder {
    program: Program,
    next_function_id: FunctionId,
    next_class_id: ClassId,
    next_module_id: ModuleId,
    next_block_id: BlockId,
}

impl ProgramBuilder {
    /// Create a new program builder
    pub fn new() -> Self {
        Self { program: Program::new(), next_function_id: 0, next_class_id: 0, next_module_id: 0, next_block_id: 0 }
    }

    /// Build the program
    pub fn build(self) -> Program {
        self.program
    }

    /// Add a global statement
    pub fn add_global_statement(&mut self, statement: Statement) -> &mut Self {
        self.program.add_global_statement(statement);
        self
    }

    /// Create a new function
    pub fn new_function(&mut self, name: String, parameters: Vec<String>) -> FunctionBuilder<'_> {
        let function_id = self.next_function_id;
        self.next_function_id += 1;

        FunctionBuilder {
            function: Function { id: function_id, name, parameters, blocks: std::collections::HashMap::new(), entry_block: 0 },
            next_block_id: 0,
            program_builder: self,
        }
    }

    /// Create a new class
    pub fn new_class(&mut self, name: String, superclass: Option<String>) -> ClassBuilder<'_> {
        let class_id = self.next_class_id;
        self.next_class_id += 1;

        ClassBuilder { class: Class { id: class_id, name, superclass, methods: std::collections::HashMap::new() }, program_builder: self }
    }

    /// Create a new module
    pub fn new_module(&mut self, name: String) -> ModuleBuilder<'_> {
        let module_id = self.next_module_id;
        self.next_module_id += 1;

        ModuleBuilder { module: Module { id: module_id, name, methods: std::collections::HashMap::new() }, program_builder: self }
    }

    /// Get the next block ID
    fn next_block_id(&mut self) -> BlockId {
        let id = self.next_block_id;
        self.next_block_id += 1;
        id
    }
}

/// Builder for creating IR functions
pub struct FunctionBuilder<'a> {
    function: Function,
    next_block_id: BlockId,
    program_builder: &'a mut ProgramBuilder,
}

impl<'a> FunctionBuilder<'a> {
    /// Create a new basic block
    pub fn new_block(&mut self) -> BlockBuilder<'_, 'a> {
        let block_id = self.next_block_id;
        self.next_block_id += 1;

        BlockBuilder { block: BasicBlock { id: block_id, statements: Vec::new(), successors: Vec::new() }, function_builder: self }
    }

    /// Set the entry block
    pub fn entry_block(&mut self, block_id: BlockId) -> &mut Self {
        self.function.entry_block = block_id;
        self
    }

    /// Build the function and add it to the program
    pub fn build(self) -> &'a mut ProgramBuilder {
        self.program_builder.program.add_function(self.function);
        self.program_builder
    }
}

/// Builder for creating IR basic blocks
pub struct BlockBuilder<'b, 'a: 'b> {
    block: BasicBlock,
    function_builder: &'b mut FunctionBuilder<'a>,
}

impl<'b, 'a: 'b> BlockBuilder<'b, 'a> {
    /// Add a statement to the block
    pub fn add_statement(&mut self, statement: Statement) -> &mut Self {
        self.block.statements.push(statement);
        self
    }

    /// Add a successor block
    pub fn add_successor(&mut self, successor_id: BlockId) -> &mut Self {
        self.block.successors.push(successor_id);
        self
    }

    /// Build the block and add it to the function
    pub fn build(self) -> &'b mut FunctionBuilder<'a> {
        self.function_builder.function.blocks.insert(self.block.id, self.block);
        self.function_builder
    }
}

/// Builder for creating IR classes
pub struct ClassBuilder<'a> {
    class: Class,
    program_builder: &'a mut ProgramBuilder,
}

impl<'a> ClassBuilder<'a> {
    /// Add a method to the class
    pub fn add_method(&mut self, method_name: String, function_id: FunctionId) -> &mut Self {
        self.class.methods.insert(method_name, function_id);
        self
    }

    /// Build the class and add it to the program
    pub fn build(self) -> &'a mut ProgramBuilder {
        self.program_builder.program.add_class(self.class);
        self.program_builder
    }
}

/// Builder for creating IR modules
pub struct ModuleBuilder<'a> {
    module: Module,
    program_builder: &'a mut ProgramBuilder,
}

impl<'a> ModuleBuilder<'a> {
    /// Add a method to the module
    pub fn add_method(&mut self, method_name: String, function_id: FunctionId) -> &mut Self {
        self.module.methods.insert(method_name, function_id);
        self
    }

    /// Build the module and add it to the program
    pub fn build(self) -> &'a mut ProgramBuilder {
        self.program_builder.program.add_module(self.module);
        self.program_builder
    }
}
