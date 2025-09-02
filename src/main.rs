// Epoch of Elria - Sandbox Open World with SVG Character

use epoch_of_elria::*;
use std::error::Error;

// Helper function to create SVG character data
fn create_svg_character() -> String {
    format!(r#"<svg width="64" height="64" xmlns="http://www.w3.org/2000/svg">
        <rect x="20" y="30" width="24" height="28" fill="blue" rx="4" stroke="black" stroke-width="1"/>
        <circle cx="32" cy="18" r="12" fill="peachpuff" stroke="black" stroke-width="1"/>
        <circle cx="28" cy="16" r="2" fill="black"/>
        <circle cx="36" cy="16" r="2" fill="black"/>
        <path d="M 28 22 Q 32 26 36 22" stroke="black" stroke-width="1" fill="none"/>
        <rect x="12" y="32" width="8" height="16" fill="peachpuff" rx="4"/>
        <rect x="44" y="32" width="8" height="16" fill="peachpuff" rx="4"/>
        <rect x="22" y="58" width="8" height="16" fill="brown" rx="4"/>
        <rect x="34" y="58" width="8" height="16" fill="brown" rx="4"/>
        <circle cx="32" cy="32" r="30" fill="none" stroke="gold" stroke-width="1" opacity="0.3"/>
    </svg>"#)
}

// Helper function to create base plate texture
fn create_base_plate_texture() -> String {
    format!(r#"<svg width="512" height="512" xmlns="http://www.w3.org/2000/svg">
        <rect width="512" height="512" fill="forestgreen"/>
        <g opacity="0.6">
            <rect x="10" y="10" width="2" height="8" fill="limegreen"/>
            <rect x="25" y="15" width="2" height="6" fill="limegreen"/>
            <rect x="40" y="8" width="2" height="10" fill="limegreen"/>
            <rect x="55" y="12" width="2" height="7" fill="limegreen"/>
            <rect x="70" y="18" width="2" height="5" fill="limegreen"/>
        </g>
        <circle cx="100" cy="100" r="15" fill="saddlebrown" opacity="0.4"/>
        <circle cx="300" cy="200" r="20" fill="saddlebrown" opacity="0.3"/>
        <circle cx="450" cy="350" r="12" fill="saddlebrown" opacity="0.4"/>
        <circle cx="150" cy="150" r="3" fill="dimgray"/>
        <circle cx="350" cy="100" r="4" fill="dimgray"/>
        <circle cx="200" cy="400" r="2" fill="dimgray"/>
    </svg>"#)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🌍 Initializing Epoch of Elria - Sandbox Open World...");

    // Create engine configuration for sandbox world
    let config = EngineConfig {
        window_width: 1600,
        window_height: 900,
        window_title: "Epoch of Elria - Sandbox Open World".to_string(),
        vsync: true,
        fullscreen: false,
        max_fps: Some(60),
        enable_physics: true,  // Enable physics for sandbox interactions
        enable_audio: true,    // Enable audio for immersive experience
        debug_mode: true,
    };

    // Initialize the game engine
    let mut engine = GameEngine::new(config)?;

    println!("✅ Engine initialized successfully!");
    println!("🌍 Creating sandbox open world...");

    // Create UI for sandbox world
    let mut ui = UI::new();

    // Get the scene for world building
    let scene = engine.get_scene();

    // === CREATE BASE PLATE (GROUND) ===
    println!("🏗️  Creating base plate...");

    // Create a large ground plane (base plate) - 100x100 units
    let ground_size = 50.0;
    scene.add_platform(
        Vector3D::new(0.0, -1.0, 0.0),           // Position (slightly below origin)
        Vector3D::new(ground_size, 0.2, ground_size)  // Size: 100x0.4x100 units
    );

    // Add some smaller platforms for variety
    for i in 0..8 {
        let angle = (i as f32 * 2.0 * std::f32::consts::PI) / 8.0;
        let distance = 15.0 + (i as f32 * 2.0);
        let x = angle.cos() * distance;
        let z = angle.sin() * distance;
        let y = (i as f32 * 0.5) - 0.5; // Varying heights

        scene.add_platform(
            Vector3D::new(x, y, z),
            Vector3D::new(3.0, 0.3, 3.0)
        );
    }

    // === CREATE SVG CHARACTER ===
    println!("👤 Creating SVG character...");

    // Generate SVG character data
    let character_svg = create_svg_character();
    println!("🎨 SVG Character created: {} bytes", character_svg.len());

    // Add the player character at a good starting position on the base plate
    let player_id = scene.add_player(Vector3D::new(0.0, 1.0, 0.0));

    // Note: The SVG character data would be used by the rendering system
    // to display a custom character instead of the default 3D model

    // === ADD SANDBOX ELEMENTS ===
    println!("🎮 Adding sandbox elements...");

    // Scatter collectibles around the world
    for i in 0..12 {
        let angle = (i as f32 * 2.0 * std::f32::consts::PI) / 12.0;
        let distance = 5.0 + (i as f32 * 2.0);
        let x = angle.cos() * distance;
        let z = angle.sin() * distance;
        let y = 1.0 + (i as f32 * 0.2); // Varying heights

        scene.add_collectible(Vector3D::new(x, y, z), 5 + (i * 2)); // Varying values
    }

    // Add some enemies for interaction
    for i in 0..6 {
        let x = (i as f32 - 2.5) * 8.0;
        let z = if i % 2 == 0 { 20.0 } else { -20.0 };
        let y = 1.0;

        scene.add_enemy(Vector3D::new(x, y, z));
    }

    println!("🌟 Sandbox world created successfully!");
    println!("🎮 Controls:");
    println!("   WASD - Move character");
    println!("   Mouse - Look around");
    println!("   Space - Jump");
    println!("   E - Interact");
    println!("   ESC - Exit");
    println!("🌍 Explore the open world and collect items!");

    // Start the sandbox game loop
    engine.update(|scene, idle_manager, input, delta_time, ui| {
        // === SANDBOX GAME LOGIC ===

        // Character movement (enhanced for open world)
        let move_speed = 10.0 * delta_time;
        let mut movement = Vector3D::new(0.0, 0.0, 0.0);

        if input.is_key_pressed(input::Key::W) {
            movement.z -= move_speed; // Move forward
        }
        if input.is_key_pressed(input::Key::S) {
            movement.z += move_speed; // Move backward
        }
        if input.is_key_pressed(input::Key::A) {
            movement.x -= move_speed; // Move left
        }
        if input.is_key_pressed(input::Key::D) {
            movement.x += move_speed; // Move right
        }
        if input.is_key_pressed(input::Key::Space) {
            movement.y += move_speed * 2.0; // Jump
        }

        // Apply movement to player (if we can access player position)
        // This would need to be implemented in the scene system

        // === SANDBOX INTERACTIONS ===

        // Interaction key
        if input.is_key_pressed(input::Key::E) {
            // Add interaction logic here
            println!("🔍 Interacting with world...");
        }

        // === DYNAMIC WORLD UPDATES ===

        // Update idle systems for resource generation
        idle_manager.update(delta_time as f64);

        // Add some dynamic elements (could spawn new objects, etc.)
        static mut time_accumulator: f32 = 0.0;
        unsafe {
            time_accumulator += delta_time;

            // Every 10 seconds, add some dynamic content
            if time_accumulator > 10.0 {
                time_accumulator = 0.0;
                println!("🌟 Dynamic world event triggered!");
                // Could add new collectibles, enemies, etc.
            }
        }

        // === UI UPDATES ===

        // Update UI with sandbox information (using proper UI methods)
        // Note: The UI system may need to be updated to support these methods
        println!("📊 FPS: {:.1}", 1.0 / delta_time);
        println!("🌍 Sandbox world running smoothly!");

    }, &mut ui)?;

    println!("🌍 Thanks for exploring the Epoch of Elria sandbox world!");
    println!("👋 See you next time, adventurer!");

    Ok(())
}
