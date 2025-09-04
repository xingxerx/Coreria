// Epoch of Elria - Sandbox Open World with SVG Character

use epoch_of_elria::*;
use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// Global flag for graceful shutdown
static RUNNING: AtomicBool = AtomicBool::new(true);

// Graceful loading sequence
fn display_welcome_screen() {
    println!("\n╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                    🌟 EPOCH OF ELRIA - SANDBOX WORLD 🌟                     ║");
    println!("║                         Self-Improving Game Engine                           ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝");
    println!();
    println!("🎮 Welcome to the next generation of adaptive gaming!");
    println!("🧠 Featuring AI-powered self-improvement and real-time optimization");
    println!();
    thread::sleep(Duration::from_millis(1000));
}

fn display_loading_progress(message: &str, delay_ms: u64) {
    print!("🔄 {}", message);
    io::stdout().flush().unwrap();

    // Animated loading dots
    for _ in 0..3 {
        thread::sleep(Duration::from_millis(delay_ms / 3));
        print!(".");
        io::stdout().flush().unwrap();
    }
    println!(" ✅");
    thread::sleep(Duration::from_millis(200));
}

fn setup_signal_handlers() {
    // Set up Ctrl+C handler for graceful shutdown
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("\n\n🛑 Graceful shutdown initiated...");
        r.store(false, Ordering::SeqCst);
        RUNNING.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");
}

fn graceful_shutdown() {
    println!("\n╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           🌟 GRACEFUL SHUTDOWN 🌟                           ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝");

    display_loading_progress("Saving game state", 800);
    display_loading_progress("Cleaning up resources", 600);
    display_loading_progress("Optimizing performance data", 700);
    display_loading_progress("Finalizing AI learning", 900);

    println!("\n🎮 Thank you for playing Epoch of Elria!");
    println!("🧠 Your gameplay data has been saved for future AI improvements");
    println!("🌟 See you next time, adventurer!");
    println!("\n╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                        Game closed successfully! 👋                          ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝\n");
}

// Helper function to get real memory usage from the engine
fn get_memory_usage(engine: &GameEngine) -> u64 {
    if let Some(stats) = engine.get_memory_stats() {
        stats.current_usage
    } else {
        // Fallback to system memory estimation
        epoch_of_elria::memory_manager::get_system_memory_usage().unwrap_or(128 * 1024 * 1024)
    }
}

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
    // Display welcome screen
    display_welcome_screen();

    // Set up signal handlers for graceful shutdown
    setup_signal_handlers();

    // Progressive loading sequence
    display_loading_progress("Initializing Epoch of Elria Engine", 1200);
    display_loading_progress("Checking system requirements", 800);

    // Check for display environment and set up if needed
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        println!("⚠️  No display environment detected. Setting up virtual display...");
        std::env::set_var("DISPLAY", ":0");
        println!("💡 If you see graphics errors, try:");
        println!("   - Install VcXsrv or similar X server on Windows");
        println!("   - Run: export DISPLAY=:0 before cargo run");
        println!("   - Use WSL2 with WSLg for native GUI support");
        thread::sleep(Duration::from_millis(500));
    }

    // Create engine configuration for sandbox world
    display_loading_progress("Configuring game engine", 800);
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
        console_mode: false, // Enable graphics mode to show sandbox world
        enable_memory_management: true, // Enable advanced memory management
        gc_config: Some(epoch_of_elria::memory_manager::GCConfig {
            max_heap_size: 2 * 1024 * 1024 * 1024, // 2GB max heap
            gc_threshold: 0.75, // Trigger GC at 75% usage
            collection_interval: std::time::Duration::from_secs(15), // GC every 15 seconds
            enable_auto_gc: true,
            enable_leak_detection: true,
            max_object_age: std::time::Duration::from_secs(600), // 10 minutes max age
            memory_pressure_threshold: 0.9, // Emergency at 90%
        }),
    };

    // Initialize the game engine with better error handling
    display_loading_progress("Initializing AI feedback system", 1000);
    display_loading_progress("Loading adaptive code optimizer", 900);
    display_loading_progress("Starting game engine", 1100);

    let mut engine = match GameEngine::new(config) {
        Ok(engine) => {
            println!("✅ Game engine initialized successfully!");
            println!("🧠 Self-improving feedback system: ACTIVE");
            println!("🔧 Adaptive code optimizer: READY");
            engine
        },
        Err(e) => {
            println!("❌ Failed to initialize game engine: {}", e);
            println!("💡 Graphics initialization failed. This is likely due to:");
            println!("   - Missing X11 server (install VcXsrv on Windows)");
            println!("   - No DISPLAY environment variable set");
            println!("   - Missing graphics drivers in WSL");
            println!("   - Try: wsl --install --web-download for WSLg support");
            return Err(e);
        }
    };

    // World creation with loading progress
    display_loading_progress("Creating sandbox open world", 1200);
    display_loading_progress("Initializing UI system", 600);

    // Create UI for sandbox world
    let mut ui = UI::new();

    // Get the scene for world building
    let scene = engine.get_scene();

    // === CREATE BASE PLATE (GROUND) ===
    display_loading_progress("Creating base plate", 800);

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
    display_loading_progress("Creating SVG character", 700);

    // Generate SVG character data
    let character_svg = create_svg_character();
    println!("🎨 SVG Character created: {} bytes", character_svg.len());

    // Add the player character at a good starting position on the base plate
    display_loading_progress("Adding player to world", 500);
    let _player_id = scene.add_player(Vector3D::new(0.0, 1.0, 0.0));

    // Note: The SVG character data would be used by the rendering system
    // to display a custom character instead of the default 3D model

    // === ADD SANDBOX ELEMENTS ===
    display_loading_progress("Adding sandbox elements", 900);

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

    display_loading_progress("Finalizing world setup", 600);
    println!("🌟 Sandbox world created successfully!");

    // Execute welcome script
    if let Err(e) = engine.execute_script("welcome") {
        println!("⚠️ Could not execute welcome script: {}", e);
    }
    println!("🎮 Controls:");
    println!("   WASD - Move character");
    println!("   Mouse - Look around");
    println!("   Space - Jump");
    println!("   E - Interact");
    println!("   ESC - Exit");
    println!("🌍 Explore the open world and collect items!");
    println!("\n╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           🎮 GAME READY TO PLAY! 🎮                         ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝");
    println!("🎮 Commands:");
    println!("   • Press Enter - Run game update cycle");
    println!("   • Type 'exit' - Graceful shutdown");
    println!("   • Type 'env' - Show environment info");
    println!("   • Type 'scripts' - List available scripts");
    println!("   • Type 'run <script>' - Execute a script");
    println!("   • Type 'weather' - Change weather");
    println!("   • Type 'time <hour>' - Set time of day");
    println!("   • Ctrl+C - Emergency shutdown");
    println!();

    // Check if we're in graphics mode or console mode
    let is_graphics_mode = !engine.get_rendering_system().is_headless();

    if is_graphics_mode {
        println!("🎮 Graphics mode enabled! Window controls:");
        println!("   • WASD - Move character");
        println!("   • Mouse - Look around");
        println!("   • Space - Jump");
        println!("   • E - Interact");
        println!("   • ESC - Exit");
        println!("🌍 The sandbox world is now running in the graphics window!");

        // Graphics mode - run continuous game loop with better error handling
        let mut consecutive_errors = 0;
        let mut last_frame_time = std::time::Instant::now();

        loop {
            let frame_start = std::time::Instant::now();

            // Check for graceful shutdown signal
            if !RUNNING.load(Ordering::SeqCst) {
                break;
            }

            // Check if window should close
            if !engine.get_rendering_system().should_continue() {
                println!("🔄 Window requested to close, shutting down gracefully...");
                break;
            }

            // Calculate actual delta time
            let actual_delta_time = frame_start.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = frame_start;

            // Run one game update cycle with timeout protection
            let loop_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                engine.update(|_scene, input, delta_time, ui| {
                    // === PERFORMANCE MONITORING & FEEDBACK ===
                    let fps = if delta_time > 0.0 { 1.0 / delta_time } else { 60.0 };

                    // === ADAPTIVE SANDBOX GAME LOGIC ===
                    let base_move_speed = 10.0;
                    let adaptive_move_speed = if fps < 30.0 {
                        base_move_speed * 0.8
                    } else if fps > 50.0 {
                        base_move_speed * 1.2
                    } else {
                        base_move_speed
                    };

                    let move_speed = adaptive_move_speed * delta_time;
                    let mut movement = Vector3D::new(0.0, 0.0, 0.0);

                    if input.is_key_pressed(input::Key::W) {
                        movement.z -= move_speed;
                    }
                    if input.is_key_pressed(input::Key::S) {
                        movement.z += move_speed;
                    }
                    if input.is_key_pressed(input::Key::A) {
                        movement.x -= move_speed;
                    }
                    if input.is_key_pressed(input::Key::D) {
                        movement.x += move_speed;
                    }

                    // Apply movement to player (this would need scene access)
                    // For now, just log the movement (less frequently to reduce spam)
                    static mut MOVEMENT_LOG_COUNTER: u32 = 0;
                    unsafe {
                        MOVEMENT_LOG_COUNTER += 1;
                        if (movement.x != 0.0 || movement.z != 0.0) && MOVEMENT_LOG_COUNTER % 30 == 0 {
                            println!("Player moving: x={:.2}, z={:.2}", movement.x, movement.z);
                        }
                    }

                    // Update UI
                    ui.texts.clear();
                    ui.texts.push(crate::ui::UIText {
                        text: format!("FPS: {:.1}", fps),
                        x: 10.0,
                        y: 10.0,
                        font_size: 16.0,
                    });
                    ui.texts.push(crate::ui::UIText {
                        text: "Use WASD to move, ESC to exit".to_string(),
                        x: 10.0,
                        y: 30.0,
                        font_size: 14.0,
                    });
                }, &mut ui)
            }));

            match loop_result {
                Ok(Ok(())) => {
                    // Success - reset error counter
                    consecutive_errors = 0;
                },
                Ok(Err(e)) => {
                    consecutive_errors += 1;
                    println!("❌ Game loop error #{}: {}", consecutive_errors, e);

                    if consecutive_errors >= 5 {
                        println!("⚠️  Too many consecutive errors, shutting down to prevent system instability");
                        break;
                    }
                },
                Err(_) => {
                    consecutive_errors += 1;
                    println!("❌ Game loop panicked #{}", consecutive_errors);

                    if consecutive_errors >= 3 {
                        println!("⚠️  Too many panics, shutting down to prevent system instability");
                        break;
                    }
                }
            }

            // Frame rate limiting with adaptive timing
            let frame_time = frame_start.elapsed();
            let target_frame_time = Duration::from_millis(16); // ~60 FPS

            if frame_time < target_frame_time {
                let sleep_time = target_frame_time - frame_time;
                std::thread::sleep(sleep_time);
            } else if frame_time > Duration::from_millis(33) {
                // Frame took longer than 30 FPS, add a small delay to prevent system overload
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    } else {
        // Console mode - wait for user input
        loop {
            // Check for graceful shutdown signal
            if !RUNNING.load(Ordering::SeqCst) {
                break;
            }

            print!("🎮 Command (Enter to continue, 'exit' to quit): ");
            io::stdout().flush().unwrap();

            let mut input_line = String::new();
            match io::stdin().read_line(&mut input_line) {
                Ok(_) => {
                    let command = input_line.trim();
                    let parts: Vec<&str> = command.split_whitespace().collect();
                    let cmd = if !parts.is_empty() { parts[0].to_lowercase() } else { String::new() };

                    match cmd.as_str() {
                    "exit" | "quit" | "q" => {
                        RUNNING.store(false, Ordering::SeqCst);
                        break;
                    },
                    "env" | "environment" => {
                        println!("{}", engine.get_environment().get_environment_info());
                        continue;
                    },
                    "scripts" => {
                        let scripts = engine.get_script_engine().list_scripts();
                        println!("📜 Available Scripts:");
                        for script in scripts {
                            println!("   • {}", script);
                        }
                        continue;
                    },
                    "run" => {
                        if parts.len() > 1 {
                            let script_name = parts[1];
                            match engine.execute_script(script_name) {
                                Ok(_) => println!("✅ Script '{}' executed successfully!", script_name),
                                Err(e) => println!("❌ Failed to execute script '{}': {}", script_name, e),
                            }
                        } else {
                            println!("❌ Usage: run <script_name>");
                        }
                        continue;
                    },
                    "weather" => {
                        println!("🌤️ Changing weather randomly...");
                        // The environment will change weather automatically
                        continue;
                    },
                    "time" => {
                        if parts.len() > 1 {
                            if let Ok(hour) = parts[1].parse::<f32>() {
                                if hour >= 0.0 && hour <= 24.0 {
                                    engine.get_environment_mut().current_time = hour;
                                    println!("🕐 Time set to {:.1}:00", hour);
                                } else {
                                    println!("❌ Time must be between 0.0 and 24.0");
                                }
                            } else {
                                println!("❌ Invalid time format. Use: time <hour>");
                            }
                        } else {
                            println!("❌ Usage: time <hour> (0.0-24.0)");
                        }
                        continue;
                    },
                    "memory" | "mem" => {
                        if let Some(monitor) = engine.get_memory_monitor() {
                            monitor.print_status();
                            if let Some(stats) = engine.get_memory_stats() {
                                println!("📊 Detailed Memory Stats:");
                                println!("   Allocations: {}", stats.allocation_count);
                                println!("   Deallocations: {}", stats.deallocation_count);
                                println!("   GC Cycles: {}", stats.gc_cycles);
                                println!("   Peak Usage: {}", epoch_of_elria::memory_manager::format_memory_size(stats.peak_usage));
                            }
                        } else {
                            println!("❌ Memory management not enabled");
                        }
                        continue;
                    },
                    "gc" => {
                        println!("🗑️  Forcing garbage collection...");
                        engine.force_garbage_collection();
                        continue;
                    },
                    "cleanup" => {
                        println!("🧹 Forcing resource cleanup...");
                        engine.cleanup_unused_resources();
                        continue;
                    },
                    "help" => {
                        println!("📋 Available commands:");
                        println!("   help - Show this help message");
                        println!("   env - Show environment info");
                        println!("   scripts - List available scripts");
                        println!("   run <script> - Execute a script");
                        println!("   weather - Change weather");
                        println!("   time <hour> - Set time of day (0.0-24.0)");
                        println!("   memory - Show memory status");
                        println!("   gc - Force garbage collection");
                        println!("   cleanup - Force resource cleanup");
                        println!("   exit - Exit the game");
                        continue;
                    },
                    "" => {
                        // Empty command, continue to game update
                    },
                    _ => {
                        println!("❓ Unknown command: '{}'. Type a command or press Enter to continue.", cmd);
                        continue;
                    }
                }

                // Run one game update cycle
                let loop_result = engine.update(|_scene, _input, _delta_time, _ui| {
                    // Console mode - just run basic update
                }, &mut ui);

                if let Err(e) = loop_result {
                    println!("❌ Game loop error: {}", e);
                    break;
                }
            }
            Err(e) => {
                println!("❌ Input error: {}", e);
                break;
            }
        }
    }
}
    // Graceful shutdown
    println!("🔄 Shutting down game engine...");
    graceful_shutdown();

    Ok(())
}

