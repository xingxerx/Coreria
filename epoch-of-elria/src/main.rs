use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Vector3, Translation3, UnitQuaternion, Point3};
use kiss3d::scene::SceneNode;
use kiss3d::camera::{ArcBall, Camera};
use std::collections::HashSet;
use std::time::{Duration, Instant};

pub mod world;
mod renderer;

use world::{World, BlockType};
use renderer::OptimizedBlockRenderer;

struct Player {
    node: SceneNode,
    position: Vector3<f32>,
    velocity: Vector3<f32>,
    acceleration: f32,
    max_speed: f32,
    friction: f32,
    on_ground: bool,
}

struct Platform {
    node: SceneNode,
    position: Vector3<f32>,
    size: Vector3<f32>,
}

// Epoch of Elria Color Palette
const DEEP_BLUE: (f32, f32, f32) = (0.18, 0.25, 0.33);     // #2E4053
const NEON_ORANGE: (f32, f32, f32) = (1.00, 0.76, 0.03);   // #FFC107
const NEON_BLUE: (f32, f32, f32) = (0.00, 0.75, 1.00);     // #00BFFF
const NIGHT_AMBIENT: (f32, f32, f32) = (0.1, 0.15, 0.25);  // Dark blue ambient
const DAY_AMBIENT: (f32, f32, f32) = (0.8, 0.9, 1.0);      // Bright daylight

struct TimeSystem {
    game_start_time: Instant,
    cycle_duration: Duration,      // 15 minutes total
    day_duration: Duration,        // 10 minutes day
    night_duration: Duration,      // 5 minutes night
}

// Performance monitoring system
struct PerformanceMonitor {
    frame_count: u64,
    last_fps_update: Instant,
    current_fps: f32,
    frame_times: Vec<f32>,
    max_frame_time_samples: usize,
}

impl PerformanceMonitor {
    fn new() -> Self {
        Self {
            frame_count: 0,
            last_fps_update: Instant::now(),
            current_fps: 0.0,
            frame_times: Vec::with_capacity(60),
            max_frame_time_samples: 60,
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.frame_count += 1;
        self.frame_times.push(delta_time);

        if self.frame_times.len() > self.max_frame_time_samples {
            self.frame_times.remove(0);
        }

        let now = Instant::now();
        if now.duration_since(self.last_fps_update).as_secs_f32() >= 0.5 {
            self.current_fps = self.frame_count as f32 / now.duration_since(self.last_fps_update).as_secs_f32();
            self.frame_count = 0;
            // Print performance stats (reduced frequency for better performance)
            let avg_frame_time = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
            let min_frame_time = self.frame_times.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            let max_frame_time = self.frame_times.iter().fold(0.0f32, |a, &b| a.max(b));

            println!("🚀 FPS: {:.1} | Avg: {:.2}ms | Min: {:.2}ms | Max: {:.2}ms",
                     self.current_fps, avg_frame_time * 1000.0, min_frame_time * 1000.0, max_frame_time * 1000.0);

            self.last_fps_update = now;
        }
    }

    fn get_fps(&self) -> f32 {
        self.current_fps
    }
}

impl TimeSystem {
    fn new() -> Self {
        Self {
            game_start_time: Instant::now(),
            cycle_duration: Duration::from_secs(15 * 60), // 15 minutes
            day_duration: Duration::from_secs(10 * 60),   // 10 minutes
            night_duration: Duration::from_secs(5 * 60),  // 5 minutes
        }
    }

    fn get_time_info(&self) -> TimeInfo {
        let elapsed = self.game_start_time.elapsed();
        let cycle_time = elapsed.as_secs_f32() % self.cycle_duration.as_secs_f32();

        let is_day = cycle_time < self.day_duration.as_secs_f32();
        let phase_progress = if is_day {
            cycle_time / self.day_duration.as_secs_f32()
        } else {
            (cycle_time - self.day_duration.as_secs_f32()) / self.night_duration.as_secs_f32()
        };

        // Calculate transition factor for smooth lighting changes
        let transition_factor = if is_day {
            // During day: 0.0 at dawn, 1.0 at noon, 0.8 at dusk
            if phase_progress < 0.5 {
                phase_progress * 2.0 // 0 to 1
            } else {
                1.0 - (phase_progress - 0.5) * 0.4 // 1 to 0.8
            }
        } else {
            // During night: smooth transition from 0.2 to 0.0 and back
            0.2 * (1.0 - (phase_progress - 0.5).abs() * 2.0).max(0.0)
        };

        TimeInfo {
            is_day,
            phase_progress,
            transition_factor,
            cycle_time,
            total_elapsed: elapsed.as_secs_f32(),
        }
    }
}

struct TimeInfo {
    is_day: bool,
    phase_progress: f32,      // 0.0 to 1.0 within current phase
    transition_factor: f32,   // 0.0 to 1.0 for lighting intensity
    cycle_time: f32,         // Current time within 15-minute cycle
    total_elapsed: f32,      // Total game time
}

struct GameUI {
    show_clock: bool,
    show_minimap: bool,
}

struct CameraSystem {
    camera: ArcBall,
    follow_distance: f32,
    follow_height: f32,
    smoothing: f32,
}

impl CameraSystem {
    fn new(player_pos: Vector3<f32>, world: &World) -> Self {
        // Calculate safe camera position - high above player and behind
        let camera_height = player_pos.y + 50.0; // Raise camera by 50 units as requested
        let camera_distance = 15.0;

        // Ensure camera is above ground
        let ground_height = world.get_surface_height(player_pos.x, player_pos.z - camera_distance);
        let safe_camera_height = camera_height.max(ground_height + 10.0);

        let camera_pos = Point3::new(
            player_pos.x,
            safe_camera_height,
            player_pos.z + camera_distance
        );
        let target_pos = Point3::new(player_pos.x, player_pos.y, player_pos.z);

        println!("📷 Camera positioned at ({:.1}, {:.1}, {:.1}) looking at ({:.1}, {:.1}, {:.1})",
                 camera_pos.x, camera_pos.y, camera_pos.z,
                 target_pos.x, target_pos.y, target_pos.z);

        Self {
            camera: ArcBall::new(camera_pos, target_pos),
            follow_distance: camera_distance,
            follow_height: 50.0, // 50 units above player as requested
            smoothing: 0.1,
        }
    }

    fn update(&mut self, player_pos: Vector3<f32>, world: &World, delta_time: f32) {
        // Calculate desired camera position
        let desired_camera_height = player_pos.y + self.follow_height;
        let ground_height = world.get_surface_height(player_pos.x, player_pos.z - self.follow_distance);
        let safe_camera_height = desired_camera_height.max(ground_height + 10.0);

        let desired_camera_pos = Point3::new(
            player_pos.x,
            safe_camera_height,
            player_pos.z + self.follow_distance
        );
        let desired_target = Point3::new(player_pos.x, player_pos.y + 2.0, player_pos.z);

        // Smooth camera movement
        let current_eye = self.camera.eye();
        let current_at = self.camera.at();

        let lerp_factor = (self.smoothing * delta_time * 60.0).min(1.0); // 60 FPS normalized

        let new_eye = Point3::new(
            current_eye.x + (desired_camera_pos.x - current_eye.x) * lerp_factor,
            current_eye.y + (desired_camera_pos.y - current_eye.y) * lerp_factor,
            current_eye.z + (desired_camera_pos.z - current_eye.z) * lerp_factor,
        );

        let new_at = Point3::new(
            current_at.x + (desired_target.x - current_at.x) * lerp_factor,
            current_at.y + (desired_target.y - current_at.y) * lerp_factor,
            current_at.z + (desired_target.z - current_at.z) * lerp_factor,
        );

        // Update camera position by creating a new ArcBall
        // ArcBall doesn't have set methods, so we recreate it
        self.camera = ArcBall::new(new_eye, new_at);
    }
}

impl GameUI {
    fn new() -> Self {
        Self {
            show_clock: true,
            show_minimap: true,
        }
    }

    fn render_clock(&self, _window: &mut Window, _time_info: &TimeInfo) {
        // Clock rendering would be implemented as UI overlay in a full implementation
        // For now, all visual feedback is handled through the 3D scene and atmospheric lighting
    }

    fn render_minimap(&self, _window: &mut Window, _player_pos: Vector3<f32>, _platforms: &[Platform]) {
        // Minimap rendering would be implemented as UI overlay in a full implementation
        // For now, all navigation feedback is handled through the 3D scene
    }
}

impl Player {
    fn new(mut node: SceneNode, start_pos: Vector3<f32>) -> Self {
        node.set_local_translation(Translation3::new(start_pos.x, start_pos.y, start_pos.z));
        Self {
            node,
            position: start_pos,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            acceleration: 25.0,  // How quickly we accelerate
            max_speed: 6.0,      // Maximum horizontal speed
            friction: 0.88,      // Friction coefficient (0.0 = no friction, 1.0 = instant stop)
            on_ground: false,
        }
    }

    fn update(&mut self, delta_time: f32) {
        // Apply gravity
        if !self.on_ground {
            self.velocity.y -= 15.0 * delta_time; // Gravity strength
        }

        // Update position based on velocity
        self.position += self.velocity * delta_time;

        // Update visual position
        self.node.set_local_translation(Translation3::new(
            self.position.x,
            self.position.y,
            self.position.z
        ));
    }

    fn jump(&mut self) {
        if self.on_ground {
            self.velocity.y = 8.0; // Jump strength
            self.on_ground = false;
            println!("🦘 Jump!");
        }
    }

    fn move_horizontal(&mut self, direction: Vector3<f32>, delta_time: f32) {
        if direction.magnitude() > 0.0 {
            // Apply acceleration in the desired direction
            let normalized_direction = direction.normalize();
            self.velocity.x += normalized_direction.x * self.acceleration * delta_time;
            self.velocity.z += normalized_direction.z * self.acceleration * delta_time;

            // Limit to max speed
            let horizontal_speed = (self.velocity.x * self.velocity.x + self.velocity.z * self.velocity.z).sqrt();
            if horizontal_speed > self.max_speed {
                let scale = self.max_speed / horizontal_speed;
                self.velocity.x *= scale;
                self.velocity.z *= scale;
            }

            // Rotate player to face movement direction
            self.face_direction(normalized_direction);
        } else {
            // Apply friction when no input
            self.velocity.x *= self.friction;
            self.velocity.z *= self.friction;

            // Stop very small movements to prevent jitter
            if self.velocity.x.abs() < 0.01 {
                self.velocity.x = 0.0;
            }
            if self.velocity.z.abs() < 0.01 {
                self.velocity.z = 0.0;
            }
        }
    }

    fn face_direction(&mut self, direction: Vector3<f32>) {
        // Calculate rotation angle from direction vector
        let angle = direction.z.atan2(direction.x);
        // Apply rotation to the visual node
        self.node.set_local_rotation(UnitQuaternion::from_axis_angle(&Vector3::y_axis(), angle));
    }

    fn check_world_collision(&mut self, world: &World) {
       self.on_ground = false;

        // Check collision with blocks below player
        let player_bottom = self.position.y - 0.25;
        let check_y = player_bottom.floor() as i32;

        // Check multiple points around player's base
        let check_points = [
            (self.position.x - 0.2, check_y as f32, self.position.z - 0.2),
            (self.position.x + 0.2, check_y as f32, self.position.z - 0.2),
            (self.position.x - 0.2, check_y as f32, self.position.z + 0.2),
            (self.position.x + 0.2, check_y as f32, self.position.z + 0.2),
            (self.position.x, check_y as f32, self.position.z),
        ];

        for (x, y, z) in check_points.iter() {
            if world.is_solid_at(*x, *y, *z) {
                // Found solid ground
                self.position.y = y + 1.25; // Block height + player half-height
                self.velocity.y = 0.0;
                self.on_ground = true;
                break;
            }
        }

        // Check horizontal collisions
        let next_x = self.position.x + self.velocity.x * 0.016; // Predict next position
        let next_z = self.position.z + self.velocity.z * 0.016;

        // Check X collision
        if world.is_solid_at(next_x, self.position.y, self.position.z) {
            self.velocity.x = 0.0;
        }

        // Check Z collision
        if world.is_solid_at(self.position.x, self.position.y, next_z) {
            self.velocity.z = 0.0;
        }

        // Check head collision
        let head_y = self.position.y + 0.25;
        if world.is_solid_at(self.position.x, head_y, self.position.z) && self.velocity.y > 0.0 {
            self.velocity.y = 0.0;
        }
    }
}

fn apply_atmospheric_lighting(window: &mut Window, time_info: &TimeInfo) {
    // Enhanced Epoch of Elria atmospheric lighting
    let (r, g, b) = if time_info.is_day {
        // Day lighting: warm progression from dawn to noon to dusk
        let day_intensity = time_info.transition_factor;
        let base_day = DAY_AMBIENT;

        // Add warm tones during day with subtle neon hints
        let warm_factor = if time_info.phase_progress < 0.3 || time_info.phase_progress > 0.7 {
            // Dawn/dusk: warmer, more orange
            0.2
        } else {
            // Midday: cooler, more blue
            0.0
        };

        (
            (base_day.0 * day_intensity + NEON_ORANGE.0 * warm_factor * 0.1).min(1.0),
            (base_day.1 * day_intensity + NEON_ORANGE.1 * warm_factor * 0.05).min(1.0),
            (base_day.2 * day_intensity).min(1.0),
        )
    } else {
        // Night lighting: deep blue with neon accents and spiral energy
        let night_intensity = time_info.transition_factor;
        let base_night = NIGHT_AMBIENT;

        // Add subtle neon blue glow during night
        let neon_factor = (time_info.phase_progress * 3.14159).sin().abs() * 0.3;

        (
            (base_night.0 + night_intensity * 0.05 + NEON_BLUE.0 * neon_factor * 0.1).min(1.0),
            (base_night.1 + night_intensity * 0.1 + NEON_BLUE.1 * neon_factor * 0.15).min(1.0),
            (base_night.2 + night_intensity * 0.2 + NEON_BLUE.2 * neon_factor * 0.2).min(1.0),
        )
    };

    // Apply enhanced atmospheric background
    window.set_background_color(r, g, b);
}

fn animate_energy_orbs(orbs: &mut Vec<SceneNode>, time_info: &TimeInfo) {
    let time = time_info.total_elapsed;

    for (i, orb) in orbs.iter_mut().enumerate() {
        let base_angle = (i as f32) * 1.256; // Base spiral position
        let rotation_speed = if time_info.is_day { 0.5 } else { 1.2 }; // Faster at night
        let current_angle = base_angle + time * rotation_speed;

        // Spiral motion with vertical oscillation
        let radius = 8.0 + (time * 0.3 + i as f32).sin() * 1.5;
        let x = current_angle.cos() * radius;
        let z = current_angle.sin() * radius;
        let y = 1.0 + i as f32 * 0.5 + (time * 2.0 + i as f32 * 0.5).sin() * 0.3;

        orb.set_local_translation(Translation3::new(x, y, z));

        // Dynamic color based on time and night/day
        let intensity = if time_info.is_day { 0.6 } else { 1.0 };
        let pulse = (time * 3.0 + i as f32).sin() * 0.2 + 0.8;

        orb.set_color(
            NEON_BLUE.0 * intensity * pulse,
            NEON_BLUE.1 * intensity * pulse,
            NEON_BLUE.2 * intensity * pulse,
        );
    }
}



impl Platform {
    fn new(mut node: SceneNode, position: Vector3<f32>, size: Vector3<f32>) -> Self {
        node.set_local_translation(Translation3::new(position.x, position.y, position.z));
        Self { node, position, size }
    }
}

fn main() {
    println!("🌟 CORERIA EVERYTHING TM - ULTRA HIGH PERFORMANCE EDITION");
    println!("🚀 UNLIMITED FPS - MAXIMUM PERFORMANCE MODE");
    println!("🎮 Use WASD/Arrow Keys to move, SPACE to jump, ESC to exit");

    // Force X11 usage for better compatibility (fixes Wayland issues)
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");
    std::env::set_var("GDK_BACKEND", "x11");

    println!("🖥️  Initializing window with X11 backend for maximum compatibility...");

    // Create window with more robust error handling
    let mut window = match std::panic::catch_unwind(|| {
        Window::new("Coreria everything TM - ULTRA PERFORMANCE")
    }) {
        Ok(window) => {
            println!("✅ Window created successfully!");
            window
        },
        Err(_) => {
            println!("⚠️  Primary window creation failed, trying fallback...");
            // Fallback: try with a simpler title
            Window::new("Coreria Game")
        }
    };

    // Configure window for better performance and compatibility
    println!("🔧 Configuring window settings...");

    // Initialize performance monitoring
    let mut perf_monitor = PerformanceMonitor::new();

    // Initialize time system and UI
    let time_system = TimeSystem::new();
    let game_ui = GameUI::new();

    // Initialize procedural world system with async chunk generation
    let mut world = World::new(12345); // Fixed seed for consistent world
    let mut block_renderer = OptimizedBlockRenderer::new();

    println!("🔥 PERFORMANCE MODE ACTIVATED - PREPARING FOR MAXIMUM FPS!");

    // Find a safe spawn position on the terrain surface
    println!("🌍 Generating terrain and finding safe spawn position...");
    let safe_spawn_pos = world.find_safe_spawn_position(0.0, 0.0);

    // Initialize camera system with safe positioning
    let mut camera_system = CameraSystem::new(safe_spawn_pos, &world);

    // Kiss3D will use the camera during render() calls
    window.set_light(Light::StickToCamera);

    // Create player with dynamic neon glow at safe spawn position
    let mut player_node = window.add_cube(0.5, 0.5, 0.5);
    player_node.set_color(NEON_ORANGE.0, NEON_ORANGE.1 * 0.8, NEON_ORANGE.2 * 0.3); // Neon orange glow
    let mut player = Player::new(player_node, safe_spawn_pos);

    // Add some atmospheric elements
    let mut energy_orbs = Vec::new();
    for i in 0..5 {
        let mut orb = window.add_sphere(0.1);
        let angle = (i as f32) * 1.256; // Spiral positioning
        let x = angle.cos() * 8.0;
        let z = angle.sin() * 8.0;
        orb.set_local_translation(Translation3::new(x, 1.0 + i as f32 * 0.5, z));
        orb.set_color(NEON_BLUE.0, NEON_BLUE.1, NEON_BLUE.2);
        energy_orbs.push(orb);
    }

    let mut last_time = std::time::Instant::now();
    let mut pressed_keys = HashSet::new();
    let mut last_ui_update = 0.0f32; // For UI update throttling
    let mut frame_count = 0u64;

    // Game is now running - MAXIMUM PERFORMANCE MODE ENGAGED!
    println!("🚀 ENTERING ULTRA HIGH PERFORMANCE RENDER LOOP!");

    while window.render_with_camera(&mut camera_system.camera) {
        // Calculate delta time for maximum precision
        let current_time = std::time::Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        // Update performance monitoring
        perf_monitor.update(delta_time);
        frame_count += 1;

        // Update time system and get current time info
        let time_info = time_system.get_time_info();

        // PERFORMANCE OPTIMIZATION: Only update world every few frames to reduce CPU load
        if frame_count % 3 == 0 {  // Update world every 3rd frame
            world.update_chunks(player.position);
        }

        // PERFORMANCE OPTIMIZATION: Update rendering with optimized frequency
        if frame_count % 2 == 0 {  // Update rendering every 2nd frame
            block_renderer.update_rendering(&world, &mut window, player.position);
        }

        // PERFORMANCE OPTIMIZATION: Reduce lighting and animation updates
        if frame_count % 5 == 0 {  // Update lighting every 5th frame
            apply_atmospheric_lighting(&mut window, &time_info);
        }

        // PERFORMANCE OPTIMIZATION: Reduce orb animation frequency
        if frame_count % 4 == 0 {  // Update orbs every 4th frame
            animate_energy_orbs(&mut energy_orbs, &time_info);
        }

        // Handle input events
        for event in window.events().iter() {
            match event.value {
                kiss3d::event::WindowEvent::Key(key, action, _) => {
                    match action {
                        kiss3d::event::Action::Press => {
                            pressed_keys.insert(key);
                            if key == kiss3d::event::Key::Space {
                                player.jump();
                            }
                            if key == kiss3d::event::Key::Escape {
                                return;
                            }
                        }
                        kiss3d::event::Action::Release => {
                            pressed_keys.remove(&key);
                        }
                    }
                }
                _ => {}
            }
        }

        // Handle continuous movement - WASD and Arrow Keys
        let mut movement = Vector3::new(0.0, 0.0, 0.0);

        // WASD controls
        if pressed_keys.contains(&kiss3d::event::Key::W) || pressed_keys.contains(&kiss3d::event::Key::Up) {
            movement.z -= 1.0;
        }
        if pressed_keys.contains(&kiss3d::event::Key::S) || pressed_keys.contains(&kiss3d::event::Key::Down) {
            movement.z += 1.0;
        }
        if pressed_keys.contains(&kiss3d::event::Key::A) || pressed_keys.contains(&kiss3d::event::Key::Left) {
            movement.x -= 1.0;
        }
        if pressed_keys.contains(&kiss3d::event::Key::D) || pressed_keys.contains(&kiss3d::event::Key::Right) {
            movement.x += 1.0;
        }

        // Apply smooth movement with acceleration
        player.move_horizontal(movement, delta_time);

        // Update physics
        player.check_world_collision(&world);
        player.update(delta_time);

        // PERFORMANCE OPTIMIZATION: Update camera position occasionally
        if frame_count % 5 == 0 {  // Update camera every 5th frame
            camera_system.update(player.position, &world, delta_time);
        }

        // PERFORMANCE OPTIMIZATION: Reduce UI update frequency for maximum FPS
        if time_info.total_elapsed - last_ui_update >= 2.0 {  // Update UI every 2 seconds instead of 1
            game_ui.render_clock(&mut window, &time_info);
            // Skip minimap for maximum performance
            last_ui_update = time_info.total_elapsed;
        }

        // Reset if player falls too far
        if player.position.y < -10.0 {
            println!("🔄 Player fell too far, respawning at safe location...");
            let respawn_pos = world.find_safe_spawn_position(player.position.x, player.position.z);
            player.position = respawn_pos;
            player.velocity = Vector3::new(0.0, 0.0, 0.0);
        }
    }

    println!("🌟 Thanks for playing Epoch of Elria!");
}
