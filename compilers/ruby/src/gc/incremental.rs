//! 增量垃圾收集器
//! 
//! 实现增量垃圾收集策略，将垃圾收集过程分成多个小步骤，减少暂停时间。

use ruby_types::RubyValue;
use std::collections::HashSet;

/// 增量垃圾收集器的状态
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum IncrementalGCState {
    /// 空闲状态
    Idle,
    /// 标记阶段
    Marking,
    /// 清除阶段
    Sweeping,
    /// 完成阶段
    Completed,
}

/// 增量垃圾收集器
/// 
/// 将垃圾收集过程分成多个小步骤，在应用程序执行的间隙进行。
pub struct IncrementalGC {
    /// 已分配的对象
    objects: Vec<Box<RubyValue>>,
    /// 内存使用阈值
    threshold: usize,
    /// 当前内存使用量
    memory_used: usize,
    /// 垃圾收集状态
    state: IncrementalGCState,
    /// 已标记对象的集合
    marked: HashSet<*const RubyValue>,
    /// 标记阶段的当前索引
    mark_index: usize,
    /// 清除阶段的当前索引
    sweep_index: usize,
    /// 是否启用日志
    enable_logging: bool,
}

impl IncrementalGC {
    /// 创建新的增量垃圾收集器
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            threshold: 1024 * 1024, // 1MB 阈值
            memory_used: 0,
            state: IncrementalGCState::Idle,
            marked: HashSet::new(),
            mark_index: 0,
            sweep_index: 0,
            enable_logging: false,
        }
    }

    /// 分配新对象
    /// 
    /// # 参数
    /// - `value`：要分配的 Ruby 对象
    /// 
    /// # 返回值
    /// - `RubyValue`：分配的对象
    pub fn allocate(&mut self, value: RubyValue) -> RubyValue {
        // 暂时直接返回值，后续需要修改为通过 GC 分配
        value
    }

    /// 标记根对象
    /// 
    /// # 参数
    /// - `roots`：根对象列表
    fn mark_roots(&mut self, roots: &[&RubyValue]) {
        for root in roots {
            root.mark(&mut self.marked);
        }
    }

    /// 执行增量标记步骤
    /// 
    /// # 参数
    /// - `roots`：根对象列表
    /// - `step_size`：步骤大小
    /// 
    /// # 返回值
    /// - `bool`：标记阶段是否完成
    fn incremental_mark(&mut self, roots: &[&RubyValue], step_size: usize) -> bool {
        // 第一次标记时标记根对象
        if self.mark_index == 0 {
            self.mark_roots(roots);
        }

        // 执行标记步骤
        let start_index = self.mark_index;
        let end_index = (start_index + step_size).min(self.objects.len());

        for i in start_index..end_index {
            let object = &self.objects[i];
            let obj_ptr = object.as_ref() as *const RubyValue;
            if self.marked.contains(&obj_ptr) {
                // 标记对象的引用
                object.mark(&mut self.marked);
            }
        }

        self.mark_index = end_index;
        self.mark_index >= self.objects.len()
    }

    /// 执行增量清除步骤
    /// 
    /// # 参数
    /// - `step_size`：步骤大小
    /// 
    /// # 返回值
    /// - `bool`：清除阶段是否完成
    fn incremental_sweep(&mut self, step_size: usize) -> bool {
        let start_index = self.sweep_index;
        let end_index = (start_index + step_size).min(self.objects.len());

        let mut new_objects = Vec::new();
        
        // 保留未处理的对象
        for i in 0..start_index {
            new_objects.push(self.objects[i].clone());
        }

        // 处理当前步骤的对象
        for i in start_index..end_index {
            let object = &self.objects[i];
            let obj_ptr = object.as_ref() as *const RubyValue;
            if self.marked.contains(&obj_ptr) {
                new_objects.push(object.clone());
            } else {
                // 对象被回收
                self.memory_used -= 1;
            }
        }

        // 保留未处理的对象
        for i in end_index..self.objects.len() {
            new_objects.push(self.objects[i].clone());
        }

        self.objects = new_objects;
        self.sweep_index = end_index;
        self.sweep_index >= self.objects.len()
    }

    /// 执行一次增量垃圾收集步骤
    /// 
    /// # 参数
    /// - `roots`：根对象列表
    /// - `step_size`：步骤大小
    pub fn collect_step(&mut self, roots: &[&RubyValue], step_size: usize) {
        match self.state {
            IncrementalGCState::Idle => {
                // 开始垃圾收集
                if self.enable_logging {
                    println!("[GC] Starting incremental garbage collection...");
                    println!("[GC] Objects before: {}", self.objects.len());
                }
                
                // 重置状态
                self.marked.clear();
                self.mark_index = 0;
                self.sweep_index = 0;
                self.state = IncrementalGCState::Marking;
            }
            IncrementalGCState::Marking => {
                // 执行标记步骤
                let mark_complete = self.incremental_mark(roots, step_size);
                if mark_complete {
                    if self.enable_logging {
                        println!("[GC] Marking phase completed");
                    }
                    self.state = IncrementalGCState::Sweeping;
                }
            }
            IncrementalGCState::Sweeping => {
                // 执行清除步骤
                let sweep_complete = self.incremental_sweep(step_size);
                if sweep_complete {
                    if self.enable_logging {
                        println!("[GC] Sweeping phase completed");
                        println!("[GC] Objects after: {}", self.objects.len());
                        println!("[GC] Memory used: {}", self.memory_used);
                    }
                    self.state = IncrementalGCState::Completed;
                }
            }
            IncrementalGCState::Completed => {
                // 完成垃圾收集
                if self.enable_logging {
                    println!("[GC] Incremental garbage collection completed");
                }
                self.state = IncrementalGCState::Idle;
            }
        }
    }

    /// 执行全量垃圾收集
    /// 
    /// # 参数
    /// - `roots`：根对象列表
    pub fn collect(&mut self, roots: &[&RubyValue]) {
        // 一次性执行所有步骤
        while self.state != IncrementalGCState::Idle {
            self.collect_step(roots, self.objects.len());
        }
        
        // 开始新的垃圾收集
        self.collect_step(roots, self.objects.len());
        while self.state != IncrementalGCState::Idle {
            self.collect_step(roots, self.objects.len());
        }
    }

    /// 获取当前内存使用量
    /// 
    /// # 返回值
    /// - `usize`：当前内存使用量
    pub fn memory_used(&self) -> usize {
        self.memory_used
    }

    /// 设置内存阈值
    /// 
    /// # 参数
    /// - `threshold`：新的内存阈值
    pub fn set_threshold(&mut self, threshold: usize) {
        self.threshold = threshold;
    }

    /// 启用日志
    /// 
    /// # 参数
    /// - `enable`：是否启用日志
    pub fn set_logging(&mut self, enable: bool) {
        self.enable_logging = enable;
    }

    /// 获取当前状态
    /// 
    /// # 返回值
    /// - `IncrementalGCState`：当前垃圾收集器状态
    pub fn state(&self) -> IncrementalGCState {
        self.state
    }
}

impl crate::gc::GC for IncrementalGC {
    fn allocate(&mut self, value: RubyValue) -> RubyValue {
        self.allocate(value)
    }

    fn collect(&mut self, roots: &[&RubyValue]) {
        self.collect(roots)
    }

    fn memory_used(&self) -> usize {
        self.memory_used()
    }

    fn set_threshold(&mut self, threshold: usize) {
        self.set_threshold(threshold)
    }
}