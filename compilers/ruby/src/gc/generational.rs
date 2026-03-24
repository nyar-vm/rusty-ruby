//! 分代垃圾收集器
//!
//! 实现基于世代的垃圾收集策略，将对象分为年轻代和老年代。

use ruby_types::RubyValue;
use std::collections::HashSet;

/// 代的类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Generation {
    /// 年轻代
    Young,
    /// 老年代
    Old,
}

/// 代的配置
struct GenerationConfig {
    /// 代的类型
    generation: Generation,
    /// 内存阈值
    threshold: usize,
    /// 晋升年龄
    promotion_age: usize,
}

/// 分代垃圾收集器
///
/// 将对象分为年轻代和老年代，不同代使用不同的收集策略。
pub struct GenerationalGC {
    /// 年轻代对象
    young_objects: Vec<Box<RubyValue>>,
    /// 老年代对象
    old_objects: Vec<Box<RubyValue>>,
    /// 年轻代配置
    young_config: GenerationConfig,
    /// 老年代配置
    old_config: GenerationConfig,
    /// 对象年龄
    object_ages: std::collections::HashMap<*const RubyValue, usize>,
    /// 内存使用量
    memory_used: usize,
    /// 是否启用日志
    enable_logging: bool,
}

impl GenerationalGC {
    /// 创建新的分代垃圾收集器
    pub fn new() -> Self {
        Self {
            young_objects: Vec::new(),
            old_objects: Vec::new(),
            young_config: GenerationConfig {
                generation: Generation::Young,
                threshold: 512 * 1024, // 512KB 阈值，增加年轻代容量
                promotion_age: 3,      // 3次收集后晋升，增加年轻代对象停留时间
            },
            old_config: GenerationConfig {
                generation: Generation::Old,
                threshold: 2 * 1024 * 1024, // 2MB 阈值，增加老年代容量
                promotion_age: 0,
            },
            object_ages: std::collections::HashMap::new(),
            memory_used: 0,
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
        // 创建对象的 Box 包装
        let boxed_value = Box::new(value);
        let obj_ptr = boxed_value.as_ref() as *const RubyValue;

        // 将对象添加到年轻代
        self.young_objects.push(boxed_value);
        self.object_ages.insert(obj_ptr, 0);
        self.memory_used += 1;

        // 检查是否需要执行垃圾收集
        if self.memory_used >= self.young_config.threshold {
            // 这里应该触发垃圾收集，但暂时返回值
        }

        // 返回对象的克隆
        *(*self.young_objects.last().unwrap()).clone()
    }

    /// 标记根对象
    ///
    /// # 参数
    /// - `roots`：根对象列表
    /// - `marked`：已标记对象的集合
    fn mark_roots(&self, roots: &[&RubyValue], marked: &mut HashSet<*const RubyValue>) {
        for root in roots {
            root.mark(marked);
        }
    }

    /// 标记年轻代
    ///
    /// # 参数
    /// - `roots`：根对象列表
    ///
    /// # 返回值
    /// - `HashSet<*const RubyValue>`：已标记对象的集合
    fn mark_young(&self, roots: &[&RubyValue]) -> HashSet<*const RubyValue> {
        let mut marked = HashSet::new();
        self.mark_roots(roots, &mut marked);
        marked
    }

    /// 标记老年代
    ///
    /// # 参数
    /// - `roots`：根对象列表
    ///
    /// # 返回值
    /// - `HashSet<*const RubyValue>`：已标记对象的集合
    fn mark_old(&self, roots: &[&RubyValue]) -> HashSet<*const RubyValue> {
        let mut marked = HashSet::new();
        self.mark_roots(roots, &mut marked);
        marked
    }

    /// 清除年轻代
    ///
    /// # 参数
    /// - `marked`：已标记对象的集合
    fn sweep_young(&mut self, marked: &HashSet<*const RubyValue>) {
        let mut new_young_objects = Vec::new();
        let mut promoted_objects = Vec::new();

        for object in self.young_objects.drain(..) {
            let obj_ptr = object.as_ref() as *const RubyValue;
            if marked.contains(&obj_ptr) {
                // 对象仍然被引用
                let age = self.object_ages.get(&obj_ptr).unwrap_or(&0) + 1;
                if age >= self.young_config.promotion_age {
                    // 晋升到老年代
                    promoted_objects.push(object);
                    self.object_ages.remove(&obj_ptr);
                }
                else {
                    // 留在年轻代
                    new_young_objects.push(object);
                    self.object_ages.insert(obj_ptr, age);
                }
            }
            else {
                // 对象被回收
                self.memory_used -= 1;
                self.object_ages.remove(&obj_ptr);
            }
        }

        self.young_objects = new_young_objects;
        self.old_objects.extend(promoted_objects);
    }

    /// 清除老年代
    ///
    /// # 参数
    /// - `marked`：已标记对象的集合
    fn sweep_old(&mut self, marked: &HashSet<*const RubyValue>) {
        let mut new_old_objects = Vec::new();

        for object in self.old_objects.drain(..) {
            let obj_ptr = object.as_ref() as *const RubyValue;
            if marked.contains(&obj_ptr) {
                new_old_objects.push(object);
            }
            else {
                // 对象被回收
                self.memory_used -= 1;
            }
        }

        self.old_objects = new_old_objects;
    }

    /// 执行年轻代垃圾收集
    ///
    /// # 参数
    /// - `roots`：根对象列表
    pub fn collect_young(&mut self, roots: &[&RubyValue]) {
        if self.enable_logging {
            println!("[GC] Starting young generation collection...");
            println!("[GC] Young objects before: {}", self.young_objects.len());
        }

        let marked = self.mark_young(roots);
        self.sweep_young(&marked);

        if self.enable_logging {
            println!("[GC] Young objects after: {}", self.young_objects.len());
            println!("[GC] Old objects after: {}", self.old_objects.len());
            println!("[GC] Memory used: {}", self.memory_used);
        }
    }

    /// 执行老年代垃圾收集
    ///
    /// # 参数
    /// - `roots`：根对象列表
    pub fn collect_old(&mut self, roots: &[&RubyValue]) {
        if self.enable_logging {
            println!("[GC] Starting old generation collection...");
            println!("[GC] Old objects before: {}", self.old_objects.len());
        }

        let marked = self.mark_old(roots);
        self.sweep_old(&marked);

        if self.enable_logging {
            println!("[GC] Old objects after: {}", self.old_objects.len());
            println!("[GC] Memory used: {}", self.memory_used);
        }
    }

    /// 执行全量垃圾收集
    ///
    /// # 参数
    /// - `roots`：根对象列表
    pub fn collect(&mut self, roots: &[&RubyValue]) {
        // 先收集年轻代
        self.collect_young(roots);
        // 再收集老年代
        self.collect_old(roots);
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
    /// - `generation`：代的类型
    /// - `threshold`：新的内存阈值
    pub fn set_threshold(&mut self, generation: Generation, threshold: usize) {
        match generation {
            Generation::Young => self.young_config.threshold = threshold,
            Generation::Old => self.old_config.threshold = threshold,
        }
    }

    /// 启用日志
    ///
    /// # 参数
    /// - `enable`：是否启用日志
    pub fn set_logging(&mut self, enable: bool) {
        self.enable_logging = enable;
    }
}

impl crate::gc::GC for GenerationalGC {
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
        self.set_threshold(Generation::Young, threshold);
        self.set_threshold(Generation::Old, threshold * 4);
    }
}
