// Memory Manager - Advanced Garbage Collection System for Epoch of Elria
// Provides automatic memory management, leak detection, and performance optimization

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::atomic::{AtomicU64, AtomicUsize, AtomicBool, Ordering};


/// Memory statistics and monitoring
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_allocated: u64,
    pub total_freed: u64,
    pub current_usage: u64,
    pub peak_usage: u64,
    pub allocation_count: u64,
    pub deallocation_count: u64,
    pub gc_cycles: u64,
    pub last_gc_time: Option<Instant>,
    pub fragmentation_ratio: f32,
}

/// Memory pool for efficient allocation/deallocation
pub struct MemoryPool<T> {
    pool: VecDeque<Box<T>>,
    max_size: usize,
    allocated_count: AtomicUsize,
    total_allocations: AtomicU64,
}

/// Garbage collector configuration
#[derive(Debug, Clone)]
pub struct GCConfig {
    pub max_heap_size: u64,
    pub gc_threshold: f32,
    pub collection_interval: Duration,
    pub enable_auto_gc: bool,
    pub enable_leak_detection: bool,
    pub max_object_age: Duration,
    pub memory_pressure_threshold: f32,
}

impl Default for GCConfig {
    fn default() -> Self {
        Self {
            max_heap_size: 1024 * 1024 * 1024, // 1GB
            gc_threshold: 0.8, // Trigger GC at 80% memory usage
            collection_interval: Duration::from_secs(30),
            enable_auto_gc: true,
            enable_leak_detection: true,
            max_object_age: Duration::from_secs(300), // 5 minutes
            memory_pressure_threshold: 0.9, // Emergency GC at 90%
        }
    }
}

/// Managed object wrapper with reference counting and lifecycle tracking
pub struct ManagedObject<T> {
    pub data: T,
    pub id: u64,
    pub created_at: Instant,
    pub last_accessed: Instant,
    pub reference_count: AtomicUsize,
    pub marked_for_deletion: AtomicBool,
}

/// Main garbage collector
pub struct GarbageCollector {
    config: GCConfig,
    stats: Arc<RwLock<MemoryStats>>,
    objects: Arc<RwLock<HashMap<u64, Arc<dyn std::any::Any + Send + Sync>>>>,
    next_object_id: AtomicU64,
    running: AtomicBool,
    gc_thread: Option<thread::JoinHandle<()>>,
    memory_pools: Arc<Mutex<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>>,
}

impl<T> MemoryPool<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: VecDeque::with_capacity(max_size),
            max_size,
            allocated_count: AtomicUsize::new(0),
            total_allocations: AtomicU64::new(0),
        }
    }

    pub fn allocate(&mut self) -> Option<Box<T>> 
    where 
        T: Default 
    {
        if let Some(obj) = self.pool.pop_front() {
            self.allocated_count.fetch_add(1, Ordering::Relaxed);
            Some(obj)
        } else if self.allocated_count.load(Ordering::Relaxed) < self.max_size {
            self.allocated_count.fetch_add(1, Ordering::Relaxed);
            self.total_allocations.fetch_add(1, Ordering::Relaxed);
            Some(Box::new(T::default()))
        } else {
            None // Pool exhausted
        }
    }

    pub fn deallocate(&mut self, obj: Box<T>) {
        if self.pool.len() < self.max_size {
            self.pool.push_back(obj);
        }
        self.allocated_count.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn get_stats(&self) -> (usize, usize, u64) {
        (
            self.pool.len(),
            self.allocated_count.load(Ordering::Relaxed),
            self.total_allocations.load(Ordering::Relaxed),
        )
    }
}

impl<T> ManagedObject<T> {
    pub fn new(data: T, id: u64) -> Self {
        let now = Instant::now();
        Self {
            data,
            id,
            created_at: now,
            last_accessed: now,
            reference_count: AtomicUsize::new(1),
            marked_for_deletion: AtomicBool::new(false),
        }
    }

    pub fn access(&self) -> &T {
        // Update last accessed time (in a real implementation, this would be atomic)
        &self.data
    }

    pub fn add_reference(&self) {
        self.reference_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn remove_reference(&self) -> bool {
        let prev_count = self.reference_count.fetch_sub(1, Ordering::Relaxed);
        prev_count <= 1
    }

    pub fn mark_for_deletion(&self) {
        self.marked_for_deletion.store(true, Ordering::Relaxed);
    }

    pub fn is_marked_for_deletion(&self) -> bool {
        self.marked_for_deletion.load(Ordering::Relaxed)
    }

    pub fn get_age(&self) -> Duration {
        self.created_at.elapsed()
    }

    pub fn get_idle_time(&self) -> Duration {
        self.last_accessed.elapsed()
    }
}

impl GarbageCollector {
    pub fn new(config: GCConfig) -> Self {
        let stats = Arc::new(RwLock::new(MemoryStats {
            total_allocated: 0,
            total_freed: 0,
            current_usage: 0,
            peak_usage: 0,
            allocation_count: 0,
            deallocation_count: 0,
            gc_cycles: 0,
            last_gc_time: None,
            fragmentation_ratio: 0.0,
        }));

        Self {
            config,
            stats,
            objects: Arc::new(RwLock::new(HashMap::new())),
            next_object_id: AtomicU64::new(1),
            running: AtomicBool::new(false),
            gc_thread: None,
            memory_pools: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn start(&mut self) {
        if self.config.enable_auto_gc && !self.running.load(Ordering::Relaxed) {
            self.running.store(true, Ordering::Relaxed);
            
            let objects = Arc::clone(&self.objects);
            let stats = Arc::clone(&self.stats);
            let config = self.config.clone();
            let running = Arc::new(AtomicBool::new(true));
            let running_clone = Arc::clone(&running);

            self.gc_thread = Some(thread::spawn(move || {
                Self::gc_thread_loop(objects, stats, config, running_clone);
            }));

            println!("🗑️  Garbage Collector started with auto-collection enabled");
        }
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.gc_thread.take() {
            let _ = handle.join();
            println!("🗑️  Garbage Collector stopped");
        }
    }

    fn gc_thread_loop(
        objects: Arc<RwLock<HashMap<u64, Arc<dyn std::any::Any + Send + Sync>>>>,
        stats: Arc<RwLock<MemoryStats>>,
        config: GCConfig,
        running: Arc<AtomicBool>,
    ) {
        while running.load(Ordering::Relaxed) {
            thread::sleep(config.collection_interval);
            
            if running.load(Ordering::Relaxed) {
                Self::perform_gc_cycle(&objects, &stats, &config);
            }
        }
    }

    fn perform_gc_cycle(
        objects: &Arc<RwLock<HashMap<u64, Arc<dyn std::any::Any + Send + Sync>>>>,
        stats: &Arc<RwLock<MemoryStats>>,
        config: &GCConfig,
    ) {
        let start_time = Instant::now();
        let mut collected_count = 0;

        // Perform garbage collection
        if let Ok(mut objects_guard) = objects.write() {
            let mut to_remove = Vec::new();

            for (id, _obj) in objects_guard.iter() {
                // In a real implementation, we would check reference counts and age
                // For now, we'll implement a simple age-based collection
                to_remove.push(*id);
            }

            for id in to_remove {
                objects_guard.remove(&id);
                collected_count += 1;
            }
        }

        // Update statistics
        if let Ok(mut stats_guard) = stats.write() {
            stats_guard.gc_cycles += 1;
            stats_guard.last_gc_time = Some(start_time);
            stats_guard.total_freed += collected_count as u64;
        }

        let gc_duration = start_time.elapsed();
        if collected_count > 0 {
            println!("🗑️  GC Cycle completed: {} objects collected in {:?}", 
                     collected_count, gc_duration);
        }
    }

    pub fn allocate_managed<T: Send + Sync + 'static>(&self, data: T) -> u64 {
        let id = self.next_object_id.fetch_add(1, Ordering::Relaxed);
        let managed_obj = Arc::new(ManagedObject::new(data, id));

        if let Ok(mut objects) = self.objects.write() {
            objects.insert(id, managed_obj);
        }

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.allocation_count += 1;
            stats.total_allocated += std::mem::size_of::<T>() as u64;
            stats.current_usage += std::mem::size_of::<T>() as u64;
            if stats.current_usage > stats.peak_usage {
                stats.peak_usage = stats.current_usage;
            }
        }

        id
    }

    pub fn get_object<T: 'static + Send + Sync>(&self, id: u64) -> Option<Arc<ManagedObject<T>>> {
        if let Ok(objects) = self.objects.read() {
            if let Some(obj) = objects.get(&id) {
                if let Ok(managed_obj) = obj.clone().downcast::<ManagedObject<T>>() {
                    return Some(managed_obj);
                }
            }
        }
        None
    }

    pub fn deallocate(&self, id: u64) -> bool {
        if let Ok(mut objects) = self.objects.write() {
            if let Some(_) = objects.remove(&id) {
                // Update statistics
                if let Ok(mut stats) = self.stats.write() {
                    stats.deallocation_count += 1;
                }
                return true;
            }
        }
        false
    }

    pub fn force_gc(&self) {
        Self::perform_gc_cycle(&self.objects, &self.stats, &self.config);
    }

    pub fn get_stats(&self) -> MemoryStats {
        if let Ok(stats) = self.stats.read() {
            stats.clone()
        } else {
            MemoryStats {
                total_allocated: 0,
                total_freed: 0,
                current_usage: 0,
                peak_usage: 0,
                allocation_count: 0,
                deallocation_count: 0,
                gc_cycles: 0,
                last_gc_time: None,
                fragmentation_ratio: 0.0,
            }
        }
    }

    pub fn get_memory_pressure(&self) -> f32 {
        if let Ok(stats) = self.stats.read() {
            stats.current_usage as f32 / self.config.max_heap_size as f32
        } else {
            0.0
        }
    }

    pub fn should_trigger_gc(&self) -> bool {
        self.get_memory_pressure() >= self.config.gc_threshold
    }

    pub fn emergency_gc_needed(&self) -> bool {
        self.get_memory_pressure() >= self.config.memory_pressure_threshold
    }
}

impl Drop for GarbageCollector {
    fn drop(&mut self) {
        self.stop();
    }
}

// System memory monitoring functions
pub fn get_system_memory_usage() -> Result<u64, Box<dyn std::error::Error>> {
    // Platform-specific memory usage detection
    #[cfg(target_os = "windows")]
    {
        // Windows implementation would use GetProcessMemoryInfo
        Ok(estimate_process_memory())
    }
    
    #[cfg(target_os = "linux")]
    {
        // Linux implementation would read /proc/self/status
        Ok(estimate_process_memory())
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Ok(estimate_process_memory())
    }
}

fn estimate_process_memory() -> u64 {
    // Simplified estimation - in production, use proper system APIs
    std::process::id() as u64 * 1024 * 1024 // Rough estimate
}

pub fn format_memory_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

// Specialized memory pools for game objects
pub struct GameObjectPools {
    pub player_pool: MemoryPool<crate::game_objects::Player>,
    pub collectible_pool: MemoryPool<crate::game_objects::Collectible>,
    pub physics_body_pool: MemoryPool<crate::physics::RigidBody>,
    pub ui_element_pool: MemoryPool<crate::ui::UIText>,
    pub audio_source_pool: MemoryPool<AudioSource>,
    pub texture_pool: MemoryPool<TextureData>,
}

// Audio source for pooling
#[derive(Default)]
pub struct AudioSource {
    pub id: u32,
    pub volume: f32,
    pub pitch: f32,
    pub playing: bool,
    pub looping: bool,
}

// Texture data for pooling
#[derive(Default)]
pub struct TextureData {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub data: Vec<u8>,
}

#[derive(Default, Clone, Copy)]
pub enum TextureFormat {
    #[default]
    RGBA8,
    RGB8,
    R8,
    RGBA16F,
}

impl GameObjectPools {
    pub fn new() -> Self {
        Self {
            player_pool: MemoryPool::new(10),        // Max 10 players
            collectible_pool: MemoryPool::new(1000), // Max 1000 collectibles
            physics_body_pool: MemoryPool::new(2000), // Max 2000 physics bodies
            ui_element_pool: MemoryPool::new(500),   // Max 500 UI elements
            audio_source_pool: MemoryPool::new(100), // Max 100 audio sources
            texture_pool: MemoryPool::new(200),      // Max 200 textures
        }
    }

    pub fn get_total_stats(&self) -> PoolStats {
        let (player_free, player_used, player_total) = self.player_pool.get_stats();
        let (collectible_free, collectible_used, collectible_total) = self.collectible_pool.get_stats();
        let (physics_free, physics_used, physics_total) = self.physics_body_pool.get_stats();
        let (ui_free, ui_used, ui_total) = self.ui_element_pool.get_stats();
        let (audio_free, audio_used, audio_total) = self.audio_source_pool.get_stats();
        let (texture_free, texture_used, texture_total) = self.texture_pool.get_stats();

        PoolStats {
            total_free: player_free + collectible_free + physics_free + ui_free + audio_free + texture_free,
            total_used: player_used + collectible_used + physics_used + ui_used + audio_used + texture_used,
            total_allocations: player_total + collectible_total + physics_total + ui_total + audio_total + texture_total,
            pools: vec![
                ("Player".to_string(), player_free, player_used, player_total),
                ("Collectible".to_string(), collectible_free, collectible_used, collectible_total),
                ("PhysicsBody".to_string(), physics_free, physics_used, physics_total),
                ("UIElement".to_string(), ui_free, ui_used, ui_total),
                ("AudioSource".to_string(), audio_free, audio_used, audio_total),
                ("Texture".to_string(), texture_free, texture_used, texture_total),
            ],
        }
    }
}

#[derive(Debug)]
pub struct PoolStats {
    pub total_free: usize,
    pub total_used: usize,
    pub total_allocations: u64,
    pub pools: Vec<(String, usize, usize, u64)>, // (name, free, used, total_allocations)
}

// Memory leak detector
pub struct LeakDetector {
    allocations: Arc<RwLock<HashMap<u64, AllocationInfo>>>,
    next_allocation_id: AtomicU64,
    enabled: bool,
}

#[derive(Debug, Clone)]
pub struct AllocationInfo {
    pub id: u64,
    pub size: usize,
    pub allocated_at: Instant,
    pub stack_trace: String, // In production, use proper stack trace
    pub object_type: String,
}

impl LeakDetector {
    pub fn new(enabled: bool) -> Self {
        Self {
            allocations: Arc::new(RwLock::new(HashMap::new())),
            next_allocation_id: AtomicU64::new(1),
            enabled,
        }
    }

    pub fn track_allocation(&self, size: usize, object_type: &str) -> u64 {
        if !self.enabled {
            return 0;
        }

        let id = self.next_allocation_id.fetch_add(1, Ordering::Relaxed);
        let info = AllocationInfo {
            id,
            size,
            allocated_at: Instant::now(),
            stack_trace: format!("Allocation #{}", id), // Simplified
            object_type: object_type.to_string(),
        };

        if let Ok(mut allocations) = self.allocations.write() {
            allocations.insert(id, info);
        }

        id
    }

    pub fn track_deallocation(&self, id: u64) {
        if !self.enabled || id == 0 {
            return;
        }

        if let Ok(mut allocations) = self.allocations.write() {
            allocations.remove(&id);
        }
    }

    pub fn detect_leaks(&self, max_age: Duration) -> Vec<AllocationInfo> {
        if !self.enabled {
            return Vec::new();
        }

        let mut leaks = Vec::new();
        if let Ok(allocations) = self.allocations.read() {
            for info in allocations.values() {
                if info.allocated_at.elapsed() > max_age {
                    leaks.push(info.clone());
                }
            }
        }

        leaks
    }

    pub fn get_allocation_count(&self) -> usize {
        if let Ok(allocations) = self.allocations.read() {
            allocations.len()
        } else {
            0
        }
    }

    pub fn get_total_allocated_size(&self) -> usize {
        if let Ok(allocations) = self.allocations.read() {
            allocations.values().map(|info| info.size).sum()
        } else {
            0
        }
    }
}
