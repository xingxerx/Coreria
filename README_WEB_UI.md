# 🖥️ Epoch of Elria - Native Desktop UI

## Overview

The Epoch of Elria game engine now includes a modern **native desktop UI** for game development and monitoring. This provides a professional game engine interface similar to Unity or Unreal Engine, running as a **standalone desktop application window** instead of requiring a web browser.

## Features

### 🎮 **Game Engine UI**
- **Project Explorer** - Browse game assets, scripts, scenes, and prefabs
- **Scene Editor** - Visual 3D viewport for game world editing
- **Properties Panel** - Edit object properties in real-time
- **Toolbar** - Quick access to common actions (New, Save, Run, Build)
- **Status Bar** - Real-time engine status and performance metrics

### 🔧 **Real-Time Integration**
- **Live Game State** - Real-time FPS, memory usage, and object count
- **Scene Objects** - Live view of all game objects and their properties
- **Performance Monitoring** - Track engine performance and health
- **API Endpoints** - RESTful API for external tool integration

## Getting Started

### 1. **Enable Native UI**
The native desktop UI is automatically enabled when `debug_mode` is set to `true` in the engine configuration:

```rust
let config = EngineConfig {
    debug_mode: true,  // This enables the native desktop UI
    // ... other settings
};
```

### 2. **Run the Game Engine**
```bash
cargo run
```

You'll see output like:
```
🖥️  Starting native UI window...
✅ Native UI window started successfully!
```

### 3. **Access the Native UI**
The native desktop UI window will automatically open alongside the 3D game window. You'll have:
- **3D Game Window** - The main game viewport with Kiss3D rendering
- **Native UI Window** - The desktop development interface

## UI Components

### **Toolbar**
- 📁 **New Project** - Create a new game project
- 💾 **Save** - Save current project state
- ▶️ **Run Game** - Launch the game engine
- 🔨 **Build** - Compile the project
- 🔍 **Search** - Search through assets and objects

### **Project Explorer**
Navigate through your game project structure:
- 📁 **Assets** - Textures, models, audio, materials
- 📜 **Scripts** - Rust source files and game logic
- 🎬 **Scenes** - Game scenes and levels
- 🧩 **Prefabs** - Reusable game object templates

### **Scene Editor**
- **3D Viewport** - Visual representation of the game world
- **View Modes** - Switch between 3D, 2D, and code views
- **Real-time Stats** - FPS counter and object count
- **Camera Controls** - Navigate the 3D scene

### **Properties Panel**
Edit selected object properties:
- 🏷️ **Name** - Object identifier
- 📍 **Position** - X, Y, Z coordinates
- 🔄 **Rotation** - Euler angles
- 📏 **Scale** - Object scaling
- ❤️ **Health** - For game entities
- ⚡ **Speed** - Movement speed
- 🔧 **Actions** - Apply changes, reset to defaults

## API Endpoints

### **Game State API**
```
GET /api/game-state
```
Returns real-time game state in JSON format:
```json
{
  "fps": 59.1,
  "memory_usage": 134217728,
  "object_count": 42,
  "player_position": [0.0, 1.0, 0.0],
  "engine_status": "Running",
  "scene_objects": [...]
}
```

### **Command API**
```
POST /api/run-game      - Start the game
POST /api/save-project  - Save project
POST /api/build-project - Build project
```

## Technical Details

### **Architecture**
- **Web Server** - Built-in HTTP server on port 8080
- **Real-time Updates** - Game state synchronized every frame
- **React Frontend** - Modern UI built with React and Tailwind CSS
- **RESTful API** - Clean API for external integrations

### **File Structure**
```
ui/
├── game_engine_ui.html     # Main UI file
src/
├── web_ui.rs              # Web server implementation
├── lib.rs                 # Engine integration
└── main.rs                # Game entry point
```

### **Performance**
- **Minimal Overhead** - Web UI only runs in debug mode
- **Efficient Updates** - Only sends changed data
- **Threaded Server** - Non-blocking web server
- **Memory Efficient** - Lightweight JSON serialization

## Customization

### **Adding New UI Components**
Edit `ui/game_engine_ui.html` to add new React components:

```javascript
const MyCustomPanel = () => (
  <div className="custom-panel">
    {/* Your custom UI here */}
  </div>
);
```

### **Extending the API**
Add new endpoints in `src/web_ui.rs`:

```rust
fn handle_api_command(request: &str, game_state: Arc<Mutex<GameState>>) -> Result<(), Box<dyn std::error::Error>> {
    if request.contains("/api/my-custom-endpoint") {
        // Handle your custom API
    }
    Ok(())
}
```

### **Custom Styling**
The UI uses Tailwind CSS for styling. Modify the CSS classes in the HTML file to customize the appearance.

## Troubleshooting

### **Port Already in Use**
If port 8080 is busy, modify the port in `src/lib.rs`:
```rust
let server = WebUIServer::new(8081); // Use different port
```

### **UI Not Loading**
1. Ensure `debug_mode: true` in engine config
2. Check that `ui/game_engine_ui.html` exists
3. Verify no firewall is blocking port 8080

### **API Not Responding**
- Check the game engine console for error messages
- Verify the web server started successfully
- Test with `curl http://127.0.0.1:8080/api/game-state`

## Future Enhancements

- **WebGL Integration** - Direct 3D rendering in the browser
- **Asset Import** - Drag-and-drop asset management
- **Script Editor** - In-browser code editing
- **Multiplayer Tools** - Network debugging and monitoring
- **Performance Profiler** - Detailed performance analysis
- **Plugin System** - Extensible UI components

## Contributing

To contribute to the web UI:

1. **Frontend Changes** - Edit `ui/game_engine_ui.html`
2. **Backend Changes** - Modify `src/web_ui.rs`
3. **API Extensions** - Add new endpoints and handlers
4. **Testing** - Test with different browsers and screen sizes

The web UI provides a professional development environment for the Epoch of Elria game engine, making it easier to create, debug, and optimize your games!
