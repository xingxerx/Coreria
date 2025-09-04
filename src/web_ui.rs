// web_ui.rs - Web-based UI server for the game engine

use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::scene::Scene;
use crate::environment::Environment;

pub struct WebUIServer {
    port: u16,
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

impl WebUIServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            running: Arc::new(Mutex::new(false)),
            game_state: Arc::new(Mutex::new(GameState::default())),
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))?;
        println!("🌐 Web UI server starting on http://127.0.0.1:{}", self.port);
        
        *self.running.lock().unwrap() = true;
        
        let running = Arc::clone(&self.running);
        let game_state = Arc::clone(&self.game_state);
        
        thread::spawn(move || {
            for stream in listener.incoming() {
                if !*running.lock().unwrap() {
                    break;
                }
                
                match stream {
                    Ok(stream) => {
                        let game_state_clone = Arc::clone(&game_state);
                        thread::spawn(move || {
                            if let Err(e) = handle_connection(stream, game_state_clone) {
                                eprintln!("⚠️  Error handling web UI connection: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        eprintln!("⚠️  Error accepting web UI connection: {}", e);
                    }
                }
            }
        });
        
        println!("✅ Web UI server started successfully!");
        println!("🌐 Open http://127.0.0.1:{} in your browser to access the game engine UI", self.port);
        
        Ok(())
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
        println!("🔄 Web UI server stopped");
    }

    pub fn update_game_state(&self, scene: &Scene, _environment: &Environment, fps: f32) {
        let mut state = self.game_state.lock().unwrap();

        state.fps = fps;
        state.memory_usage = estimate_memory_usage();
        state.object_count = scene.get_object_count();

        // For now, use default player position since we can't easily access it
        // In a full implementation, we'd need to add getter methods to Scene
        state.player_position = (0.0, 1.0, 0.0);

        state.engine_status = if fps > 30.0 { "Running" } else { "Slow" }.to_string();

        // Update scene objects - simplified for now
        state.scene_objects.clear();

        // Add basic object info based on object count
        let object_count = scene.get_object_count();

        // Add a player object
        let player_pos = state.player_position;
        state.scene_objects.push(ObjectInfo {
            id: "player".to_string(),
            name: "Player".to_string(),
            object_type: "Player".to_string(),
            position: player_pos,
            rotation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
            properties: {
                let mut props = HashMap::new();
                props.insert("health".to_string(), "100".to_string());
                props.insert("speed".to_string(), "10.0".to_string());
                props
            },
        });

        // Add some sample objects based on object count
        for i in 1..object_count.min(10) {
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
                properties: {
                    let mut props = HashMap::new();
                    match object_type {
                        "Collectible" => {
                            props.insert("value".to_string(), (i * 10).to_string());
                        },
                        "Enemy" => {
                            props.insert("health".to_string(), "50".to_string());
                            props.insert("damage".to_string(), "10".to_string());
                        },
                        _ => {
                            props.insert("material".to_string(), "Stone".to_string());
                        }
                    }
                    props
                },
            });
        }
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }
}

fn handle_connection(mut stream: TcpStream, game_state: Arc<Mutex<GameState>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    
    let (status_line, content) = if request_line.starts_with("GET / ") {
        // Serve the main UI HTML file
        let html_path = Path::new("ui/game_engine_ui.html");
        if html_path.exists() {
            let contents = fs::read_to_string(html_path)?;
            ("HTTP/1.1 200 OK", contents)
        } else {
            ("HTTP/1.1 404 NOT FOUND", create_fallback_ui())
        }
    } else if request_line.starts_with("GET /api/game-state") {
        // Serve game state as JSON
        let state = game_state.lock().unwrap();
        let json = serde_json_lite::to_string(&*state);
        ("HTTP/1.1 200 OK", json)
    } else if request_line.starts_with("POST /api/") {
        // Handle API commands
        handle_api_command(&request, game_state)?;
        ("HTTP/1.1 200 OK", r#"{"status": "ok"}"#.to_string())
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404 Not Found".to_string())
    };
    
    let content_type = if request_line.contains("/api/") {
        "application/json"
    } else {
        "text/html"
    };
    
    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
        status_line,
        content_type,
        content.len(),
        content
    );
    
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    
    Ok(())
}

fn handle_api_command(request: &str, _game_state: Arc<Mutex<GameState>>) -> Result<(), Box<dyn std::error::Error>> {
    // Parse API commands from the web UI
    if request.contains("/api/run-game") {
        println!("🎮 Web UI: Run game command received");
    } else if request.contains("/api/save-project") {
        println!("💾 Web UI: Save project command received");
    } else if request.contains("/api/build-project") {
        println!("🔨 Web UI: Build project command received");
    }
    
    Ok(())
}

fn create_fallback_ui() -> String {
    r#"<!DOCTYPE html>
<html>
<head>
    <title>Epoch of Elria - Game Engine UI</title>
    <style>
        body { font-family: Arial, sans-serif; background: #1a1a2e; color: white; padding: 20px; }
        .container { max-width: 800px; margin: 0 auto; text-align: center; }
        .status { background: #162447; padding: 20px; border-radius: 8px; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🌟 Epoch of Elria - Game Engine UI</h1>
        <div class="status">
            <h2>Engine Status</h2>
            <p>🎮 Game Engine: <span style="color: #4CAF50;">Running</span></p>
            <p>🌐 Web UI: <span style="color: #4CAF50;">Active</span></p>
            <p>📁 UI File: <span style="color: #FF9800;">ui/game_engine_ui.html not found</span></p>
        </div>
        <p>The full React-based UI will be available when the HTML file is present.</p>
        <p>Game state updates will be available at <code>/api/game-state</code></p>
    </div>
    <script>
        // Auto-refresh every 5 seconds
        setTimeout(() => location.reload(), 5000);
    </script>
</body>
</html>"#.to_string()
}

fn estimate_memory_usage() -> u64 {
    // Simplified memory estimation
    // In a real implementation, this would use system APIs
    static mut SIMULATED_MEMORY: u64 = 128 * 1024 * 1024; // Start with 128MB
    unsafe {
        SIMULATED_MEMORY += 1024; // Simulate gradual memory growth
        SIMULATED_MEMORY
    }
}

// Simple JSON serialization for game state
mod serde_json_lite {
    use super::*;
    
    pub fn to_string(state: &GameState) -> String {
        format!(
            r#"{{
                "fps": {:.1},
                "memory_usage": {},
                "object_count": {},
                "player_position": [{:.2}, {:.2}, {:.2}],
                "engine_status": "{}",
                "scene_objects": [{}]
            }}"#,
            state.fps,
            state.memory_usage,
            state.object_count,
            state.player_position.0,
            state.player_position.1,
            state.player_position.2,
            state.engine_status,
            state.scene_objects.iter()
                .map(|obj| format!(
                    r#"{{"id": "{}", "name": "{}", "type": "{}", "position": [{:.2}, {:.2}, {:.2}]}}"#,
                    obj.id, obj.name, obj.object_type, obj.position.0, obj.position.1, obj.position.2
                ))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
