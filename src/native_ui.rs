// native_ui.rs - Native desktop UI window using webview

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[cfg(target_os = "windows")]
use tao::platform::windows::EventLoopExtWindows;
use wry::webview::WebViewBuilder;

use crate::scene::Scene;
use crate::environment::Environment;

pub struct NativeUIWindow {
    running: Arc<Mutex<bool>>,
    game_state: Arc<Mutex<GameState>>,
}

#[derive(Clone)]
pub struct GameState {
    pub fps: f32,
    pub memory_usage: u64,
    pub object_count: usize,
    pub player_position: (f32, f32, f32),
    pub engine_status: String,
    pub scene_objects: Vec<ObjectInfo>,
}

#[derive(Clone)]
pub struct ObjectInfo {
    pub id: String,
    pub name: String,
    pub object_type: String,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub properties: HashMap<String, String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            fps: 60.0,
            memory_usage: 128 * 1024 * 1024, // 128MB
            object_count: 0,
            player_position: (0.0, 1.0, 0.0),
            engine_status: "Running".to_string(),
            scene_objects: Vec::new(),
        }
    }
}

impl NativeUIWindow {
    pub fn new() -> Self {
        Self {
            running: Arc::new(Mutex::new(false)),
            game_state: Arc::new(Mutex::new(GameState::default())),
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🖥️  Starting native UI window...");
        
        *self.running.lock().unwrap() = true;
        
        let running = Arc::clone(&self.running);
        let game_state = Arc::clone(&self.game_state);
        
        thread::spawn(move || {
            if let Err(e) = Self::run_ui_window(running, game_state) {
                eprintln!("❌ Native UI window error: {}", e);
            }
        });
        
        // Give the window time to start
        thread::sleep(Duration::from_millis(500));
        
        println!("✅ Native UI window started successfully!");
        
        Ok(())
    }

    fn run_ui_window(
        running: Arc<Mutex<bool>>,
        game_state: Arc<Mutex<GameState>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        let event_loop: EventLoop<()> = EventLoop::new_any_thread();

        #[cfg(not(target_os = "windows"))]
        let event_loop: EventLoop<()> = EventLoop::new();
        
        let window = WindowBuilder::new()
            .with_title("Epoch of Elria - Game Engine UI")
            .with_inner_size(tao::dpi::LogicalSize::new(1400, 900))
            .with_min_inner_size(tao::dpi::LogicalSize::new(800, 600))
            .build(&event_loop)?;

        let html_content = Self::get_ui_html_content();
        
        // Clone for the IPC handler
        let game_state_for_ipc = Arc::clone(&game_state);

        let builder = WebViewBuilder::new(window)?;
        let builder = builder.with_html(html_content)?;
        let builder = builder.with_initialization_script(&Self::get_initialization_script());
        let builder = builder.with_ipc_handler(move |_window, request: String| {
            Self::handle_ipc_message(&request, Arc::clone(&game_state_for_ipc));
        });
        let _webview = builder.build()?;

        // Start the game state update loop
        let game_state_clone = Arc::clone(&game_state);
        let running_clone = Arc::clone(&running);
        thread::spawn(move || {
            Self::game_state_update_loop(game_state_clone, running_clone);
        });

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("🔄 Native UI window closing...");
                    *running.lock().unwrap() = false;
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    // Handle window resize if needed
                }
                _ => {}
            }
        });
    }

    fn get_ui_html_content() -> String {
        // Read the HTML file or return embedded content
        if let Ok(content) = std::fs::read_to_string("ui/game_engine_ui.html") {
            content
        } else {
            Self::get_embedded_ui_html()
        }
    }

    fn get_embedded_ui_html() -> String {
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Epoch of Elria - Game Engine UI</title>
  <script src="https://cdn.jsdelivr.net/npm/react@18.2.0/umd/react.production.min.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/react-dom@18.2.0/umd/react-dom.production.min.js"></script>
  <script src="https://cdn.tailwindcss.com"></script>
  <script src="https://cdn.jsdelivr.net/npm/@babel/standalone@7.20.15/babel.min.js"></script>
  <style>
    body { margin: 0; overflow: hidden; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; }
    .split { display: flex; flex: 1; }
    .toolbar { background: #1a1a2e; border-bottom: 1px solid #2d2d4b; }
    .explorer { background: #162447; border-right: 1px solid #2d2d4b; }
    .editor { background: #0f172a; }
    .properties { background: #162447; border-left: 1px solid #2d2d4b; }
    .hover-effect:hover { background: #2d2d4b; }
    .custom-scrollbar::-webkit-scrollbar { width: 8px; }
    .custom-scrollbar::-webkit-scrollbar-track { background: #1a1a2e; }
    .custom-scrollbar::-webkit-scrollbar-thumb { background: #4b5e8e; border-radius: 4px; }
    .game-viewport {
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      border: 2px solid #4b5e8e;
      border-radius: 8px;
    }
    .status-bar {
      background: #1a1a2e;
      border-top: 1px solid #2d2d4b;
      height: 24px;
      display: flex;
      align-items: center;
      padding: 0 8px;
      font-size: 12px;
      color: #a0a0a0;
    }
  </style>
</head>
<body>
  <div id="root"></div>
  <script type="text/babel">
    const { useState, useEffect } = React;

    const Toolbar = ({ onAction, gameState }) => (
      <div className="toolbar flex items-center p-2 space-x-2 text-white">
        <div className="flex items-center space-x-2">
          <span className="text-lg font-bold text-blue-400">🌟 Epoch of Elria</span>
          <span className="text-sm text-gray-400">Native UI</span>
        </div>
        <div className="flex space-x-2 ml-4">
          <button 
            onClick={() => onAction('new')}
            className="px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded text-sm"
          >
            📁 New Project
          </button>
          <button 
            onClick={() => onAction('save')}
            className="px-3 py-1 bg-green-600 hover:bg-green-700 rounded text-sm"
          >
            💾 Save
          </button>
          <button 
            onClick={() => onAction('run')}
            className="px-3 py-1 bg-purple-600 hover:bg-purple-700 rounded text-sm"
          >
            ▶️ Run Game
          </button>
        </div>
        <div className="flex-1"></div>
        <div className="flex items-center space-x-2">
          <div className="text-sm text-green-400">🟢 Native UI Active</div>
          <div className="text-sm">FPS: {gameState.fps.toFixed(1)}</div>
        </div>
      </div>
    );

    const SceneEditor = ({ gameState }) => (
      <div className="editor flex-1 flex flex-col">
        <div className="flex items-center justify-between p-2 bg-gray-800 border-b border-gray-600">
          <div className="flex space-x-2">
            <button className="px-3 py-1 rounded text-sm bg-blue-600">🎮 3D View</button>
          </div>
          <div className="text-white text-sm">
            FPS: <span className="text-green-400">{gameState.fps.toFixed(1)}</span> | 
            Objects: <span className="text-blue-400">{gameState.object_count}</span>
          </div>
        </div>
        
        <div className="flex-1 flex items-center justify-center game-viewport m-4">
          <div className="text-center text-white">
            <div className="text-6xl mb-4">🌟</div>
            <h2 className="text-2xl font-bold mb-2">Epoch of Elria</h2>
            <p className="text-lg mb-4">Native Desktop UI</p>
            <div className="text-sm text-gray-300 space-y-1">
              <div>🎮 Game Engine: <span className="text-green-400">{gameState.engine_status}</span></div>
              <div>🖥️ UI Mode: <span className="text-blue-400">Native Desktop</span></div>
              <div>📊 Objects: <span className="text-purple-400">{gameState.object_count}</span></div>
              <div>👤 Player: <span className="text-yellow-400">({gameState.player_position[0].toFixed(1)}, {gameState.player_position[1].toFixed(1)}, {gameState.player_position[2].toFixed(1)})</span></div>
            </div>
          </div>
        </div>
      </div>
    );

    const App = () => {
      const [gameState, setGameState] = useState({
        fps: 60.0,
        memory_usage: 128 * 1024 * 1024,
        object_count: 0,
        player_position: [0.0, 1.0, 0.0],
        engine_status: 'Starting',
        scene_objects: []
      });

      const handleToolbarAction = (action) => {
        console.log(`Toolbar action: ${action}`);
        // Send IPC message to Rust backend
        if (window.ipc) {
          window.ipc.postMessage(JSON.stringify({ action, timestamp: Date.now() }));
        }
      };

      // Listen for game state updates
      useEffect(() => {
        const updateGameState = (newState) => {
          setGameState(newState);
        };

        // Register global update function
        window.updateGameState = updateGameState;

        // Simulate initial state
        setTimeout(() => {
          setGameState(prev => ({ ...prev, engine_status: 'Running' }));
        }, 1000);

        return () => {
          delete window.updateGameState;
        };
      }, []);

      return (
        <div className="h-screen flex flex-col bg-gray-900">
          <Toolbar onAction={handleToolbarAction} gameState={gameState} />
          <div className="split h-full">
            <SceneEditor gameState={gameState} />
          </div>
          <div className="status-bar text-white">
            <span>Native UI Ready</span>
            <span className="mx-2">|</span>
            <span>Engine: {gameState.engine_status}</span>
            <span className="mx-2">|</span>
            <span>FPS: {gameState.fps.toFixed(1)}</span>
            <div className="flex-1"></div>
            <span>Epoch of Elria v0.1.0</span>
          </div>
        </div>
      );
    };

    ReactDOM.render(<App />, document.getElementById('root'));
  </script>
</body>
</html>"#.to_string()
    }

    fn get_initialization_script() -> String {
        r#"
        console.log('🌟 Epoch of Elria Native UI initialized');
        
        // Set up IPC communication
        window.ipc = {
            postMessage: function(message) {
                if (window.ipc && window.ipc.postMessage) {
                    window.ipc.postMessage(message);
                }
            }
        };
        
        // Global function to update game state from Rust
        window.updateGameStateFromRust = function(stateJson) {
            try {
                const state = JSON.parse(stateJson);
                if (window.updateGameState) {
                    window.updateGameState(state);
                }
            } catch (e) {
                console.error('Failed to parse game state:', e);
            }
        };
        "#.to_string()
    }

    fn handle_ipc_message(message: &str, _game_state: Arc<Mutex<GameState>>) {
        println!("🔄 Native UI IPC message: {}", message);
        
        // Parse and handle IPC messages from the UI
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(message) {
            if let Some(action) = parsed.get("action").and_then(|v| v.as_str()) {
                match action {
                    "run" => println!("🎮 Native UI: Run game command received"),
                    "save" => println!("💾 Native UI: Save project command received"),
                    "new" => println!("📁 Native UI: New project command received"),
                    _ => println!("❓ Native UI: Unknown action: {}", action),
                }
            }
        }
    }

    fn game_state_update_loop(game_state: Arc<Mutex<GameState>>, running: Arc<Mutex<bool>>) {
        while *running.lock().unwrap() {
            // This would be called by the main game engine to update state
            // For now, just simulate some updates
            {
                let mut state = game_state.lock().unwrap();
                state.fps = 58.0 + (rand::random::<f32>() * 2.0);
                state.memory_usage += 1024; // Simulate memory growth
            }
            
            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn update_game_state(&self, scene: &Scene, _environment: &Environment, fps: f32) {
        let mut state = self.game_state.lock().unwrap();
        
        state.fps = fps;
        state.memory_usage = estimate_memory_usage();
        state.object_count = scene.get_object_count();
        state.player_position = (0.0, 1.0, 0.0); // Default position
        state.engine_status = if fps > 30.0 { "Running".to_string() } else { "Slow".to_string() };
        
        // Update scene objects - simplified
        state.scene_objects.clear();
        
        // Add basic object info
        let object_count = scene.get_object_count();
        for i in 0..object_count.min(10) {
            let object_type = match i % 3 {
                0 => "Platform",
                1 => "Collectible", 
                _ => "Enemy",
            };
            
            state.scene_objects.push(ObjectInfo {
                id: format!("object_{}", i),
                name: format!("{} {}", object_type, i),
                object_type: object_type.to_string(),
                position: (i as f32 * 2.0, 1.0, i as f32 * 2.0),
                rotation: (0.0, 0.0, 0.0),
                scale: (1.0, 1.0, 1.0),
                properties: HashMap::new(),
            });
        }
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
        println!("🔄 Native UI window stopped");
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
}

fn estimate_memory_usage() -> u64 {
    static mut SIMULATED_MEMORY: u64 = 128 * 1024 * 1024;
    unsafe {
        SIMULATED_MEMORY += 1024;
        SIMULATED_MEMORY
    }
}
