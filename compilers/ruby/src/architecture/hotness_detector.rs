//! 热点代码检测器
//! 
//! 负责检测代码的执行频率，识别热点代码，并触发编译。

use std::collections::HashMap;
use std::sync::Mutex;

/// 热点检测器
/// 
/// 追踪代码块的执行次数，当达到阈值时标记为热点代码。
pub struct HotnessDetector {
    /// 执行计数器
    execution_counts: Mutex<HashMap<usize, usize>>,
    /// 基线编译阈值
    baseline_threshold: usize,
    /// 优化编译阈值
    optimizing_threshold: usize,
}

impl HotnessDetector {
    /// 创建新的热点检测器
    pub fn new() -> Self {
        Self {
            execution_counts: Mutex::new(HashMap::new()),
            baseline_threshold: 10,  // 执行10次后进行基线编译
            optimizing_threshold: 100, // 执行100次后进行优化编译
        }
    }

    /// 增加执行计数
    /// 
    /// # 参数
    /// - `code_id`: 代码块的唯一标识符
    /// 
    /// # 返回值
    /// - `(bool, bool)`: 是否需要基线编译，是否需要优化编译
    pub fn increment_count(&self, code_id: usize) -> (bool, bool) {
        let mut counts = self.execution_counts.lock().unwrap();
        let count = counts.entry(code_id).or_insert(0);
        *count += 1;

        let need_baseline = *count == self.baseline_threshold;
        let need_optimizing = *count == self.optimizing_threshold;

        (need_baseline, need_optimizing)
    }

    /// 获取执行计数
    pub fn get_count(&self, code_id: usize) -> usize {
        let counts = self.execution_counts.lock().unwrap();
        *counts.get(&code_id).unwrap_or(&0)
    }

    /// 重置执行计数
    pub fn reset_count(&self, code_id: usize) {
        let mut counts = self.execution_counts.lock().unwrap();
        counts.remove(&code_id);
    }
}