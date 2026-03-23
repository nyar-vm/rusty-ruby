//! IR optimization utilities

use crate::{BinaryOperator, Expression, Program, Statement, UnaryOperator, traversal::Mutator};
use ruby_types::RubyValue;

/// Constant folder for expressions
pub struct ConstantFolder;

impl Mutator for ConstantFolder {
    fn mutate_expression(&mut self, expr: &mut Expression) {
        // First mutate sub-expressions
        match expr {
            Expression::MethodCall { receiver, method: _, arguments } => {
                self.mutate_expression(receiver);
                for arg in arguments {
                    self.mutate_expression(arg);
                }
            }
            Expression::BinaryOp { left, op, right } => {
                self.mutate_expression(left);
                self.mutate_expression(right);

                // Try to fold binary operations with constant operands
                if let (Expression::Literal(left_val), Expression::Literal(right_val)) = (left.as_ref(), right.as_ref()) {
                    if let Some(result) = self.evaluate_binary_op(op.clone(), left_val, right_val) {
                        *expr = Expression::Literal(result);
                    }
                }
            }
            Expression::UnaryOp { op, operand } => {
                self.mutate_expression(operand);

                // Try to fold unary operations with constant operands
                if let Expression::Literal(val) = operand.as_ref() {
                    if let Some(result) = self.evaluate_unary_op(op.clone(), val) {
                        *expr = Expression::Literal(result);
                    }
                }
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
            Expression::SuperCall { arguments } => {
                for arg in arguments {
                    self.mutate_expression(arg);
                }
            }
            _ => {}
        }
    }

    fn mutate_statement(&mut self, stmt: &mut Statement) {
        // First mutate sub-expressions
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

                // Try to fold if statements with constant conditions
                if let Expression::Literal(val) = condition {
                    if val.to_bool() {
                        // If condition is true, replace with then branch
                        let mut new_body = Vec::new();
                        new_body.extend(then_branch.iter().cloned());
                        *stmt = Statement::Expression(Expression::Literal(RubyValue::Nil));
                        // TODO: Replace with the then branch statements
                    }
                    else {
                        // If condition is false, replace with else branch
                        let mut new_body = Vec::new();
                        new_body.extend(else_branch.iter().cloned());
                        *stmt = Statement::Expression(Expression::Literal(RubyValue::Nil));
                        // TODO: Replace with the else branch statements
                    }
                }
                else {
                    for stmt in then_branch {
                        self.mutate_statement(stmt);
                    }
                    for stmt in else_branch {
                        self.mutate_statement(stmt);
                    }
                }
            }
            Statement::While { condition, body } => {
                self.mutate_expression(condition);

                // Try to fold while statements with constant conditions
                if let Expression::Literal(val) = condition {
                    if !val.to_bool() {
                        // If condition is false, remove the loop
                        *stmt = Statement::Expression(Expression::Literal(RubyValue::Nil));
                    }
                }
                else {
                    for stmt in body {
                        self.mutate_statement(stmt);
                    }
                }
            }
            Statement::For { variable: _, iterator, body } => {
                self.mutate_expression(iterator);
                for stmt in body {
                    self.mutate_statement(stmt);
                }
            }
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
            _ => {}
        }
    }
}

impl ConstantFolder {
    /// Evaluate a binary operation with constant operands
    fn evaluate_binary_op(&self, op: BinaryOperator, left: &RubyValue, right: &RubyValue) -> Option<RubyValue> {
        match op {
            BinaryOperator::Add => Some(RubyValue::Float(left.to_f64() + right.to_f64())),
            BinaryOperator::Sub => Some(RubyValue::Float(left.to_f64() - right.to_f64())),
            BinaryOperator::Mul => Some(RubyValue::Float(left.to_f64() * right.to_f64())),
            BinaryOperator::Div => {
                if right.to_f64() != 0.0 {
                    Some(RubyValue::Float(left.to_f64() / right.to_f64()))
                }
                else {
                    None // Division by zero
                }
            }
            BinaryOperator::Mod => Some(RubyValue::Float(left.to_f64() % right.to_f64())),
            BinaryOperator::Exp => Some(RubyValue::Float(left.to_f64().powf(right.to_f64()))),
            BinaryOperator::Eq => Some(RubyValue::Boolean(left == right)),
            BinaryOperator::Neq => Some(RubyValue::Boolean(left != right)),
            BinaryOperator::Lt => Some(RubyValue::Boolean(left.to_f64() < right.to_f64())),
            BinaryOperator::Lte => Some(RubyValue::Boolean(left.to_f64() <= right.to_f64())),
            BinaryOperator::Gt => Some(RubyValue::Boolean(left.to_f64() > right.to_f64())),
            BinaryOperator::Gte => Some(RubyValue::Boolean(left.to_f64() >= right.to_f64())),
            BinaryOperator::And => Some(RubyValue::Boolean(left.to_bool() && right.to_bool())),
            BinaryOperator::Or => Some(RubyValue::Boolean(left.to_bool() || right.to_bool())),
            BinaryOperator::Assign => {
                None // Assignment can't be folded
            }
        }
    }

    /// Evaluate a unary operation with constant operands
    fn evaluate_unary_op(&self, op: UnaryOperator, operand: &RubyValue) -> Option<RubyValue> {
        match op {
            UnaryOperator::Not => Some(RubyValue::Boolean(!operand.to_bool())),
            UnaryOperator::Neg => Some(RubyValue::Float(-operand.to_f64())),
            UnaryOperator::Plus => Some(RubyValue::Float(operand.to_f64())),
            UnaryOperator::BitNot => Some(RubyValue::Integer(!operand.to_i32())),
        }
    }
}

/// Dead code eliminator
pub struct DeadCodeEliminator;

impl Mutator for DeadCodeEliminator {
    fn mutate_statement(&mut self, stmt: &mut Statement) {
        // First mutate sub-expressions and statements
        match stmt {
            Statement::Expression(expr) => {
                self.mutate_expression(expr);
                // If the expression is pure, replace it with a nil expression
                if self.is_pure_expression(expr) {
                    *stmt = Statement::Expression(Expression::Literal(RubyValue::Nil));
                }
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

                // Mutate then branch
                let mut new_then_branch = Vec::new();
                for s in &mut *then_branch {
                    let mut s = s.clone();
                    self.mutate_statement(&mut s);
                    // Only keep non-dead statements
                    if !self.is_dead_statement(&s) {
                        new_then_branch.push(s);
                    }
                }
                *then_branch = new_then_branch;

                // Mutate else branch
                let mut new_else_branch = Vec::new();
                for s in &mut *else_branch {
                    let mut s = s.clone();
                    self.mutate_statement(&mut s);
                    // Only keep non-dead statements
                    if !self.is_dead_statement(&s) {
                        new_else_branch.push(s);
                    }
                }
                *else_branch = new_else_branch;
            }
            Statement::While { condition, body } => {
                self.mutate_expression(condition);

                // Mutate body
                let mut new_body = Vec::new();
                for s in &mut *body {
                    let mut s = s.clone();
                    self.mutate_statement(&mut s);
                    // Only keep non-dead statements
                    if !self.is_dead_statement(&s) {
                        new_body.push(s);
                    }
                }
                *body = new_body;
            }
            Statement::For { variable: _, iterator, body } => {
                self.mutate_expression(iterator);

                // Mutate body
                let mut new_body = Vec::new();
                for s in &mut *body {
                    let mut s = s.clone();
                    self.mutate_statement(&mut s);
                    // Only keep non-dead statements
                    if !self.is_dead_statement(&s) {
                        new_body.push(s);
                    }
                }
                *body = new_body;
            }
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    self.mutate_expression(expr);
                }
            }
            Statement::MethodDefinition { name: _, parameters: _, body } => {
                // Mutate body
                let mut new_body = Vec::new();
                for s in &mut *body {
                    let mut s = s.clone();
                    self.mutate_statement(&mut s);
                    // Only keep non-dead statements
                    if !self.is_dead_statement(&s) {
                        new_body.push(s);
                    }
                }
                *body = new_body;
            }
            Statement::ClassDefinition { name: _, superclass: _, body } => {
                // Mutate body
                let mut new_body = Vec::new();
                for s in &mut *body {
                    let mut s = s.clone();
                    self.mutate_statement(&mut s);
                    // Only keep non-dead statements
                    if !self.is_dead_statement(&s) {
                        new_body.push(s);
                    }
                }
                *body = new_body;
            }
            Statement::ModuleDefinition { name: _, body } => {
                // Mutate body
                let mut new_body = Vec::new();
                for s in &mut *body {
                    let mut s = s.clone();
                    self.mutate_statement(&mut s);
                    // Only keep non-dead statements
                    if !self.is_dead_statement(&s) {
                        new_body.push(s);
                    }
                }
                *body = new_body;
            }
            Statement::Require(expr) => {
                self.mutate_expression(expr);
            }
            Statement::Load(expr) => {
                self.mutate_expression(expr);
            }
            _ => {}
        }
    }
}

impl DeadCodeEliminator {
    /// Check if a statement is dead
    fn is_dead_statement(&self, stmt: &Statement) -> bool {
        match stmt {
            // Expression statements with no side effects are dead
            Statement::Expression(expr) => self.is_pure_expression(expr),
            _ => {
                false // Other statements have side effects
            }
        }
    }

    /// Check if an expression is pure (has no side effects)
    fn is_pure_expression(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Literal(_) => true,
            Expression::Variable(_) => true,
            Expression::GlobalVariable(_) => false,   // Global variables have side effects
            Expression::InstanceVariable(_) => false, // Instance variables have side effects
            Expression::ClassVariable(_) => false,    // Class variables have side effects
            Expression::MethodCall { .. } => false,   // Method calls may have side effects
            Expression::BinaryOp { left, right, .. } => self.is_pure_expression(left) && self.is_pure_expression(right),
            Expression::UnaryOp { operand, .. } => self.is_pure_expression(operand),
            Expression::ArrayLiteral(elements) => elements.iter().all(|e| self.is_pure_expression(e)),
            Expression::HashLiteral(pairs) => pairs.values().all(|e| self.is_pure_expression(e)),
            Expression::Block { .. } => false, // Blocks may have side effects
            Expression::SelfRef => true,
            Expression::SuperCall { .. } => false, // Super calls may have side effects
        }
    }
}

/// Optimize a program
pub fn optimize_program(program: &mut Program) {
    // Apply constant folding
    let mut constant_folder = ConstantFolder;
    constant_folder.mutate_program(program);

    // Apply dead code elimination
    let mut dead_code_eliminator = DeadCodeEliminator;
    dead_code_eliminator.mutate_program(program);
}
