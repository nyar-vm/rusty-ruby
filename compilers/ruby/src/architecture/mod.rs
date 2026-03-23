//! 多层级编译架构
//! 
//! 实现类似 V8 的多层级编译架构，包括解释器、基线编译器和优化编译器。

pub mod interpreter;
pub mod baseline_compiler;
pub mod optimizing_compiler;
pub mod execution_engine;
pub mod hotness_detector;