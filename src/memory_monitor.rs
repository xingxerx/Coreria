// Memory Monitoring System for Epoch of Elria
// Real-time memory usage tracking, alerts, and performance analysis

use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::memory_manager::{GarbageCollector, MemoryStats, format_memory_size};

/// Memory alert levels
#[derive(Debug, Clone, PartialEq)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Memory alert
#[derive(Debug, Clone)]
pub struct MemoryAlert {
    pub level: AlertLevel,
    pub message: String,
    pub timestamp: Instant,
    pub memory_usage: u64,
    pub memory_pressure: f32,
    pub suggested_action: String,
}

/// Memory monitoring configuration
#[derive(Debug, Clone)]
pub struct MonitorConfig {
    pub monitoring_interval: Duration,
    pub warning_threshold: f32,    // 0.0-1.0
    pub critical_threshold: f32,   // 0.0-1.0
    pub emergency_threshold: f32,  // 0.0-1.0
    pub max_history_size: usize,
    pub enable_alerts: bool,
    pub enable_auto_gc: bool,
    pub enable_detailed_tracking: bool,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            monitoring_interval: Duration::from_secs(5),
            warning_threshold: 0.7,   // 70%
            critical_threshold: 0.85, // 85%
            emergency_threshold: 0.95, // 95%
            max_history_size: 1000,
            enable_alerts: true,
            enable_auto_gc: true,
            enable_detailed_tracking: true,
        }
    }
}

/// Memory usage snapshot
#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    pub timestamp: Instant,
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
    pub memory_pressure: f32,
    pub gc_stats: MemoryStats,
    pub process_memory: u64,
    pub system_memory: u64,
    pub fragmentation: f32,
}

/// Detailed memory breakdown by category
#[derive(Debug, Clone)]
pub struct MemoryBreakdown {
    pub game_objects: u64,
    pub textures: u64,
    pub audio: u64,
    pub ui_elements: u64,
    pub physics: u64,
    pub scripts: u64,
    pub other: u64,
}

/// Memory performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub allocation_rate: f64,      // allocations per second
    pub deallocation_rate: f64,    // deallocations per second
    pub gc_frequency: f64,         // GC cycles per minute
    pub average_gc_time: Duration,
    pub memory_growth_rate: f64,   // bytes per second
    pub fragmentation_trend: f64,  // fragmentation change rate
}

/// Memory monitor
pub struct MemoryMonitor {
    config: MonitorConfig,
    gc_ref: Option<Arc<GarbageCollector>>,
    history: Arc<RwLock<VecDeque<MemorySnapshot>>>,
    alerts: Arc<RwLock<VecDeque<MemoryAlert>>>,
    breakdown: Arc<RwLock<MemoryBreakdown>>,
    metrics: Arc<RwLock<PerformanceMetrics>>,
    running: AtomicBool,
    monitor_thread: Option<thread::JoinHandle<()>>,
    last_snapshot: Arc<RwLock<Option<MemorySnapshot>>>,
    alert_callbacks: Arc<RwLock<Vec<Box<dyn Fn(&MemoryAlert) + Send + Sync>>>>,
}

impl MemoryMonitor {
    pub fn new(config: MonitorConfig) -> Self {
        Self {
            config,
            gc_ref: None,
            history: Arc::new(RwLock::new(VecDeque::new())),
            alerts: Arc::new(RwLock::new(VecDeque::new())),
            breakdown: Arc::new(RwLock::new(MemoryBreakdown {
                game_objects: 0,
                textures: 0,
                audio: 0,
                ui_elements: 0,
                physics: 0,
                scripts: 0,
                other: 0,
            })),
            metrics: Arc::new(RwLock::new(PerformanceMetrics {
                allocation_rate: 0.0,
                deallocation_rate: 0.0,
                gc_frequency: 0.0,
                average_gc_time: Duration::from_millis(0),
                memory_growth_rate: 0.0,
                fragmentation_trend: 0.0,
            })),
            running: AtomicBool::new(false),
            monitor_thread: None,
            last_snapshot: Arc::new(RwLock::new(None)),
            alert_callbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn with_garbage_collector(mut self, gc: Arc<GarbageCollector>) -> Self {
        self.gc_ref = Some(gc);
        self
    }

    pub fn start(&mut self) {
        if !self.running.load(Ordering::Relaxed) {
            self.running.store(true, Ordering::Relaxed);

            let config = self.config.clone();
            let gc_ref = self.gc_ref.clone();
            let history = Arc::clone(&self.history);
            let alerts = Arc::clone(&self.alerts);
            let breakdown = Arc::clone(&self.breakdown);
            let metrics = Arc::clone(&self.metrics);
            let last_snapshot = Arc::clone(&self.last_snapshot);
            let alert_callbacks = Arc::clone(&self.alert_callbacks);
            let running = Arc::new(AtomicBool::new(true));
            let running_clone = Arc::clone(&running);

            self.monitor_thread = Some(thread::spawn(move || {
                Self::monitor_thread_loop(
                    config,
                    gc_ref,
                    history,
                    alerts,
                    breakdown,
                    metrics,
                    last_snapshot,
                    alert_callbacks,
                    running_clone,
                );
            }));

            println!("📊 Memory monitor started");
        }
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.monitor_thread.take() {
            let _ = handle.join();
            println!("📊 Memory monitor stopped");
        }
    }

    fn monitor_thread_loop(
        config: MonitorConfig,
        gc_ref: Option<Arc<GarbageCollector>>,
        history: Arc<RwLock<VecDeque<MemorySnapshot>>>,
        alerts: Arc<RwLock<VecDeque<MemoryAlert>>>,
        breakdown: Arc<RwLock<MemoryBreakdown>>,
        metrics: Arc<RwLock<PerformanceMetrics>>,
        last_snapshot: Arc<RwLock<Option<MemorySnapshot>>>,
        alert_callbacks: Arc<RwLock<Vec<Box<dyn Fn(&MemoryAlert) + Send + Sync>>>>,
        running: Arc<AtomicBool>,
    ) {
        while running.load(Ordering::Relaxed) {
            thread::sleep(config.monitoring_interval);
            
            if running.load(Ordering::Relaxed) {
                let snapshot = Self::take_memory_snapshot(&gc_ref);
                Self::update_history(&history, snapshot.clone(), config.max_history_size);
                Self::update_metrics(&metrics, &snapshot, &last_snapshot);
                Self::check_alerts(&config, &snapshot, &alerts, &alert_callbacks, &gc_ref);
                
                if let Ok(mut last) = last_snapshot.write() {
                    *last = Some(snapshot);
                }
            }
        }
    }

    fn take_memory_snapshot(gc_ref: &Option<Arc<GarbageCollector>>) -> MemorySnapshot {
        let process_memory = Self::get_process_memory();
        let system_memory = Self::get_system_memory();
        let gc_stats = if let Some(gc) = gc_ref {
            gc.get_stats()
        } else {
            MemoryStats {
                total_allocated: 0,
                total_freed: 0,
                current_usage: process_memory,
                peak_usage: process_memory,
                allocation_count: 0,
                deallocation_count: 0,
                gc_cycles: 0,
                last_gc_time: None,
                fragmentation_ratio: 0.0,
            }
        };

        let memory_pressure = if let Some(gc) = gc_ref {
            gc.get_memory_pressure()
        } else {
            process_memory as f32 / system_memory as f32
        };

        MemorySnapshot {
            timestamp: Instant::now(),
            total_memory: system_memory,
            used_memory: process_memory,
            free_memory: system_memory.saturating_sub(process_memory),
            memory_pressure,
            gc_stats: gc_stats.clone(),
            process_memory,
            system_memory,
            fragmentation: gc_stats.fragmentation_ratio,
        }
    }

    fn update_history(
        history: &Arc<RwLock<VecDeque<MemorySnapshot>>>,
        snapshot: MemorySnapshot,
        max_size: usize,
    ) {
        if let Ok(mut hist) = history.write() {
            hist.push_back(snapshot);
            while hist.len() > max_size {
                hist.pop_front();
            }
        }
    }

    fn update_metrics(
        metrics: &Arc<RwLock<PerformanceMetrics>>,
        current: &MemorySnapshot,
        last_snapshot: &Arc<RwLock<Option<MemorySnapshot>>>,
    ) {
        if let (Ok(mut metrics_guard), Ok(last_guard)) = (metrics.write(), last_snapshot.read()) {
            if let Some(last) = &*last_guard {
                let time_diff = current.timestamp.duration_since(last.timestamp).as_secs_f64();
                if time_diff > 0.0 {
                    // Calculate rates
                    let alloc_diff = current.gc_stats.allocation_count.saturating_sub(last.gc_stats.allocation_count);
                    let dealloc_diff = current.gc_stats.deallocation_count.saturating_sub(last.gc_stats.deallocation_count);
                    let memory_diff = current.used_memory as i64 - last.used_memory as i64;
                    let gc_diff = current.gc_stats.gc_cycles.saturating_sub(last.gc_stats.gc_cycles);

                    metrics_guard.allocation_rate = alloc_diff as f64 / time_diff;
                    metrics_guard.deallocation_rate = dealloc_diff as f64 / time_diff;
                    metrics_guard.memory_growth_rate = memory_diff as f64 / time_diff;
                    metrics_guard.gc_frequency = gc_diff as f64 / (time_diff / 60.0); // per minute
                    metrics_guard.fragmentation_trend = 
                        (current.fragmentation - last.fragmentation) as f64 / time_diff;
                }
            }
        }
    }

    fn check_alerts(
        config: &MonitorConfig,
        snapshot: &MemorySnapshot,
        alerts: &Arc<RwLock<VecDeque<MemoryAlert>>>,
        callbacks: &Arc<RwLock<Vec<Box<dyn Fn(&MemoryAlert) + Send + Sync>>>>,
        gc_ref: &Option<Arc<GarbageCollector>>,
    ) {
        if !config.enable_alerts {
            return;
        }

        let pressure = snapshot.memory_pressure;
        let alert = if pressure >= config.emergency_threshold {
            Some(MemoryAlert {
                level: AlertLevel::Emergency,
                message: format!("EMERGENCY: Memory usage at {:.1}%! Immediate action required!", pressure * 100.0),
                timestamp: snapshot.timestamp,
                memory_usage: snapshot.used_memory,
                memory_pressure: pressure,
                suggested_action: "Force garbage collection and free unused resources immediately".to_string(),
            })
        } else if pressure >= config.critical_threshold {
            Some(MemoryAlert {
                level: AlertLevel::Critical,
                message: format!("CRITICAL: Memory usage at {:.1}%", pressure * 100.0),
                timestamp: snapshot.timestamp,
                memory_usage: snapshot.used_memory,
                memory_pressure: pressure,
                suggested_action: "Trigger garbage collection and cleanup unused objects".to_string(),
            })
        } else if pressure >= config.warning_threshold {
            Some(MemoryAlert {
                level: AlertLevel::Warning,
                message: format!("WARNING: Memory usage at {:.1}%", pressure * 100.0),
                timestamp: snapshot.timestamp,
                memory_usage: snapshot.used_memory,
                memory_pressure: pressure,
                suggested_action: "Consider cleaning up old objects and textures".to_string(),
            })
        } else {
            None
        };

        if let Some(alert) = alert {
            // Store alert
            if let Ok(mut alerts_guard) = alerts.write() {
                alerts_guard.push_back(alert.clone());
                while alerts_guard.len() > 100 { // Keep last 100 alerts
                    alerts_guard.pop_front();
                }
            }

            // Trigger callbacks
            if let Ok(callbacks_guard) = callbacks.read() {
                for callback in callbacks_guard.iter() {
                    callback(&alert);
                }
            }

            // Auto-trigger GC if enabled
            if config.enable_auto_gc {
                if let Some(gc) = gc_ref {
                    match alert.level {
                        AlertLevel::Emergency | AlertLevel::Critical => {
                            gc.force_gc();
                            println!("🚨 Auto-triggered emergency GC due to high memory pressure");
                        },
                        AlertLevel::Warning => {
                            if gc.should_trigger_gc() {
                                gc.force_gc();
                                println!("⚠️  Auto-triggered GC due to memory warning");
                            }
                        },
                        _ => {}
                    }
                }
            }

            println!("🚨 Memory Alert [{}]: {}", 
                     match alert.level {
                         AlertLevel::Info => "INFO",
                         AlertLevel::Warning => "WARN",
                         AlertLevel::Critical => "CRIT",
                         AlertLevel::Emergency => "EMRG",
                     },
                     alert.message);
        }
    }

    fn get_process_memory() -> u64 {
        // Platform-specific implementation would go here
        // For now, use a simple estimation
        crate::memory_manager::get_system_memory_usage().unwrap_or(128 * 1024 * 1024)
    }

    fn get_system_memory() -> u64 {
        // Platform-specific implementation would go here
        // For now, assume 8GB system memory
        8 * 1024 * 1024 * 1024
    }

    pub fn add_alert_callback<F>(&self, callback: F) 
    where 
        F: Fn(&MemoryAlert) + Send + Sync + 'static 
    {
        if let Ok(mut callbacks) = self.alert_callbacks.write() {
            callbacks.push(Box::new(callback));
        }
    }

    pub fn get_current_snapshot(&self) -> Option<MemorySnapshot> {
        if let Ok(snapshot) = self.last_snapshot.read() {
            snapshot.clone()
        } else {
            None
        }
    }

    pub fn get_history(&self, count: usize) -> Vec<MemorySnapshot> {
        if let Ok(history) = self.history.read() {
            history.iter().rev().take(count).cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_recent_alerts(&self, count: usize) -> Vec<MemoryAlert> {
        if let Ok(alerts) = self.alerts.read() {
            alerts.iter().rev().take(count).cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_metrics(&self) -> PerformanceMetrics {
        if let Ok(metrics) = self.metrics.read() {
            metrics.clone()
        } else {
            PerformanceMetrics {
                allocation_rate: 0.0,
                deallocation_rate: 0.0,
                gc_frequency: 0.0,
                average_gc_time: Duration::from_millis(0),
                memory_growth_rate: 0.0,
                fragmentation_trend: 0.0,
            }
        }
    }

    pub fn print_status(&self) {
        if let Some(snapshot) = self.get_current_snapshot() {
            println!("📊 Memory Status:");
            println!("   Used: {} ({:.1}%)", 
                     format_memory_size(snapshot.used_memory),
                     snapshot.memory_pressure * 100.0);
            println!("   Total: {}", format_memory_size(snapshot.total_memory));
            println!("   GC Cycles: {}", snapshot.gc_stats.gc_cycles);
            println!("   Fragmentation: {:.1}%", snapshot.fragmentation * 100.0);
        }
    }
}

impl Drop for MemoryMonitor {
    fn drop(&mut self) {
        self.stop();
    }
}
