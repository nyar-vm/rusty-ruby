//! Ruby 代码生成实现

use crate::vm;
use oak_ruby::ast::{ExpressionNode, LiteralNode, RubyAst, StatementNode};
use ruby_ir::{
    BasicBlock, BinaryOperator, BlockId, Class, ClassId, Expression, Function, FunctionId, Module, ModuleId, Program, Statement, UnaryOperator,
};
use ruby_types::{RubyError, RubyResult, RubyValue};
use std::collections::HashMap;
use vm::Instruction;

/// Type alias for Ruby result
pub type Result<T> = RubyResult<T>;

/// Convert Ruby AST to IR
pub fn ast_to_ir(ast: &RubyAst) -> Result<Program> {
    let mut program = Program::new();
    let mut context = ConversionContext::new();

    // Convert global statements
    for stmt in &ast.statements {
        let ir_stmt = context.convert_statement(stmt)?;
        program.add_global_statement(ir_stmt);
    }

    // Add functions
    for (_id, function) in context.functions {
        program.add_function(function);
    }

    // Add classes
    for (_id, class) in context.classes {
        program.add_class(class);
    }

    // Add modules
    for (_id, module) in context.modules {
        program.add_module(module);
    }

    Ok(program)
}

/// Conversion context for AST to IR conversion
struct ConversionContext {
    next_function_id: FunctionId,
    next_block_id: BlockId,
    next_class_id: ClassId,
    next_module_id: ModuleId,
    functions: HashMap<FunctionId, Function>,
    classes: HashMap<ClassId, Class>,
    modules: HashMap<ModuleId, Module>,
}

impl ConversionContext {
    /// Create a new conversion context
    fn new() -> Self {
        Self {
            next_function_id: 1,
            next_block_id: 1,
            next_class_id: 1,
            next_module_id: 1,
            functions: HashMap::new(),
            classes: HashMap::new(),
            modules: HashMap::new(),
        }
    }

    /// Get the next function ID
    fn next_function_id(&mut self) -> FunctionId {
        let id = self.next_function_id;
        self.next_function_id += 1;
        id
    }

    /// Get the next block ID
    fn next_block_id(&mut self) -> BlockId {
        let id = self.next_block_id;
        self.next_block_id += 1;
        id
    }

    /// Get the next class ID
    fn next_class_id(&mut self) -> ClassId {
        let id = self.next_class_id;
        self.next_class_id += 1;
        id
    }

    /// Get the next module ID
    fn next_module_id(&mut self) -> ModuleId {
        let id = self.next_module_id;
        self.next_module_id += 1;
        id
    }

    /// Convert a statement node to IR statement
    fn convert_statement(&mut self, stmt: &StatementNode) -> Result<Statement> {
        match stmt {
            StatementNode::Expression(expr) => {
                let ir_expr = self.convert_expression(expr)?;
                Ok(Statement::Expression(ir_expr))
            }
            StatementNode::MethodDef { name, params, body, .. } => {
                let function_id = self.next_function_id();
                let entry_block_id = self.next_block_id();

                let mut block = BasicBlock { id: entry_block_id, statements: Vec::new(), successors: Vec::new() };

                for stmt in body {
                    let ir_stmt = self.convert_statement(stmt)?;
                    block.statements.push(ir_stmt);
                }

                let mut blocks = HashMap::new();
                blocks.insert(entry_block_id, block);

                let function =
                    Function { id: function_id, name: name.clone(), parameters: params.clone(), blocks, entry_block: entry_block_id };

                self.functions.insert(function_id, function);

                Ok(Statement::MethodDefinition {
                    name: name.clone(),
                    parameters: params.clone(),
                    body: body.iter().map(|stmt| self.convert_statement(stmt).unwrap()).collect(),
                })
            }
            StatementNode::ClassDef { name, superclass, body, .. } => {
                let class_id = self.next_class_id();

                let mut methods = HashMap::new();

                // Process class body
                for stmt in body {
                    if let StatementNode::MethodDef { name: method_name, params, body: method_body, .. } = stmt {
                        let function_id = self.next_function_id();
                        let entry_block_id = self.next_block_id();

                        let mut block = BasicBlock { id: entry_block_id, statements: Vec::new(), successors: Vec::new() };

                        for stmt in method_body {
                            let ir_stmt = self.convert_statement(stmt)?;
                            block.statements.push(ir_stmt);
                        }

                        let mut blocks = HashMap::new();
                        blocks.insert(entry_block_id, block);

                        let function = Function {
                            id: function_id,
                            name: method_name.clone(),
                            parameters: params.clone(),
                            blocks,
                            entry_block: entry_block_id,
                        };

                        self.functions.insert(function_id, function);
                        methods.insert(method_name.clone(), function_id);
                    }
                }

                let class = Class { id: class_id, name: name.clone(), superclass: superclass.clone(), methods };

                self.classes.insert(class_id, class);

                Ok(Statement::ClassDefinition {
                    name: name.clone(),
                    superclass: superclass.clone(),
                    body: body.iter().map(|stmt| self.convert_statement(stmt).unwrap()).collect(),
                })
            }
            StatementNode::Assignment { target, value, .. } => {
                let ir_value = self.convert_expression(value)?;
                if target.starts_with('$') {
                    // Global variable assignment
                    Ok(Statement::GlobalAssignment { name: target.clone(), value: ir_value })
                }
                else if target.starts_with('@') {
                    // Instance variable assignment
                    Ok(Statement::InstanceAssignment { name: target.clone(), value: ir_value })
                }
                else if target.starts_with("@@") {
                    // Class variable assignment
                    Ok(Statement::ClassAssignment { name: target.clone(), value: ir_value })
                }
                else {
                    // Local variable assignment
                    Ok(Statement::Assignment { name: target.clone(), value: ir_value })
                }
            }
            StatementNode::If { condition, then_body, else_body, .. } => {
                let ir_condition = self.convert_expression(condition)?;
                let ir_then_body: Vec<Statement> = then_body.iter().map(|stmt| self.convert_statement(stmt).unwrap()).collect();
                let ir_else_body: Vec<Statement> =
                    else_body.as_ref().map(|body| body.iter().map(|stmt| self.convert_statement(stmt).unwrap()).collect()).unwrap_or_default();

                Ok(Statement::If { condition: ir_condition, then_branch: ir_then_body, else_branch: ir_else_body })
            }
            StatementNode::While { condition, body, .. } => {
                let ir_condition = self.convert_expression(condition)?;
                let ir_body: Vec<Statement> = body.iter().map(|stmt| self.convert_statement(stmt).unwrap()).collect();

                Ok(Statement::While { condition: ir_condition, body: ir_body })
            }
            StatementNode::Until { condition, body, .. } => {
                let ir_condition = self.convert_expression(condition)?;
                let ir_body: Vec<Statement> = body.iter().map(|stmt| self.convert_statement(stmt).unwrap()).collect();

                Ok(Statement::Until { condition: ir_condition, body: ir_body })
            }
            StatementNode::Case { value, when_clauses, else_clause, .. } => {
                let ir_value = self.convert_expression(value)?;
                let mut ir_when_clauses = Vec::new();

                for (cond, body) in when_clauses {
                    let ir_cond = self.convert_expression(cond)?;
                    let ir_body: Vec<Statement> = body.iter().map(|stmt| self.convert_statement(stmt).unwrap()).collect();
                    ir_when_clauses.push((ir_cond, ir_body));
                }

                let ir_else_clause = else_clause
                    .as_ref()
                    .map(|body| body.iter().map(|stmt| self.convert_statement(stmt).unwrap()).collect())
                    .unwrap_or_default();

                Ok(Statement::Case { value: ir_value, when_clauses: ir_when_clauses, else_clause: ir_else_clause })
            }
            StatementNode::Return { value, .. } => {
                let ir_value = value.as_ref().map(|expr| self.convert_expression(expr).unwrap());
                Ok(Statement::Return(ir_value))
            }
            StatementNode::Next { .. } => Ok(Statement::Next),
            StatementNode::Redo { .. } => Ok(Statement::Redo),
        }
    }

    /// Convert an expression node to IR expression
    fn convert_expression(&mut self, expr: &ExpressionNode) -> Result<Expression> {
        match expr {
            ExpressionNode::Identifier { name, .. } => {
                if name.starts_with('$') {
                    // Global variable
                    Ok(Expression::GlobalVariable(name.clone()))
                }
                else if name.starts_with('@') {
                    // Instance variable
                    Ok(Expression::InstanceVariable(name.clone()))
                }
                else if name.starts_with("@@") {
                    // Class variable
                    Ok(Expression::ClassVariable(name.clone()))
                }
                else {
                    // Local variable
                    Ok(Expression::Variable(name.clone()))
                }
            }
            ExpressionNode::Literal(literal) => {
                let ruby_value = self.convert_literal(literal)?;
                Ok(Expression::Literal(ruby_value))
            }
            ExpressionNode::MethodCall { receiver, method, args, .. } => {
                let ir_receiver = receiver.as_ref().map(|expr| Box::new(self.convert_expression(expr).unwrap()));
                let ir_args: Vec<Expression> = args.iter().map(|arg| self.convert_expression(arg).unwrap()).collect();

                Ok(Expression::MethodCall {
                    receiver: ir_receiver.unwrap_or_else(|| Box::new(Expression::SelfRef)),
                    method: method.clone(),
                    arguments: ir_args,
                })
            }
            ExpressionNode::BinaryOp { left, operator, right, .. } => {
                let ir_left = Box::new(self.convert_expression(left)?);
                let ir_right = Box::new(self.convert_expression(right)?);
                let ir_operator = self.convert_binary_operator(operator)?;

                Ok(Expression::BinaryOp { left: ir_left, op: ir_operator, right: ir_right })
            }
            ExpressionNode::UnaryOp { operator, operand, .. } => {
                let ir_operand = Box::new(self.convert_expression(operand)?);
                let ir_operator = self.convert_unary_operator(operator)?;

                Ok(Expression::UnaryOp { op: ir_operator, operand: ir_operand })
            }
            ExpressionNode::Array { elements, .. } => {
                let ir_elements: Vec<Expression> = elements.iter().map(|elem| self.convert_expression(elem).unwrap()).collect();
                Ok(Expression::ArrayLiteral(ir_elements))
            }
            ExpressionNode::Hash { pairs, .. } => {
                let mut ir_pairs = HashMap::new();
                for (key, value) in pairs {
                    let ir_key = self.convert_expression(key)?;
                    let ir_value = self.convert_expression(value)?;
                    // For simplicity, we'll only handle string keys for now
                    if let Expression::Literal(RubyValue::String(key_str)) = ir_key {
                        ir_pairs.insert(key_str, ir_value);
                    }
                }
                Ok(Expression::HashLiteral(ir_pairs))
            }
        }
    }

    /// Convert a literal node to RubyValue
    fn convert_literal(&self, literal: &LiteralNode) -> Result<RubyValue> {
        match literal {
            LiteralNode::Integer { value, .. } => Ok(RubyValue::Integer(*value as i32)),
            LiteralNode::Float { value, .. } => Ok(RubyValue::Float(*value)),
            LiteralNode::String { value, .. } => Ok(RubyValue::String(value.clone())),
            LiteralNode::Symbol { value, .. } => Ok(RubyValue::Symbol(value.clone())),
            LiteralNode::Boolean { value, .. } => Ok(RubyValue::Boolean(*value)),
            LiteralNode::Nil { .. } => Ok(RubyValue::Nil),
        }
    }

    /// Convert a binary operator string to BinaryOperator enum
    fn convert_binary_operator(&self, op: &str) -> Result<BinaryOperator> {
        match op {
            "+" => Ok(BinaryOperator::Add),
            "-" => Ok(BinaryOperator::Sub),
            "*" => Ok(BinaryOperator::Mul),
            "/" => Ok(BinaryOperator::Div),
            "%" => Ok(BinaryOperator::Mod),
            "**" => Ok(BinaryOperator::Exp),
            "==" => Ok(BinaryOperator::Eq),
            "!=" => Ok(BinaryOperator::Neq),
            "<" => Ok(BinaryOperator::Lt),
            "<=" => Ok(BinaryOperator::Lte),
            ">" => Ok(BinaryOperator::Gt),
            ">=" => Ok(BinaryOperator::Gte),
            "&&" => Ok(BinaryOperator::And),
            "||" => Ok(BinaryOperator::Or),
            "=" => Ok(BinaryOperator::Assign),
            _ => Err(RubyError::SyntaxError(format!("Unknown binary operator: {}", op))),
        }
    }

    /// Convert a unary operator string to UnaryOperator enum
    fn convert_unary_operator(&self, op: &str) -> Result<UnaryOperator> {
        match op {
            "!" => Ok(UnaryOperator::Not),
            "-" => Ok(UnaryOperator::Neg),
            "+" => Ok(UnaryOperator::Plus),
            "~" => Ok(UnaryOperator::BitNot),
            _ => Err(RubyError::SyntaxError(format!("Unknown unary operator: {}", op))),
        }
    }
}

/// Generate VM instructions from IR
pub fn ir_to_vm(_program: &Program) -> Result<Vec<u8>> {
    // TODO: Implement VM instruction generation
    // For now, we'll return an empty vector
    Ok(Vec::new())
}

/// Generate VM instructions from IR (using our new VM instruction set)
pub fn ir_to_vm_instructions(program: &Program) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::new();

    // Process global statements
    for stmt in &program.global_statements {
        generate_statement_instructions(stmt, &mut instructions)?;
    }

    Ok(instructions)
}

/// Generate instructions for a statement
fn generate_statement_instructions(stmt: &Statement, instructions: &mut Vec<Instruction>) -> Result<()> {
    match stmt {
        Statement::Expression(expr) => {
            generate_expression_instructions(expr, instructions)?;
        }
        Statement::Assignment { name, value } => {
            generate_expression_instructions(value, instructions)?;
            // Store the value in the appropriate variable
            if name.starts_with('$') {
                // Global variable
                instructions.push(Instruction::StoreGlobal(name.clone()));
            }
            else if name.starts_with('@') {
                // Instance variable
                instructions.push(Instruction::StoreInstance(name.clone()));
            }
            else if name.starts_with("@@") {
                // Class variable
                instructions.push(Instruction::StoreClass(name.clone()));
            }
            else {
                // Local variable (using index 0 for simplicity)
                instructions.push(Instruction::StoreLocal(0));
            }
        }
        Statement::GlobalAssignment { name, value } => {
            generate_expression_instructions(value, instructions)?;
            instructions.push(Instruction::StoreGlobal(name.clone()));
        }
        Statement::InstanceAssignment { name, value } => {
            generate_expression_instructions(value, instructions)?;
            instructions.push(Instruction::StoreInstance(name.clone()));
        }
        Statement::ClassAssignment { name, value } => {
            generate_expression_instructions(value, instructions)?;
            instructions.push(Instruction::StoreClass(name.clone()));
        }
        Statement::MethodDefinition { name: _, parameters: _, body: _ } => {
            // For now, we'll just generate a placeholder
            // In a real implementation, we'd generate instructions for the method body
            // and register the method with the VM
        }
        Statement::ClassDefinition { name: _, superclass: _, body: _ } => {
            // For now, we'll just generate a placeholder
        }
        Statement::If { condition, then_branch, else_branch } => {
            generate_expression_instructions(condition, instructions)?;

            // Calculate jump offsets
            let else_offset = then_branch.len() as i32 + 1; // +1 for the jump instruction
            let end_offset = else_branch.len() as i32 + 1; // +1 for the jump instruction

            // Jump to else branch if condition is false
            instructions.push(Instruction::JumpIfFalse(else_offset));

            // Generate then branch instructions
            for stmt in then_branch {
                generate_statement_instructions(stmt, instructions)?;
            }

            // Jump to end if then branch is taken
            instructions.push(Instruction::Jump(end_offset));

            // Generate else branch instructions
            for stmt in else_branch {
                generate_statement_instructions(stmt, instructions)?;
            }
        }
        Statement::While { condition, body } => {
            let loop_start = instructions.len() as i32;

            // Generate condition instructions
            generate_expression_instructions(condition, instructions)?;

            // Calculate jump offset
            let end_offset = body.len() as i32 + 1; // +1 for the jump instruction

            // Jump to end if condition is false
            instructions.push(Instruction::JumpIfFalse(end_offset));

            // Generate body instructions
            for stmt in body {
                generate_statement_instructions(stmt, instructions)?;
            }

            // Jump back to loop start
            let loop_offset = (loop_start - instructions.len() as i32) - 1; // -1 to account for the current instruction
            instructions.push(Instruction::Jump(loop_offset));
        }
        Statement::Until { condition, body } => {
            let loop_start = instructions.len() as i32;

            // Generate condition instructions
            generate_expression_instructions(condition, instructions)?;

            // Calculate jump offset
            let end_offset = body.len() as i32 + 1; // +1 for the jump instruction

            // Jump to end if condition is true (opposite of while)
            instructions.push(Instruction::JumpIfTrue(end_offset));

            // Generate body instructions
            for stmt in body {
                generate_statement_instructions(stmt, instructions)?;
            }

            // Jump back to loop start
            let loop_offset = (loop_start - instructions.len() as i32) - 1; // -1 to account for the current instruction
            instructions.push(Instruction::Jump(loop_offset));
        }
        Statement::Case { value, when_clauses, else_clause } => {
            // Generate value expression
            generate_expression_instructions(value, instructions)?;

            // Save value to register 1
            instructions.push(Instruction::Move(0, 1));

            let total_branches = when_clauses.len();
            let mut current_branch = 0;

            for (cond, branch_body) in when_clauses {
                // Generate condition expression
                generate_expression_instructions(cond, instructions)?;

                // Compare with saved value
                instructions.push(Instruction::Move(0, 2));
                instructions.push(Instruction::Move(1, 0));
                instructions.push(Instruction::Eq);

                // Calculate jump offset if condition is false
                let branch_size = branch_body.len() as i32 + 1; // +1 for the jump instruction
                let remaining_branches = (total_branches - current_branch - 1) as i32;
                let else_size = else_clause.len() as i32;
                let jump_offset = branch_size + remaining_branches * (branch_size + 1) + else_size;

                // Jump to next condition if not equal
                instructions.push(Instruction::JumpIfFalse(jump_offset as i32));

                // Generate branch body instructions
                for stmt in branch_body {
                    generate_statement_instructions(stmt, instructions)?;
                }

                // Jump to end after executing branch
                let end_offset = (remaining_branches * (branch_size + 1) + else_size) as i32;
                instructions.push(Instruction::Jump(end_offset));

                current_branch += 1;
            }

            // Generate else clause instructions
            for stmt in else_clause {
                generate_statement_instructions(stmt, instructions)?;
            }
        }
        Statement::For { variable: _, iterator, body } => {
            // For now, we'll generate a while loop equivalent
            // This is a simplified implementation
            let loop_start = instructions.len() as i32;

            // Generate iterator expression (assuming it's an array)
            generate_expression_instructions(iterator, instructions)?;

            // Check if iterator is empty (simplified)
            instructions.push(Instruction::LoadConst(RubyValue::Integer(0)));
            instructions.push(Instruction::Eq);

            // Calculate jump offset
            let end_offset = body.len() as i32 + 2; // +2 for the jump back and the current instruction

            // Jump to end if iterator is empty
            instructions.push(Instruction::JumpIfTrue(end_offset));

            // Generate body instructions
            for stmt in body {
                generate_statement_instructions(stmt, instructions)?;
            }

            // Jump back to loop start
            let loop_offset = (loop_start - instructions.len() as i32) - 1; // -1 to account for the current instruction
            instructions.push(Instruction::Jump(loop_offset));
        }
        Statement::Break => {
            // Generate a jump instruction to break out of the loop
            // Note: This is a simplified implementation
            // In a real implementation, we would need to track loop boundaries
            instructions.push(Instruction::Jump(100)); // Jump a large offset to break out of most loops
        }
        Statement::Next => {
            // Generate a jump instruction to skip to the next iteration
            // Note: This is a simplified implementation
            instructions.push(Instruction::Jump(50)); // Jump to the end of the current loop iteration
        }
        Statement::Redo => {
            // Generate a jump instruction to redo the current iteration
            // Note: This is a simplified implementation
            instructions.push(Instruction::Jump(-50)); // Jump back to the start of the loop
        }
        Statement::Return(value) => {
            if let Some(expr) = value {
                generate_expression_instructions(expr, instructions)?;
            }
            else {
                instructions.push(Instruction::Nil);
            }
            instructions.push(Instruction::Return);
        }
        _ => {
            // For other statement types, we'll just generate a placeholder
        }
    }

    Ok(())
}

/// Generate instructions for an expression
fn generate_expression_instructions(expr: &Expression, instructions: &mut Vec<Instruction>) -> Result<()> {
    match expr {
        Expression::Variable(name) => {
            if name.starts_with('$') {
                // Global variable
                instructions.push(Instruction::LoadGlobal(name.clone()));
            }
            else if name.starts_with('@') {
                // Instance variable
                instructions.push(Instruction::LoadInstance(name.clone()));
            }
            else if name.starts_with("@@") {
                // Class variable
                instructions.push(Instruction::LoadClass(name.clone()));
            }
            else {
                // Local variable (using index 0 for simplicity)
                instructions.push(Instruction::LoadLocal(0));
            }
        }
        Expression::GlobalVariable(name) => {
            instructions.push(Instruction::LoadGlobal(name.clone()));
        }
        Expression::InstanceVariable(name) => {
            instructions.push(Instruction::LoadInstance(name.clone()));
        }
        Expression::ClassVariable(name) => {
            instructions.push(Instruction::LoadClass(name.clone()));
        }
        Expression::Literal(value) => {
            instructions.push(Instruction::LoadConst(value.clone()));
        }
        Expression::MethodCall { receiver, method, arguments } => {
            // Generate receiver instructions
            generate_expression_instructions(receiver, instructions)?;

            // Generate arguments instructions
            for arg in arguments {
                generate_expression_instructions(arg, instructions)?;
            }

            // Call the method
            instructions.push(Instruction::CallMethod(method.clone(), arguments.len()));
        }
        Expression::BinaryOp { left, op, right } => {
            // Generate left operand instructions
            generate_expression_instructions(left, instructions)?;

            // Save left value to register 1
            instructions.push(Instruction::Move(0, 1));

            // Generate right operand instructions
            generate_expression_instructions(right, instructions)?;

            // Perform the operation
            match op {
                BinaryOperator::Add => instructions.push(Instruction::Add),
                BinaryOperator::Sub => instructions.push(Instruction::Sub),
                BinaryOperator::Mul => instructions.push(Instruction::Mul),
                BinaryOperator::Div => instructions.push(Instruction::Div),
                BinaryOperator::Mod => instructions.push(Instruction::Mod),
                BinaryOperator::Exp => instructions.push(Instruction::Exp),
                BinaryOperator::Eq => instructions.push(Instruction::Eq),
                BinaryOperator::Neq => instructions.push(Instruction::Neq),
                BinaryOperator::Lt => instructions.push(Instruction::Lt),
                BinaryOperator::Lte => instructions.push(Instruction::Lte),
                BinaryOperator::Gt => instructions.push(Instruction::Gt),
                BinaryOperator::Gte => instructions.push(Instruction::Gte),
                BinaryOperator::And => instructions.push(Instruction::And),
                BinaryOperator::Or => instructions.push(Instruction::Or),
                BinaryOperator::Assign => {
                    // Assignment is handled at the statement level
                }
            }
        }
        Expression::UnaryOp { op, operand } => {
            // Generate operand instructions
            generate_expression_instructions(operand, instructions)?;

            // Perform the operation
            match op {
                UnaryOperator::Not => instructions.push(Instruction::Not),
                // For other unary operators, we'd need to implement them
                _ => {}
            }
        }
        Expression::ArrayLiteral(elements) => {
            // Generate elements instructions
            for elem in elements {
                generate_expression_instructions(elem, instructions)?;
            }

            // Create the array
            instructions.push(Instruction::NewArray(elements.len()));
        }
        Expression::HashLiteral(pairs) => {
            // Generate key-value pairs instructions
            for (key, value) in pairs {
                // Generate key instructions (key is already a string)
                instructions.push(Instruction::LoadConst(RubyValue::String(key.clone())));
                // Generate value instructions
                generate_expression_instructions(value, instructions)?;
            }

            // Create the hash
            instructions.push(Instruction::NewHash(pairs.len()));
        }
        Expression::SelfRef => {
            // For self reference, we'd need to implement it
        }
        Expression::Block { parameters: _, body: _ } => {
            // For now, we'll just generate a placeholder
        }
        Expression::SuperCall { arguments: _ } => {
            // For now, we'll just generate a placeholder
        }
        _ => {
            // For other expression types, we'll just generate a placeholder
        }
    }

    Ok(())
}
