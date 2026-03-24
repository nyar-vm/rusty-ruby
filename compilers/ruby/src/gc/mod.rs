//! 垃圾收集系统
//!
//! 实现分代垃圾收集器，支持增量收集和并发收集。

pub mod concurrent;
pub mod generational;
pub mod incremental;

use ruby_types::RubyValue;

/// 垃圾收集器特性
pub trait GC {
    /// 分配新对象
    fn allocate(&mut self, value: RubyValue) -> RubyValue;
    /// 执行垃圾收集
    fn collect(&mut self, roots: &[&RubyValue]);
    /// 获取当前内存使用量
    fn memory_used(&self) -> usize;
    /// 设置内存阈值
    fn set_threshold(&mut self, threshold: usize);
}

/// 垃圾收集器类型
pub enum GCType {
    /// 分代垃圾收集器
    Generational,
    /// 增量垃圾收集器
    Incremental,
    /// 并发垃圾收集器
    Concurrent,
}

/// 创建垃圾收集器
///
/// # 参数
/// - `gc_type`：垃圾收集器类型
///
/// # 返回值
/// - `Box<dyn GC>`：垃圾收集器实例
pub fn create_gc(gc_type: GCType) -> Box<dyn GC> {
    match gc_type {
        GCType::Generational => Box::new(generational::GenerationalGC::new()),
        GCType::Incremental => Box::new(incremental::IncrementalGC::new()),
        GCType::Concurrent => Box::new(concurrent::ConcurrentGC::new()),
    }
}
