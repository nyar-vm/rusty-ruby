//! 热点代码检测器
//!
//! 负责检测代码的执行频率，识别热点代码，并触发编译。

use std::{collections::HashMap, sync::Mutex};

/// 代码特性
#[derive(Debug, Default, Clone)]
pub struct CodeFeatures {
    /// 算术操作指令数量
    pub arithmetic_ops: usize,
    /// 条件分支指令数量
    pub branch_ops: usize,
    /// 函数调用指令数量
    pub call_ops: usize,
    /// 循环检测标志
    pub has_loop: bool,
    /// 指令总数量
    pub total_instructions: usize,
}

/// 热点检测器
///
/// 追踪代码块的执行次数，分析代码特性，当达到阈值时标记为热点代码。
pub struct HotnessDetector {
    /// 执行计数器
    execution_counts: Mutex<HashMap<usize, usize>>,
    /// 代码特性缓存
    code_features: Mutex<HashMap<usize, CodeFeatures>>,
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
            code_features: Mutex::new(HashMap::new()),
            baseline_threshold: 10,    // 执行10次后进行基线编译
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

        let (baseline_threshold, optimizing_threshold) = self.get_adjusted_thresholds(code_id);
        let need_baseline = *count == baseline_threshold;
        let need_optimizing = *count == optimizing_threshold;

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

    /// 缓存代码特性
    ///
    /// # 参数
    /// - `code_id`: 代码块的唯一标识符
    /// - `features`: 代码特性
    pub fn cache_code_features(&self, code_id: usize, features: CodeFeatures) {
        let mut features_cache = self.code_features.lock().unwrap();
        features_cache.insert(code_id, features);
    }

    /// 获取代码特性
    ///
    /// # 参数
    /// - `code_id`: 代码块的唯一标识符
    ///
    /// # 返回值
    /// - `Option<&CodeFeatures>`: 代码特性，如果存在的话
    pub fn get_code_features(&self, code_id: usize) -> Option<CodeFeatures> {
        let features_cache = self.code_features.lock().unwrap();
        features_cache.get(&code_id).cloned()
    }

    /// 根据代码特性调整编译阈值
    ///
    /// # 参数
    /// - `code_id`: 代码块的唯一标识符
    ///
    /// # 返回值
    /// - `(usize, usize)`: 调整后的基线编译阈值和优化编译阈值
    pub fn get_adjusted_thresholds(&self, code_id: usize) -> (usize, usize) {
        if let Some(features) = self.get_code_features(code_id) {
            // 根据代码特性调整阈值
            let mut baseline_threshold = self.baseline_threshold;
            let mut optimizing_threshold = self.optimizing_threshold;

            // 对于计算密集型代码，降低编译阈值
            if features.arithmetic_ops > features.total_instructions / 2 {
                baseline_threshold = baseline_threshold / 2;
                optimizing_threshold = optimizing_threshold / 2;
            }

            // 对于有循环的代码，降低编译阈值
            if features.has_loop {
                baseline_threshold = baseline_threshold / 2;
                optimizing_threshold = optimizing_threshold / 2;
            }

            // 对于方法调用密集型代码，降低编译阈值
            if features.call_ops > features.total_instructions / 3 {
                baseline_threshold = baseline_threshold / 2;
                optimizing_threshold = optimizing_threshold / 2;
            }

            // 对于分支密集型代码，降低编译阈值
            if features.branch_ops > features.total_instructions / 3 {
                baseline_threshold = baseline_threshold / 2;
                optimizing_threshold = optimizing_threshold / 2;
            }

            // 对于小型代码块，提高编译阈值
            if features.total_instructions < 5 {
                baseline_threshold = baseline_threshold * 2;
                optimizing_threshold = optimizing_threshold * 2;
            }

            // 确保阈值不小于最小阈值
            baseline_threshold = baseline_threshold.max(5);
            optimizing_threshold = optimizing_threshold.max(50);

            (baseline_threshold, optimizing_threshold)
        }
        else {
            (self.baseline_threshold, self.optimizing_threshold)
        }
    }
}
