// lib.rs - Epoch of Elria Game Engine Library

pub mod math;
pub mod game_objects;
pub mod rendering;
pub mod input;
pub mod physics;
pub mod audio;
pub mod scene;
pub mod ui;
pub mod game_framework;
pub mod game_state;
pub mod environment;
pub mod scripting;
pub mod web_ui;
pub mod native_ui;
pub mod memory_manager;
pub mod auto_cleanup;
pub mod memory_monitor;

// Re-export commonly used types
pub use math::{Vector2D, Vector3D};
pub use game_objects::{GameObject, Player, Collectible};
pub use rendering::RenderingSystem;
pub use input::InputManager;
pub use physics::PhysicsWorld;
pub use scene::Scene;
pub use ui::UI;
pub use environment::Environment;
pub use scripting::{ScriptEngine, Script, ScriptCommand};
use crate::audio::AudioSystem;
use crate::web_ui::WebUIServer;
use crate::native_ui::NativeUIWindow;
use crate::memory_manager::{GarbageCollector, GCConfig, GameObjectPools};
use crate::auto_cleanup::AutoCleanupManager;
use crate::memory_monitor::MemoryMonitor;
use std::sync::Arc;

// Engine configuration
#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
    pub vsync: bool,
    pub fullscreen: bool,
    pub max_fps: Option<u32>,
    pub enable_physics: bool,
    pub enable_audio: bool,
    pub debug_mode: bool,
    pub console_mode: bool,
    pub enable_memory_management: bool,
    pub gc_config: Option<GCConfig>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_width: 1024,
            window_height: 768,
            window_title: "Epoch of Elria Game Engine".to_string(),
            vsync: true,
            fullscreen: false,
            max_fps: None,
            enable_physics: true,
            enable_audio: true,
            debug_mode: false,
            console_mode: false,
            enable_memory_management: true,
            gc_config: Some(GCConfig::default()),
        }
    }
}

// Main game engine struct
pub struct GameEngine {
    config: EngineConfig,
    rendering_system: RenderingSystem,
    input_manager: InputManager,
    physics_world: Option<PhysicsWorld>,
    audio_system: Option<AudioSystem>,
    scene: Scene,
    environment: Environment,
    script_engine: ScriptEngine,
    web_ui_server: Option<WebUIServer>,
    native_ui_window: Option<NativeUIWindow>,
    running: bool,
    delta_time: f32,
    total_time: f32,
    // Memory management components
    garbage_collector: Option<Arc<GarbageCollector>>,
    cleanup_manager: Option<AutoCleanupManager>,
    memory_monitor: Option<MemoryMonitor>,
    object_pools: Option<GameObjectPools>,
}

impl GameEngine {
    pub fn new(config: EngineConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let rendering_system = RenderingSystem::new(&config)?;
        let input_manager = InputManager::new();
        let physics_world = if config.enable_physics {
            Some(PhysicsWorld::new())
        } else {
            None
        };
        let audio_system = if config.enable_audio {
            Some(AudioSystem::new()?)
        } else {
            None
        };
        let scene = Scene::new("Main Scene");
        let environment = Environment::create_forest_environment();
        let script_engine = ScriptEngine::new();

        // Initialize memory management systems
        let (garbage_collector, cleanup_manager, memory_monitor, object_pools) =
            if config.enable_memory_management {
                println!("🗑️  Initializing memory management systems...");

                // Create garbage collector
                let gc_config = config.gc_config.clone().unwrap_or_default();
                let mut gc = GarbageCollector::new(gc_config);
                gc.start();
                let gc_arc = Arc::new(gc);

                // Create cleanup manager
                let mut cleanup = AutoCleanupManager::new(std::time::Duration::from_secs(30))
                    .with_garbage_collector(Arc::clone(&gc_arc));
                cleanup.start();

                // Create memory monitor
                let mut monitor = MemoryMonitor::new(crate::memory_monitor::MonitorConfig::default())
                    .with_garbage_collector(Arc::clone(&gc_arc));

                // Add alert callback for critical memory situations
                monitor.add_alert_callback(|alert| {
                    match alert.level {
                        crate::memory_monitor::AlertLevel::Critical |
                        crate::memory_monitor::AlertLevel::Emergency => {
                            println!("🚨 CRITICAL MEMORY ALERT: {}", alert.message);
                            println!("   Suggested Action: {}", alert.suggested_action);
                        },
                        _ => {}
                    }
                });

                monitor.start();

                // Create object pools
                let pools = GameObjectPools::new();

                println!("✅ Memory management systems initialized successfully!");

                (Some(gc_arc), Some(cleanup), Some(monitor), Some(pools))
            } else {
                (None, None, None, None)
            };

        // Initialize UI systems
        let web_ui_server = if config.debug_mode {
            let server = WebUIServer::new(8080);
            if let Err(e) = server.start() {
                println!("⚠️  Failed to start web UI server: {}", e);
                None
            } else {
                Some(server)
            }
        } else {
            None
        };

        // Initialize native UI window
        let native_ui_window = if config.debug_mode {
            let window = NativeUIWindow::new();
            if let Err(e) = window.start() {
                println!("⚠️  Failed to start native UI window: {}", e);
                None
            } else {
                Some(window)
            }
        } else {
            None
        };

        Ok(Self {
            config,
            rendering_system,
            input_manager,
            physics_world,
            audio_system,
            scene,
            environment,
            script_engine,
            web_ui_server,
            native_ui_window,
            running: false,
            delta_time: 0.0,
            total_time: 0.0,
            garbage_collector,
            cleanup_manager,
            memory_monitor,
            object_pools,
        })
    }

        pub fn update<F>(&mut self, mut update_fn: F, ui: &mut UI) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(&mut Scene, &InputManager, f32, &mut UI),
    {
        let mut last_time = std::time::Instant::now();

        let current_time = std::time::Instant::now();
        self.delta_time = current_time.duration_since(last_time).as_secs_f32();
        self.total_time += self.delta_time;
        last_time = current_time;

        // Handle input
        self.input_manager.update(&mut self.rendering_system);

        // Update environment
        self.environment.update(self.delta_time);

        // Update scripting system
        if let Some(player) = self.scene.get_player_mut() {
            self.script_engine.update(self.delta_time, player, &mut self.environment);
        }

        // Update game logic
        update_fn(&mut self.scene, &self.input_manager, self.delta_time, ui);

        // Update physics
        if let Some(ref mut physics) = self.physics_world {
            physics.step(self.delta_time);
        }

        // Update audio
        if let Some(ref mut audio) = self.audio_system {
            audio.update(self.delta_time);
        }

        // Render
        self.rendering_system.render(&self.scene, ui)?;

        // Update UI systems with current game state
        let fps = if self.delta_time > 0.0 { 1.0 / self.delta_time } else { 60.0 };

        if let Some(ref web_ui) = self.web_ui_server {
            web_ui.update_game_state(&self.scene, &self.environment, fps);
        }

        if let Some(ref native_ui) = self.native_ui_window {
            native_ui.update_game_state(&self.scene, &self.environment, fps);
        }

        // Memory management updates
        if let Some(ref gc) = self.garbage_collector {
            // Check if emergency GC is needed
            if gc.emergency_gc_needed() {
                println!("🚨 Emergency garbage collection triggered!");
                gc.force_gc();
            }
        }

        // Print memory status periodically (every 5 seconds at 60 FPS)
        static mut MEMORY_STATUS_COUNTER: u32 = 0;
        unsafe {
            MEMORY_STATUS_COUNTER += 1;
            if MEMORY_STATUS_COUNTER % 300 == 0 { // Every 5 seconds at 60 FPS
                if let Some(ref monitor) = self.memory_monitor {
                    monitor.print_status();
                }
            }
        }

        // Check for exit conditions
        if self.input_manager.is_key_pressed(input::Key::Escape) {
            self.running = false;
        }

        // Save game data on graceful exit
        log::info!("Game loop ended. Saving game data...");

        Ok(())
    }


    pub fn stop(&mut self) {
        // This method might also be a good place to trigger a save if called.
        if self.running { // Only save if it was running, to prevent saving on initial error perhaps
            log::info!("GameEngine stop called. Saving game data...");
        }
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn get_total_time(&self) -> f32 {
        self.total_time
    }

    pub fn get_scene(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn get_environment(&self) -> &Environment {
        &self.environment
    }

    pub fn get_environment_mut(&mut self) -> &mut Environment {
        &mut self.environment
    }

    pub fn get_script_engine(&mut self) -> &mut ScriptEngine {
        &mut self.script_engine
    }

    pub fn execute_script(&mut self, script_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.script_engine.execute_script(script_name)
    }

    pub fn get_input(&self) -> &InputManager {
        &self.input_manager
    }

    pub fn get_physics(&mut self) -> Option<&mut PhysicsWorld> {
        self.physics_world.as_mut()
    }

    pub fn get_audio(&mut self) -> Option<&mut AudioSystem> {
        self.audio_system.as_mut()
    }

    pub fn get_rendering_system(&self) -> &RenderingSystem {
        &self.rendering_system
    }

    // Memory management getters
    pub fn get_garbage_collector(&self) -> Option<&Arc<GarbageCollector>> {
        self.garbage_collector.as_ref()
    }

    pub fn get_memory_monitor(&self) -> Option<&MemoryMonitor> {
        self.memory_monitor.as_ref()
    }

    pub fn get_object_pools(&self) -> Option<&GameObjectPools> {
        self.object_pools.as_ref()
    }

    pub fn force_garbage_collection(&self) {
        if let Some(ref gc) = self.garbage_collector {
            println!("🗑️  Forcing garbage collection...");
            gc.force_gc();
        }
    }

    pub fn get_memory_stats(&self) -> Option<crate::memory_manager::MemoryStats> {
        self.garbage_collector.as_ref().map(|gc| gc.get_stats())
    }

    pub fn cleanup_unused_resources(&self) {
        if let Some(ref cleanup) = self.cleanup_manager {
            cleanup.force_cleanup();
        }
    }

    pub fn get_memory_pressure(&self) -> f32 {
        if let Some(ref gc) = self.garbage_collector {
            gc.get_memory_pressure()
        } else {
            0.0
        }
    }
}

// Utility functions
pub fn initialize_logging() {
    env_logger::init();
}

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// Error types
#[derive(Debug)]
pub enum EngineError {
    InitializationError(String),
    RenderingError(String),
    PhysicsError(String),
    AudioError(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::InitializationError(msg) => write!(f, "Initialization error: {}", msg),
            EngineError::RenderingError(msg) => write!(f, "Rendering error: {}", msg),
            EngineError::PhysicsError(msg) => write!(f, "Physics error: {}", msg),
            EngineError::AudioError(msg) => write!(f, "Audio error: {}", msg),
            EngineError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for EngineError {}

impl From<std::io::Error> for EngineError {
    fn from(err: std::io::Error) -> Self {
        EngineError::IoError(err)
    }
}