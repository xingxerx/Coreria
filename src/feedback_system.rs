// feedback_system.rs - Self-Improving Feedback Loop System
// This system monitors performance and automatically optimizes the codebase

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub fps: f32,
    pub frame_time: f32,
    pub memory_usage: u64,
    pub render_time: f32,
    pub physics_time: f32,
    pub update_time: f32,
    pub cpu_usage: f32,
    pub gpu_usage: f32,
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct OptimizationSuggestion {
    pub category: String,
    pub description: String,
    pub impact: f32, // 0.0 to 1.0
    pub implementation: String,
    pub code_changes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AdaptiveParameter {
    pub name: String,
    pub current_value: f32,
    pub optimal_range: (f32, f32),
    pub adjustment_rate: f32,
    pub performance_correlation: f32,
}

pub struct FeedbackSystem {
    // Performance monitoring
    metrics_history: VecDeque<PerformanceMetrics>,
    current_metrics: PerformanceMetrics,
    
    // Analysis engine
    performance_targets: HashMap<String, f32>,
    bottleneck_detector: BottleneckDetector,
    
    // Optimization system
    adaptive_parameters: HashMap<String, AdaptiveParameter>,
    optimization_suggestions: Vec<OptimizationSuggestion>,
    
    // Self-improvement
    code_variants: HashMap<String, String>,
    performance_history: HashMap<String, f32>,
    learning_rate: f32,
    
    // Configuration
    max_history_size: usize,
    analysis_interval: Duration,
    last_analysis: Instant,
}

#[derive(Debug)]
struct BottleneckDetector {
    thresholds: HashMap<String, f32>,
    detection_history: VecDeque<String>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            fps: 60.0,
            frame_time: 16.67,
            memory_usage: 0,
            render_time: 0.0,
            physics_time: 0.0,
            update_time: 0.0,
            cpu_usage: 0.0,
            gpu_usage: 0.0,
            timestamp: Instant::now(),
        }
    }
}

impl FeedbackSystem {
    pub fn new() -> Self {
        let mut performance_targets = HashMap::new();
        performance_targets.insert("fps".to_string(), 60.0);
        performance_targets.insert("frame_time".to_string(), 16.67);
        performance_targets.insert("memory_usage".to_string(), 512.0 * 1024.0 * 1024.0); // 512MB
        performance_targets.insert("cpu_usage".to_string(), 70.0);
        
        let mut adaptive_parameters = HashMap::new();
        adaptive_parameters.insert("render_quality".to_string(), AdaptiveParameter {
            name: "render_quality".to_string(),
            current_value: 1.0,
            optimal_range: (0.5, 1.0),
            adjustment_rate: 0.1,
            performance_correlation: -0.8, // Higher quality = lower performance
        });
        
        adaptive_parameters.insert("physics_iterations".to_string(), AdaptiveParameter {
            name: "physics_iterations".to_string(),
            current_value: 10.0,
            optimal_range: (5.0, 20.0),
            adjustment_rate: 1.0,
            performance_correlation: -0.6,
        });
        
        let mut thresholds = HashMap::new();
        thresholds.insert("fps".to_string(), 30.0);
        thresholds.insert("memory".to_string(), 1024.0 * 1024.0 * 1024.0); // 1GB
        thresholds.insert("cpu".to_string(), 80.0);
        
        Self {
            metrics_history: VecDeque::new(),
            current_metrics: PerformanceMetrics::default(),
            performance_targets,
            bottleneck_detector: BottleneckDetector {
                thresholds,
                detection_history: VecDeque::new(),
            },
            adaptive_parameters,
            optimization_suggestions: Vec::new(),
            code_variants: HashMap::new(),
            performance_history: HashMap::new(),
            learning_rate: 0.1,
            max_history_size: 1000,
            analysis_interval: Duration::from_secs(5),
            last_analysis: Instant::now(),
        }
    }
    
    // === PERFORMANCE MONITORING ===
    
    pub fn update_metrics(&mut self, fps: f32, frame_time: f32, memory_usage: u64) {
        self.current_metrics = PerformanceMetrics {
            fps,
            frame_time,
            memory_usage,
            render_time: self.current_metrics.render_time,
            physics_time: self.current_metrics.physics_time,
            update_time: self.current_metrics.update_time,
            cpu_usage: self.estimate_cpu_usage(),
            gpu_usage: self.estimate_gpu_usage(),
            timestamp: Instant::now(),
        };
        
        // Store in history
        self.metrics_history.push_back(self.current_metrics.clone());
        if self.metrics_history.len() > self.max_history_size {
            self.metrics_history.pop_front();
        }
        
        // Trigger analysis if interval has passed
        if self.last_analysis.elapsed() >= self.analysis_interval {
            self.analyze_performance();
            self.generate_optimizations();
            self.adapt_parameters();
            self.last_analysis = Instant::now();
        }
    }
    
    pub fn record_subsystem_time(&mut self, subsystem: &str, time: f32) {
        match subsystem {
            "render" => self.current_metrics.render_time = time,
            "physics" => self.current_metrics.physics_time = time,
            "update" => self.current_metrics.update_time = time,
            _ => {}
        }
    }
    
    // === PERFORMANCE ANALYSIS ===
    
    fn analyze_performance(&mut self) {
        println!("🔍 Analyzing performance...");
        
        // Detect bottlenecks
        self.detect_bottlenecks();
        
        // Analyze trends
        self.analyze_performance_trends();
        
        // Check against targets
        self.check_performance_targets();
        
        println!("📊 Performance analysis complete");
    }
    
    fn detect_bottlenecks(&mut self) {
        let metrics = &self.current_metrics;
        
        if metrics.fps < self.bottleneck_detector.thresholds["fps"] {
            self.bottleneck_detector.detection_history.push_back("Low FPS detected".to_string());
            println!("⚠️  Bottleneck detected: Low FPS ({:.1})", metrics.fps);
        }
        
        if metrics.memory_usage as f32 > self.bottleneck_detector.thresholds["memory"] {
            self.bottleneck_detector.detection_history.push_back("High memory usage".to_string());
            println!("⚠️  Bottleneck detected: High memory usage ({:.1} MB)", 
                     metrics.memory_usage as f32 / (1024.0 * 1024.0));
        }
        
        if metrics.cpu_usage > self.bottleneck_detector.thresholds["cpu"] {
            self.bottleneck_detector.detection_history.push_back("High CPU usage".to_string());
            println!("⚠️  Bottleneck detected: High CPU usage ({:.1}%)", metrics.cpu_usage);
        }
        
        // Keep history manageable
        if self.bottleneck_detector.detection_history.len() > 50 {
            self.bottleneck_detector.detection_history.pop_front();
        }
    }
    
    fn analyze_performance_trends(&self) {
        if self.metrics_history.len() < 10 {
            return;
        }
        
        let recent_metrics: Vec<_> = self.metrics_history.iter().rev().take(10).collect();
        let avg_fps: f32 = recent_metrics.iter().map(|m| m.fps).sum::<f32>() / recent_metrics.len() as f32;
        let fps_trend = recent_metrics[0].fps - recent_metrics[recent_metrics.len()-1].fps;
        
        println!("📈 Performance trends:");
        println!("   Average FPS (last 10 frames): {:.1}", avg_fps);
        println!("   FPS trend: {:.1}", fps_trend);
        
        if fps_trend < -5.0 {
            println!("📉 Performance degradation detected!");
        } else if fps_trend > 5.0 {
            println!("📈 Performance improvement detected!");
        }
    }
    
    fn check_performance_targets(&self) {
        let metrics = &self.current_metrics;
        
        for (target_name, target_value) in &self.performance_targets {
            let current_value = match target_name.as_str() {
                "fps" => metrics.fps,
                "frame_time" => metrics.frame_time,
                "memory_usage" => metrics.memory_usage as f32,
                "cpu_usage" => metrics.cpu_usage,
                _ => continue,
            };
            
            let performance_ratio = current_value / target_value;
            
            if target_name == "fps" && performance_ratio < 0.8 {
                println!("🎯 Target miss: {} is {:.1}% of target", target_name, performance_ratio * 100.0);
            } else if target_name != "fps" && performance_ratio > 1.2 {
                println!("🎯 Target miss: {} is {:.1}% of target", target_name, performance_ratio * 100.0);
            }
        }
    }
    
    // === OPTIMIZATION GENERATION ===
    
    fn generate_optimizations(&mut self) {
        self.optimization_suggestions.clear();
        
        let metrics = &self.current_metrics;
        
        // FPS optimization suggestions
        if metrics.fps < 45.0 {
            self.optimization_suggestions.push(OptimizationSuggestion {
                category: "Rendering".to_string(),
                description: "Reduce render quality to improve FPS".to_string(),
                impact: 0.8,
                implementation: "Lower texture resolution and reduce polygon count".to_string(),
                code_changes: vec![
                    "renderer.set_quality(0.7)".to_string(),
                    "scene.reduce_detail_level()".to_string(),
                ],
            });
        }
        
        // Memory optimization suggestions
        if metrics.memory_usage > 800 * 1024 * 1024 { // 800MB
            self.optimization_suggestions.push(OptimizationSuggestion {
                category: "Memory".to_string(),
                description: "Implement memory pooling and garbage collection".to_string(),
                impact: 0.6,
                implementation: "Use object pools and periodic cleanup".to_string(),
                code_changes: vec![
                    "scene.cleanup_unused_objects()".to_string(),
                    "renderer.flush_texture_cache()".to_string(),
                ],
            });
        }
        
        // Physics optimization suggestions
        if metrics.physics_time > 5.0 {
            self.optimization_suggestions.push(OptimizationSuggestion {
                category: "Physics".to_string(),
                description: "Reduce physics simulation complexity".to_string(),
                impact: 0.5,
                implementation: "Lower physics iteration count and use spatial partitioning".to_string(),
                code_changes: vec![
                    "physics.set_iterations(8)".to_string(),
                    "physics.enable_spatial_optimization()".to_string(),
                ],
            });
        }
        
        println!("💡 Generated {} optimization suggestions", self.optimization_suggestions.len());
        for suggestion in &self.optimization_suggestions {
            println!("   {} (Impact: {:.1}%): {}", 
                     suggestion.category, 
                     suggestion.impact * 100.0, 
                     suggestion.description);
        }
    }
    
    // === ADAPTIVE PARAMETER ADJUSTMENT ===
    
    fn adapt_parameters(&mut self) {
        let target_fps = self.performance_targets["fps"];
        let current_fps = self.current_metrics.fps;
        let performance_delta = (current_fps - target_fps) / target_fps;
        
        for (name, param) in &mut self.adaptive_parameters {
            // Calculate adjustment based on performance correlation
            let adjustment = performance_delta * param.performance_correlation * param.adjustment_rate;
            let new_value = param.current_value + adjustment;
            
            // Clamp to optimal range
            param.current_value = new_value.clamp(param.optimal_range.0, param.optimal_range.1);
            
            println!("🔧 Adapted {}: {:.2} -> {:.2}", name, param.current_value - adjustment, param.current_value);
        }
    }
    
    // === UTILITY FUNCTIONS ===
    
    fn estimate_cpu_usage(&self) -> f32 {
        // Simplified CPU usage estimation based on frame time
        let target_frame_time = 16.67; // 60 FPS
        (self.current_metrics.frame_time / target_frame_time * 100.0).min(100.0)
    }
    
    fn estimate_gpu_usage(&self) -> f32 {
        // Simplified GPU usage estimation based on render time
        let target_render_time = 10.0; // ms
        (self.current_metrics.render_time / target_render_time * 100.0).min(100.0)
    }
    
    // === PUBLIC INTERFACE ===
    
    pub fn get_current_metrics(&self) -> &PerformanceMetrics {
        &self.current_metrics
    }
    
    pub fn get_optimization_suggestions(&self) -> &Vec<OptimizationSuggestion> {
        &self.optimization_suggestions
    }
    
    pub fn get_adaptive_parameter(&self, name: &str) -> Option<f32> {
        self.adaptive_parameters.get(name).map(|p| p.current_value)
    }
    
    pub fn apply_optimization(&mut self, suggestion: &OptimizationSuggestion) {
        println!("🚀 Applying optimization: {}", suggestion.description);
        
        // Record the optimization attempt
        self.performance_history.insert(
            suggestion.description.clone(),
            self.current_metrics.fps
        );
        
        // In a real implementation, this would actually modify the code
        for code_change in &suggestion.code_changes {
            println!("   Executing: {}", code_change);
        }
    }
    
    pub fn generate_performance_report(&self) -> String {
        format!(
            "=== PERFORMANCE REPORT ===\n\
             FPS: {:.1} (Target: {:.1})\n\
             Frame Time: {:.2}ms\n\
             Memory: {:.1}MB\n\
             CPU Usage: {:.1}%\n\
             GPU Usage: {:.1}%\n\
             Optimizations Available: {}\n\
             Adaptive Parameters: {}\n",
            self.current_metrics.fps,
            self.performance_targets["fps"],
            self.current_metrics.frame_time,
            self.current_metrics.memory_usage as f32 / (1024.0 * 1024.0),
            self.current_metrics.cpu_usage,
            self.current_metrics.gpu_usage,
            self.optimization_suggestions.len(),
            self.adaptive_parameters.len()
        )
    }
}
