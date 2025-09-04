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
    running: bool,
    delta_time: f32,
    total_time: f32,
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

        // Initialize web UI server
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
            running: false,
            delta_time: 0.0,
            total_time: 0.0,
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

        // Update web UI with current game state
        if let Some(ref web_ui) = self.web_ui_server {
            let fps = if self.delta_time > 0.0 { 1.0 / self.delta_time } else { 60.0 };
            web_ui.update_game_state(&self.scene, &self.environment, fps);
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