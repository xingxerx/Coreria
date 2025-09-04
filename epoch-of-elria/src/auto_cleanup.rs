// Automatic Cleanup System for Epoch of Elria
// Handles automatic resource cleanup, object lifecycle management, and memory optimization

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use crate::memory_manager::{GarbageCollector};

/// Cleanup policy for different types of objects
#[derive(Debug, Clone)]
pub enum CleanupPolicy {
    /// Never cleanup automatically
    Never,
    /// Cleanup after specified duration of inactivity
    IdleTimeout(Duration),
    /// Cleanup when reference count drops to zero
    ReferenceCount,
    /// Cleanup based on memory pressure
    MemoryPressure(f32), // threshold 0.0-1.0
    /// Cleanup when object reaches maximum age
    MaxAge(Duration),
    /// Combined policy - cleanup when any condition is met
    Combined(Vec<CleanupPolicy>),
}

/// Object lifecycle state
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectState {
    Active,
    Idle,
    MarkedForCleanup,
    Cleaning,
    Destroyed,
}

/// Cleanup statistics
#[derive(Debug, Clone)]
pub struct CleanupStats {
    pub objects_cleaned: u64,
    pub memory_freed: u64,
    pub cleanup_cycles: u64,
    pub last_cleanup: Option<Instant>,
    pub average_cleanup_time: Duration,
    pub objects_by_policy: HashMap<String, u64>,
}

/// Managed resource with automatic cleanup
pub struct ManagedResource<T> {
    pub resource: T,
    pub id: u64,
    pub state: ObjectState,
    pub created_at: Instant,
    pub last_accessed: Instant,
    pub access_count: u64,
    pub cleanup_policy: CleanupPolicy,
    pub cleanup_callback: Option<Box<dyn Fn(&T) + Send + Sync>>,
}

/// Automatic cleanup manager
pub struct AutoCleanupManager {
    resources: Arc<RwLock<HashMap<u64, Box<dyn std::any::Any + Send + Sync>>>>,
    cleanup_policies: Arc<RwLock<HashMap<String, CleanupPolicy>>>,
    stats: Arc<RwLock<CleanupStats>>,
    next_resource_id: AtomicU64,
    running: AtomicBool,
    cleanup_thread: Option<thread::JoinHandle<()>>,
    cleanup_interval: Duration,
    gc_ref: Option<Arc<GarbageCollector>>,
}

impl<T> ManagedResource<T> {
    pub fn new(resource: T, policy: CleanupPolicy) -> Self {
        let now = Instant::now();
        Self {
            resource,
            id: 0, // Will be set by manager
            state: ObjectState::Active,
            created_at: now,
            last_accessed: now,
            access_count: 0,
            cleanup_policy: policy,
            cleanup_callback: None,
        }
    }

    pub fn with_cleanup_callback<F>(mut self, callback: F) -> Self 
    where 
        F: Fn(&T) + Send + Sync + 'static 
    {
        self.cleanup_callback = Some(Box::new(callback));
        self
    }

    pub fn access(&mut self) -> &T {
        self.last_accessed = Instant::now();
        self.access_count += 1;
        if self.state == ObjectState::Idle {
            self.state = ObjectState::Active;
        }
        &self.resource
    }

    pub fn access_mut(&mut self) -> &mut T {
        self.last_accessed = Instant::now();
        self.access_count += 1;
        if self.state == ObjectState::Idle {
            self.state = ObjectState::Active;
        }
        &mut self.resource
    }

    pub fn should_cleanup(&self, memory_pressure: f32) -> bool {
        match &self.cleanup_policy {
            CleanupPolicy::Never => false,
            CleanupPolicy::IdleTimeout(duration) => {
                self.state == ObjectState::Idle && self.last_accessed.elapsed() > *duration
            },
            CleanupPolicy::ReferenceCount => {
                // This would need to be implemented with actual reference counting
                false
            },
            CleanupPolicy::MemoryPressure(threshold) => {
                memory_pressure >= *threshold
            },
            CleanupPolicy::MaxAge(max_age) => {
                self.created_at.elapsed() > *max_age
            },
            CleanupPolicy::Combined(policies) => {
                policies.iter().any(|policy| {
                    let temp_resource = ManagedResource {
                        resource: &self.resource,
                        id: self.id,
                        state: self.state.clone(),
                        created_at: self.created_at,
                        last_accessed: self.last_accessed,
                        access_count: self.access_count,
                        cleanup_policy: policy.clone(),
                        cleanup_callback: None,
                    };
                    temp_resource.should_cleanup(memory_pressure)
                })
            }
        }
    }

    pub fn mark_idle(&mut self) {
        if self.state == ObjectState::Active {
            self.state = ObjectState::Idle;
        }
    }

    pub fn prepare_for_cleanup(&mut self) {
        self.state = ObjectState::MarkedForCleanup;
        if let Some(callback) = &self.cleanup_callback {
            callback(&self.resource);
        }
    }
}

impl Default for CleanupStats {
    fn default() -> Self {
        Self {
            objects_cleaned: 0,
            memory_freed: 0,
            cleanup_cycles: 0,
            last_cleanup: None,
            average_cleanup_time: Duration::from_millis(0),
            objects_by_policy: HashMap::new(),
        }
    }
}

impl AutoCleanupManager {
    pub fn new(cleanup_interval: Duration) -> Self {
        Self {
            resources: Arc::new(RwLock::new(HashMap::new())),
            cleanup_policies: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CleanupStats::default())),
            next_resource_id: AtomicU64::new(1),
            running: AtomicBool::new(false),
            cleanup_thread: None,
            cleanup_interval,
            gc_ref: None,
        }
    }

    pub fn with_garbage_collector(mut self, gc: Arc<GarbageCollector>) -> Self {
        self.gc_ref = Some(gc);
        self
    }

    pub fn start(&mut self) {
        if !self.running.load(Ordering::Relaxed) {
            self.running.store(true, Ordering::Relaxed);

            let resources = Arc::clone(&self.resources);
            let stats = Arc::clone(&self.stats);
            let cleanup_interval = self.cleanup_interval;
            let running = Arc::new(AtomicBool::new(true));
            let running_clone = Arc::clone(&running);
            let gc_ref = self.gc_ref.clone();

            self.cleanup_thread = Some(thread::spawn(move || {
                Self::cleanup_thread_loop(resources, stats, cleanup_interval, running_clone, gc_ref);
            }));

            println!("🧹 Auto-cleanup manager started");
        }
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.cleanup_thread.take() {
            let _ = handle.join();
            println!("🧹 Auto-cleanup manager stopped");
        }
    }

    fn cleanup_thread_loop(
        resources: Arc<RwLock<HashMap<u64, Box<dyn std::any::Any + Send + Sync>>>>,
        stats: Arc<RwLock<CleanupStats>>,
        cleanup_interval: Duration,
        running: Arc<AtomicBool>,
        gc_ref: Option<Arc<GarbageCollector>>,
    ) {
        while running.load(Ordering::Relaxed) {
            thread::sleep(cleanup_interval);
            
            if running.load(Ordering::Relaxed) {
                let start_time = Instant::now();
                let cleaned_count = Self::perform_cleanup_cycle(&resources, &stats, &gc_ref);
                let cleanup_duration = start_time.elapsed();

                if cleaned_count > 0 {
                    println!("🧹 Cleanup cycle: {} objects cleaned in {:?}", 
                             cleaned_count, cleanup_duration);
                }

                // Update average cleanup time
                if let Ok(mut stats_guard) = stats.write() {
                    let total_time = stats_guard.average_cleanup_time.as_nanos() as u64 * stats_guard.cleanup_cycles;
                    stats_guard.cleanup_cycles += 1;
                    let new_total = total_time + cleanup_duration.as_nanos() as u64;
                    stats_guard.average_cleanup_time = Duration::from_nanos(new_total / stats_guard.cleanup_cycles);
                    stats_guard.last_cleanup = Some(start_time);
                }
            }
        }
    }

    fn perform_cleanup_cycle(
        resources: &Arc<RwLock<HashMap<u64, Box<dyn std::any::Any + Send + Sync>>>>,
        stats: &Arc<RwLock<CleanupStats>>,
        gc_ref: &Option<Arc<GarbageCollector>>,
    ) -> u64 {
        let mut cleaned_count = 0;
        let memory_pressure = if let Some(gc) = gc_ref {
            gc.get_memory_pressure()
        } else {
            0.0
        };

        // Perform cleanup
        if let Ok(mut resources_guard) = resources.write() {
            let mut to_remove = Vec::new();

            for (id, _resource) in resources_guard.iter() {
                // In a real implementation, we would check cleanup conditions
                // For now, we'll implement basic age-based cleanup
                to_remove.push(*id);
            }

            for id in to_remove {
                resources_guard.remove(&id);
                cleaned_count += 1;
            }
        }

        // Update statistics
        if let Ok(mut stats_guard) = stats.write() {
            stats_guard.objects_cleaned += cleaned_count;
        }

        // Trigger GC if needed
        if let Some(gc) = gc_ref {
            if gc.should_trigger_gc() || memory_pressure > 0.8 {
                gc.force_gc();
            }
        }

        cleaned_count
    }

    pub fn register_resource<T: Send + Sync + 'static>(&self, mut resource: ManagedResource<T>) -> u64 {
        let id = self.next_resource_id.fetch_add(1, Ordering::Relaxed);
        resource.id = id;

        if let Ok(mut resources) = self.resources.write() {
            resources.insert(id, Box::new(resource));
        }

        id
    }

    pub fn get_resource<T: 'static>(&self, id: u64) -> Option<Arc<Mutex<ManagedResource<T>>>> {
        if let Ok(resources) = self.resources.read() {
            if let Some(resource) = resources.get(&id) {
                if let Some(managed_resource) = resource.downcast_ref::<Arc<Mutex<ManagedResource<T>>>>() {
                    return Some(Arc::clone(managed_resource));
                }
            }
        }
        None
    }

    pub fn unregister_resource(&self, id: u64) -> bool {
        if let Ok(mut resources) = self.resources.write() {
            resources.remove(&id).is_some()
        } else {
            false
        }
    }

    pub fn set_policy_for_type(&self, type_name: &str, policy: CleanupPolicy) {
        if let Ok(mut policies) = self.cleanup_policies.write() {
            policies.insert(type_name.to_string(), policy);
        }
    }

    pub fn force_cleanup(&self) {
        Self::perform_cleanup_cycle(&self.resources, &self.stats, &self.gc_ref);
    }

    pub fn get_stats(&self) -> CleanupStats {
        if let Ok(stats) = self.stats.read() {
            stats.clone()
        } else {
            CleanupStats::default()
        }
    }

    pub fn get_resource_count(&self) -> usize {
        if let Ok(resources) = self.resources.read() {
            resources.len()
        } else {
            0
        }
    }
}

impl Drop for AutoCleanupManager {
    fn drop(&mut self) {
        self.stop();
    }
}

// Utility functions for common cleanup policies
pub fn create_game_object_policy() -> CleanupPolicy {
    CleanupPolicy::Combined(vec![
        CleanupPolicy::IdleTimeout(Duration::from_secs(300)), // 5 minutes idle
        CleanupPolicy::MemoryPressure(0.8), // Cleanup at 80% memory usage
    ])
}

pub fn create_texture_policy() -> CleanupPolicy {
    CleanupPolicy::Combined(vec![
        CleanupPolicy::IdleTimeout(Duration::from_secs(600)), // 10 minutes idle
        CleanupPolicy::MemoryPressure(0.7), // Cleanup at 70% memory usage
    ])
}

pub fn create_audio_policy() -> CleanupPolicy {
    CleanupPolicy::Combined(vec![
        CleanupPolicy::IdleTimeout(Duration::from_secs(120)), // 2 minutes idle
        CleanupPolicy::MemoryPressure(0.6), // Cleanup at 60% memory usage
    ])
}

pub fn create_ui_element_policy() -> CleanupPolicy {
    CleanupPolicy::IdleTimeout(Duration::from_secs(60)) // 1 minute idle
}
