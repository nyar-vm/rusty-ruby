//! 可视化工具
//! 
//! 用于可视化编译和优化过程，包括代码流程、优化效果等。

use crate::architecture::{execution_engine::CompilationState, hotness_detector::HotnessDetector};
use std::collections::HashMap;

/// 编译状态的简化表示
#[derive(Debug, Clone)]
pub enum SimplifiedCompilationState {
    /// 未编译，使用解释器
    Interpreted,
    /// 已基线编译
    BaselineCompiled,
    /// 已优化编译
    OptimizedCompiled,
}

/// 可视化数据
#[derive(Debug, Clone)]
pub struct VisualizationData {
    /// 函数编译状态
    pub function_states: HashMap<String, SimplifiedCompilationState>,
    /// 函数执行次数
    pub function_executions: HashMap<String, usize>,
    /// 优化事件
    pub optimization_events: Vec<String>,
    /// 热点代码信息
    pub hotspots: Vec<(String, usize)>,
}

/// 可视化工具
/// 
/// 用于可视化编译和优化过程。
pub struct Visualizer {
    /// 可视化数据
    data: VisualizationData,
}

impl Visualizer {
    /// 创建新的可视化工具
    pub fn new() -> Self {
        Self {
            data: VisualizationData {
                function_states: HashMap::new(),
                function_executions: HashMap::new(),
                optimization_events: Vec::new(),
                hotspots: Vec::new(),
            },
        }
    }

    /// 记录函数编译状态
    pub fn record_function_state(&mut self, function_name: &str, state: &CompilationState) {
        let simplified_state = match state {
            CompilationState::Interpreted => SimplifiedCompilationState::Interpreted,
            CompilationState::BaselineCompiled(_) => SimplifiedCompilationState::BaselineCompiled,
            CompilationState::OptimizedCompiled(_) => SimplifiedCompilationState::OptimizedCompiled,
        };
        self.data.function_states.insert(function_name.to_string(), simplified_state);
    }

    /// 记录函数执行次数
    pub fn record_function_execution(&mut self, function_name: &str) {
        *self.data.function_executions.entry(function_name.to_string()).or_insert(0) += 1;
    }

    /// 记录优化事件
    pub fn record_optimization(&mut self, event: &str) {
        self.data.optimization_events.push(event.to_string());
    }

    /// 记录热点代码
    pub fn record_hotspot(&mut self, function_name: &str, execution_count: usize) {
        self.data.hotspots.push((function_name.to_string(), execution_count));
    }

    /// 生成编译状态可视化
    pub fn generate_compilation_visualization(&self) -> String {
        let mut visualization = String::new();

        visualization.push_str("=== Compilation State Visualization ===\n\n");

        for (function, state) in &self.data.function_states {
            visualization.push_str(&format!("Function: {}\n", function));
            visualization.push_str(&format!("State: {:?}\n", state));
            visualization.push_str(&format!("Executions: {}\n\n", self.data.function_executions.get(function).unwrap_or(&0)));
        }

        visualization
    }

    /// 生成优化事件可视化
    pub fn generate_optimization_visualization(&self) -> String {
        let mut visualization = String::new();

        visualization.push_str("=== Optimization Events Visualization ===\n\n");

        for (i, event) in self.data.optimization_events.iter().enumerate() {
            visualization.push_str(&format!("{}. {}\n", i + 1, event));
        }

        visualization
    }

    /// 生成热点代码可视化
    pub fn generate_hotspot_visualization(&self) -> String {
        let mut visualization = String::new();

        visualization.push_str("=== Hotspot Visualization ===\n\n");

        // 按执行次数排序
        let mut sorted_hotspots = self.data.hotspots.clone();
        sorted_hotspots.sort_by(|a, b| b.1.cmp(&a.1));

        for (function, count) in sorted_hotspots {
            visualization.push_str(&format!("Function: {} - {} executions\n", function, count));
        }

        visualization
    }

    /// 生成完整的可视化报告
    pub fn generate_full_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Ruby Compiler Visualization Report ===\n\n");
        report.push_str(&self.generate_compilation_visualization());
        report.push_str(&self.generate_optimization_visualization());
        report.push_str(&self.generate_hotspot_visualization());

        report
    }

    /// 重置可视化数据
    pub fn reset(&mut self) {
        self.data = VisualizationData {
            function_states: HashMap::new(),
            function_executions: HashMap::new(),
            optimization_events: Vec::new(),
            hotspots: Vec::new(),
        };
    }

    /// 获取可视化数据
    pub fn get_data(&self) -> &VisualizationData {
        &self.data
    }
}
