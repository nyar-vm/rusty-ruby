//! 并发垃圾收集器
//!
//! 实现并发垃圾收集策略，在后台线程中执行垃圾收集，减少对主线程的影响。

use ruby_types::RubyValue;
use std::{
    collections::HashSet,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

/// Send and Sync wrapper for raw pointers
struct SendablePtr(*const RubyValue);

unsafe impl Send for SendablePtr {}
unsafe impl Sync for SendablePtr {}

impl PartialEq for SendablePtr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for SendablePtr {}

impl std::hash::Hash for SendablePtr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

/// 并发垃圾收集器的状态
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ConcurrentGCState {
    /// 空闲状态
    Idle,
    /// 标记阶段
    Marking,
    /// 清除阶段
    Sweeping,
    /// 完成阶段
    Completed,
}

/// 并发垃圾收集器
///
/// 在后台线程中执行垃圾收集，减少对主线程的影响。
pub struct ConcurrentGC {
    /// 已分配的对象
    objects: Arc<Mutex<Vec<Box<RubyValue>>>>,
    /// 内存使用阈值
    threshold: usize,
    /// 当前内存使用量
    memory_used: Arc<Mutex<usize>>,
    /// 垃圾收集状态
    state: Arc<Mutex<ConcurrentGCState>>,
    /// 已标记对象的集合
    marked: Arc<Mutex<HashSet<SendablePtr>>>,
    /// 垃圾收集线程
    gc_thread: Option<thread::JoinHandle<()>>,
    /// 是否启用日志
    enable_logging: bool,
    /// 是否正在运行
    running: Arc<AtomicBool>,
}

impl ConcurrentGC {
    /// 创建新的并发垃圾收集器
    pub fn new() -> Self {
        Self {
            objects: Arc::new(Mutex::new(Vec::new())),
            threshold: 1024 * 1024, // 1MB 阈值
            memory_used: Arc::new(Mutex::new(0)),
            state: Arc::new(Mutex::new(ConcurrentGCState::Idle)),
            marked: Arc::new(Mutex::new(HashSet::new())),
            gc_thread: None,
            enable_logging: false,
            running: Arc::new(AtomicBool::new(true)),
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
        // 启动垃圾收集线程（如果未启动）
        self.start();

        // 创建对象的 Box 包装
        let boxed_value = Box::new(value);

        // 将对象添加到对象列表
        let mut objects = self.objects.lock().unwrap();
        objects.push(boxed_value);

        // 更新内存使用量
        let mut memory_used = self.memory_used.lock().unwrap();
        *memory_used += 1;

        // 检查是否需要执行垃圾收集
        if *memory_used >= self.threshold {
            // 启动垃圾收集
            *self.state.lock().unwrap() = ConcurrentGCState::Marking;
        }

        // 返回对象的克隆
        *(*objects.last().unwrap()).clone()
    }

    /// 标记根对象
    ///
    /// # 参数
    /// - `roots`：根对象列表
    fn mark_roots(&self, roots: &[&RubyValue]) {
        let mut marked = self.marked.lock().unwrap();
        for root in roots {
            let root_ptr = SendablePtr(*root as *const RubyValue);
            marked.insert(root_ptr);
        }
    }

    /// 标记阶段
    ///
    /// # 参数
    /// - `roots`：根对象列表
    fn mark(&self, roots: &[&RubyValue]) {
        // 标记根对象
        self.mark_roots(roots);

        // 标记所有对象的引用
        let objects = self.objects.lock().unwrap();
        let marked = self.marked.lock().unwrap();

        for object in &*objects {
            let obj_ptr = SendablePtr(object.as_ref() as *const RubyValue);
            if marked.contains(&obj_ptr) {
                // 简化实现，实际需要递归标记引用
            }
        }
    }

    /// 清除阶段
    fn sweep(&self) {
        let mut objects = self.objects.lock().unwrap();
        let marked = self.marked.lock().unwrap();
        let mut memory_used = self.memory_used.lock().unwrap();

        let mut new_objects = Vec::new();

        for object in objects.drain(..) {
            let obj_ptr = SendablePtr(object.as_ref() as *const RubyValue);
            if marked.contains(&obj_ptr) {
                new_objects.push(object);
            }
            else {
                // 对象被回收
                *memory_used -= 1;
            }
        }

        *objects = new_objects;
    }

    /// 垃圾收集线程函数
    ///
    /// # 参数
    /// - `objects`：对象列表
    /// - `memory_used`：内存使用量
    /// - `state`：垃圾收集状态
    /// - `marked`：已标记对象的集合
    /// - `enable_logging`：是否启用日志
    /// - `running`：是否正在运行
    fn gc_thread_fn(
        objects: Arc<Mutex<Vec<Box<RubyValue>>>>,
        memory_used: Arc<Mutex<usize>>,
        state: Arc<Mutex<ConcurrentGCState>>,
        marked: Arc<Mutex<HashSet<SendablePtr>>>,
        enable_logging: bool,
        running: Arc<AtomicBool>,
    ) {
        while running.load(Ordering::Relaxed) {
            // 检查是否需要执行垃圾收集
            let current_state = *state.lock().unwrap();
            if current_state == ConcurrentGCState::Idle {
                // 等待一段时间
                thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }

            match current_state {
                ConcurrentGCState::Marking => {
                    if enable_logging {
                        println!("[GC] Starting marking phase...");
                    }

                    // 执行标记
                    let objects = objects.lock().unwrap();
                    let marked = marked.lock().unwrap();

                    // 标记所有对象的引用
                    for object in &*objects {
                        let obj_ptr = SendablePtr(object.as_ref() as *const RubyValue);
                        if marked.contains(&obj_ptr) {
                            // 简化实现，实际需要递归标记引用
                        }
                    }

                    if enable_logging {
                        println!("[GC] Marking phase completed");
                    }

                    *state.lock().unwrap() = ConcurrentGCState::Sweeping;
                }
                ConcurrentGCState::Sweeping => {
                    if enable_logging {
                        println!("[GC] Starting sweeping phase...");
                    }

                    // 执行清除
                    let mut objects = objects.lock().unwrap();
                    let marked = marked.lock().unwrap();
                    let mut memory_used = memory_used.lock().unwrap();

                    let mut new_objects = Vec::new();

                    for object in objects.drain(..) {
                        let obj_ptr = SendablePtr(object.as_ref() as *const RubyValue);
                        if marked.contains(&obj_ptr) {
                            new_objects.push(object);
                        }
                        else {
                            // 对象被回收
                            *memory_used -= 1;
                        }
                    }

                    *objects = new_objects;

                    if enable_logging {
                        println!("[GC] Sweeping phase completed");
                        println!("[GC] Objects after: {}", objects.len());
                        println!("[GC] Memory used: {}", *memory_used);
                    }

                    *state.lock().unwrap() = ConcurrentGCState::Completed;
                }
                ConcurrentGCState::Completed => {
                    if enable_logging {
                        println!("[GC] Concurrent garbage collection completed");
                    }
                    *state.lock().unwrap() = ConcurrentGCState::Idle;
                }
                _ => {}
            }
        }
    }

    /// 启动垃圾收集线程
    pub fn start(&mut self) {
        if self.gc_thread.is_none() {
            let objects = self.objects.clone();
            let memory_used = self.memory_used.clone();
            let state = self.state.clone();
            let marked = self.marked.clone();
            let enable_logging = self.enable_logging;
            let running = self.running.clone();

            let handle = thread::spawn(move || {
                Self::gc_thread_fn(objects, memory_used, state, marked, enable_logging, running);
            });

            self.gc_thread = Some(handle);
        }
    }

    /// 执行垃圾收集
    ///
    /// # 参数
    /// - `roots`：根对象列表
    pub fn collect(&mut self, roots: &[&RubyValue]) {
        // 标记根对象
        self.mark_roots(roots);

        // 启动垃圾收集
        *self.state.lock().unwrap() = ConcurrentGCState::Marking;

        // 等待垃圾收集完成
        while *self.state.lock().unwrap() != ConcurrentGCState::Idle {
            thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    /// 获取当前内存使用量
    ///
    /// # 返回值
    /// - `usize`：当前内存使用量
    pub fn memory_used(&self) -> usize {
        *self.memory_used.lock().unwrap()
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

    /// 停止垃圾收集线程
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.gc_thread.take() {
            handle.join().unwrap();
        }
    }
}

impl Drop for ConcurrentGC {
    fn drop(&mut self) {
        self.stop();
    }
}

impl crate::gc::GC for ConcurrentGC {
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
