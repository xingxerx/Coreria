// rendering.rs - Rendering system for the game engine

use crate::math::Vector3D;
use crate::scene::Scene;
use crate::ui::UI;
use crate::EngineConfig;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::{ArcBall, FirstPerson, Camera};
use kiss3d::nalgebra::{Point2, Point3, Vector3};
use kiss3d::text::Font;
use std::time::{Duration, Instant};

pub struct RenderingSystem {
    window: Option<Window>,
    headless_mode: bool,
    camera_type: CameraType,
    arcball_camera: Option<ArcBall>,
    fps_camera: Option<FirstPerson>,
    background_color: (f32, f32, f32),
    wireframe_mode: bool,
    show_debug_info: bool,
    frame_count: u64,
    fps: f32,
    frame_time_accumulator: f32,
    last_fps_update: Instant,
    last_render_time: Instant,
    target_fps: f32,
    objects_initialized: bool,
    consecutive_render_failures: u32,
    last_successful_render: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraType {
    ArcBall,
    FirstPerson,
}

impl RenderingSystem {
    pub fn new(config: &EngineConfig) -> Result<Self, Box<dyn std::error::Error>> {
        println!("🎮 Initializing rendering system...");

        // Check if we should run in console mode
        let (window, headless_mode) = if config.console_mode {
            println!("🖥️  Console mode enabled - running headless");
            (None, true)
        } else {
            // Try to create graphics window with timeout
            match Self::try_create_window(config) {
                Ok(window) => {
                    println!("✅ Graphics window created successfully");
                    (Some(window), false)
                },
                Err(e) => {
                    println!("⚠️  Graphics initialization failed: {}", e);
                    println!("🔄 Falling back to headless mode");
                    (None, true)
                }
            }
        };

        // Set up camera
        let camera_type = CameraType::ArcBall;
        let arcball_camera = Some(ArcBall::new(Point3::new(10.0, 10.0, 10.0), Point3::origin()));
        let fps_camera = None;

        // Set background color
        let background_color = (0.1, 0.1, 0.2);

        let now = Instant::now();

        Ok(Self {
            window,
            headless_mode,
            camera_type,
            arcball_camera,
            fps_camera,
            background_color,
            wireframe_mode: false,
            show_debug_info: config.debug_mode,
            frame_count: 0,
            fps: 60.0,
            frame_time_accumulator: 0.0,
            last_fps_update: now,
            last_render_time: now,
            target_fps: 60.0,
            objects_initialized: false,
            consecutive_render_failures: 0,
            last_successful_render: now,
        })
    }

    fn try_create_window(config: &EngineConfig) -> Result<Window, Box<dyn std::error::Error>> {
        // Create window with error handling
        let mut window = Window::new_with_size(&config.window_title, config.window_width, config.window_height);

        // Set up lighting
        window.set_light(Light::StickToCamera);

        // Set background color to a nice sky blue
        let background_color = (0.5, 0.7, 1.0);
        window.set_background_color(background_color.0, background_color.1, background_color.2);

        Ok(window)
    }

    pub fn should_continue(&self) -> bool {
        if self.headless_mode {
            // In headless mode, continue running (controlled by main loop)
            true
        } else if let Some(window) = &self.window {
            !window.should_close()
        } else {
            false
        }
    }

    pub fn render(&mut self, scene: &Scene, ui: &UI) -> Result<(), Box<dyn std::error::Error>> {
        // Update FPS counter
        self.update_fps();

        // Handle frame rate limiting
        self.limit_frame_rate();

        if self.headless_mode {
            // In headless mode, just simulate rendering
            self.frame_count += 1;
            return Ok(());
        }

        if self.window.is_some() {
            // Handle camera updates
            self.update_camera();

            // Render the scene with timeout protection
            match self.safe_render_wrapper(scene, ui) {
                Ok(_) => {
                    self.frame_count += 1;
                    self.consecutive_render_failures = 0;
                    self.last_successful_render = Instant::now();
                    Ok(())
                },
                Err(e) => {
                    self.consecutive_render_failures += 1;
                    println!("⚠️  Render error #{}: {}", self.consecutive_render_failures, e);

                    // If we have too many consecutive failures or it's been too long since last success
                    let time_since_success = self.last_successful_render.elapsed();
                    if self.consecutive_render_failures >= 3 || time_since_success.as_secs() > 10 {
                        println!("⚠️  Too many render failures or timeout, switching to headless mode");
                        self.headless_mode = true;
                        self.window = None;
                    }
                    Ok(())
                }
            }
        } else {
            // No window available, switch to headless
            self.headless_mode = true;
            self.frame_count += 1;
            Ok(())
        }
    }

    fn safe_render_wrapper(&mut self, scene: &Scene, ui: &UI) -> Result<(), Box<dyn std::error::Error>> {
        // Render with timeout protection
        let render_start = Instant::now();
        let show_debug = self.show_debug_info;
        let frame_count = self.frame_count;
        let fps = self.fps;
        let headless = self.headless_mode;

        if let Some(window) = &mut self.window {
            // Check if window should close first
            if window.should_close() {
                return Err("Window requested to close".into());
            }

            // Clear the scene first
            window.set_background_color(0.5, 0.7, 1.0); // Sky blue

            // Initialize 3D objects from the scene (only once)
            if !self.objects_initialized {
                Self::render_scene_objects_static(window, scene);
                self.objects_initialized = true;
                println!("🎨 3D objects initialized in scene");
            }

            // Add timeout protection for render call
            let render_timeout = Duration::from_millis(100); // 100ms timeout
            let render_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                window.render()
            }));

            match render_result {
                Ok(render_success) => {
                    if render_success {
                        // Check if render took too long
                        let render_time = render_start.elapsed();
                        if render_time > render_timeout {
                            println!("⚠️  Slow render detected: {}ms", render_time.as_millis());
                            // If renders are consistently slow, consider switching to headless
                            if render_time.as_millis() > 500 {
                                return Err("Render timeout exceeded, switching to headless mode".into());
                            }
                        }

                        if show_debug && frame_count % 60 == 0 {
                            println!("FPS: {:.1}, Headless: {}", fps, headless);
                        }

                        // Render UI with error protection
                        let font = Font::default();
                        for text in &ui.texts {
                            let pos = Point2::new(text.x, text.y);
                            if let Err(_) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                window.draw_text(&text.text, &pos, text.font_size, &font, &Point3::new(1.0, 1.0, 1.0));
                            })) {
                                println!("⚠️  UI render error, skipping text rendering");
                            }
                        }

                        Ok(())
                    } else {
                        Err("Window render returned false".into())
                    }
                },
                Err(_) => {
                    println!("⚠️  Render call panicked, switching to headless mode");
                    Err("Render call panicked".into())
                }
            }
        } else {
            Err("No window available".into())
        }
    }

    fn limit_frame_rate(&mut self) {
        let target_frame_time = 1.0 / self.target_fps;
        let elapsed = self.last_render_time.elapsed().as_secs_f32();

        if elapsed < target_frame_time {
            let sleep_time = target_frame_time - elapsed;
            std::thread::sleep(std::time::Duration::from_secs_f32(sleep_time));
        }

        self.last_render_time = Instant::now();
    }

    pub fn render_ui(&mut self, _ui: &UI) {
        // UI rendering is now handled inline in safe_render_wrapper
        // This method is kept for compatibility
    }

    pub fn set_camera_type(&mut self, camera_type: CameraType) {
        self.camera_type = camera_type;
        match camera_type {
            CameraType::ArcBall => {
                if self.arcball_camera.is_none() {
                    self.arcball_camera = Some(ArcBall::new(Point3::new(10.0, 10.0, 10.0), Point3::origin()));
                }
            },
            CameraType::FirstPerson => {
                if self.fps_camera.is_none() {
                    self.fps_camera = Some(FirstPerson::new(Point3::new(10.0, 10.0, 10.0), Point3::origin()));
                }
            },
        }
    }

    pub fn set_background_color(&mut self, r: f32, g: f32, b: f32) {
        self.background_color = (r, g, b);
        if let Some(window) = &mut self.window {
            window.set_background_color(r, g, b);
        }
    }

    pub fn toggle_wireframe(&mut self) {
        self.wireframe_mode = !self.wireframe_mode;
        // Note: kiss3d doesn't have direct wireframe support
        // This would need to be implemented differently
    }

    pub fn toggle_debug_info(&mut self) {
        self.show_debug_info = !self.show_debug_info;
    }

    pub fn get_fps(&self) -> f32 {
        self.fps
    }

    pub fn get_frame_count(&self) -> u64 {
        self.frame_count
    }



    pub fn get_window_size(&self) -> (u32, u32) {
        if let Some(window) = &self.window {
            let size = window.size();
            (size.x as u32, size.y as u32)
        } else {
            (800, 600) // Default size for headless mode
        }
    }

    pub fn is_headless(&self) -> bool {
        self.headless_mode
    }

    pub fn set_window_title(&mut self, title: &str) {
        // Note: kiss3d doesn't provide a direct way to change window title after creation
        // This would need to be implemented at the windowing system level
    }

    pub fn screenshot(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Note: kiss3d doesn't provide direct screenshot functionality
        // This would need to be implemented using the underlying graphics API
        println!("Screenshot functionality not yet implemented: {}", filename);
        Ok(())
    }

    // Camera control methods
    pub fn move_camera(&mut self, direction: Vector3D, amount: f32) {
        match self.camera_type {
            CameraType::ArcBall => {
                // ArcBall camera movement is handled differently
                if let Some(ref mut camera) = self.arcball_camera {
                    // Implement arcball camera movement
                }
            },
            CameraType::FirstPerson => {
                if let Some(ref mut camera) = self.fps_camera {
                    let kiss3d_direction = Vector3::new(direction.x, direction.y, direction.z);
                    // Implement first person camera movement
                }
            },
        }
    }

    pub fn rotate_camera(&mut self, yaw: f32, pitch: f32) {
        match self.camera_type {
            CameraType::ArcBall => {
                // ArcBall rotation is handled by mouse input automatically
            },
            CameraType::FirstPerson => {
                if let Some(ref mut camera) = self.fps_camera {
                    // Implement first person camera rotation
                }
            },
        }
    }

    pub fn zoom_camera(&mut self, amount: f32) {
        match self.camera_type {
            CameraType::ArcBall => {
                if let Some(ref mut camera) = self.arcball_camera {
                    // Implement arcball zoom
                }
            },
            CameraType::FirstPerson => {
                // First person camera doesn't typically zoom
            },
        }
    }

    pub fn get_camera_position(&self) -> Vector3D {
        match self.camera_type {
            CameraType::ArcBall => {
                if let Some(ref camera) = self.arcball_camera {
                    let eye = camera.eye();
                    Vector3D::new(eye.x, eye.y, eye.z)
                } else {
                    Vector3D::zero()
                }
            },
            CameraType::FirstPerson => {
                if let Some(ref camera) = self.fps_camera {
                    let eye = camera.eye();
                    Vector3D::new(eye.x, eye.y, eye.z)
                } else {
                    Vector3D::zero()
                }
            },
        }
    }

    pub fn get_camera_target(&self) -> Vector3D {
        match self.camera_type {
            CameraType::ArcBall => {
                if let Some(ref camera) = self.arcball_camera {
                    let at = camera.at();
                    Vector3D::new(at.x, at.y, at.z)
                } else {
                    Vector3D::zero()
                }
            },
            CameraType::FirstPerson => {
                if let Some(ref camera) = self.fps_camera {
                    let at = camera.at();
                    Vector3D::new(at.x, at.y, at.z)
                } else {
                    Vector3D::zero()
                }
            },
        }
    }

    // Internal methods
    fn update_fps(&mut self) {
        let now = std::time::Instant::now();
        let delta = now.duration_since(self.last_fps_update).as_secs_f32();
        self.frame_time_accumulator += delta;
        
        if self.frame_time_accumulator >= 1.0 {
            self.fps = self.frame_count as f32 / self.frame_time_accumulator;
            self.frame_count = 0;
            self.frame_time_accumulator = 0.0;
        }
        
        self.last_fps_update = now;
    }

    fn update_camera(&mut self) {
        // Camera updates are handled by kiss3d automatically through event handling
    }

    fn render_scene_objects_static(window: &mut Window, scene: &Scene) {
        // Render all objects in the scene as 3D objects
        let object_ids = scene.get_all_object_ids();

        for object_id in object_ids {
            if let Some(object) = scene.get_object(object_id) {
                let position = object.get_position();
                let size = object.get_size();
                let name = object.get_name();

                // Convert our Vector3D to kiss3d's Point3
                let pos = Point3::new(position.x, position.y, position.z);

                // Create appropriate 3D shapes based on object type
                if name.contains("Platform") || name.contains("Base Plate") {
                    // Render platforms as boxes
                    let mut cube = window.add_cube(size.x, size.y, size.z);
                    cube.set_local_translation(kiss3d::nalgebra::Translation3::new(pos.x, pos.y, pos.z));
                    cube.set_color(0.2, 0.8, 0.2); // Green for platforms
                } else if name.contains("Player") {
                    // Render player as a colored sphere
                    let mut sphere = window.add_sphere(0.5);
                    sphere.set_local_translation(kiss3d::nalgebra::Translation3::new(pos.x, pos.y, pos.z));
                    sphere.set_color(0.2, 0.4, 1.0); // Blue for player
                } else if name.contains("Collectible") {
                    // Render collectibles as small cubes
                    let mut cube = window.add_cube(0.3, 0.3, 0.3);
                    cube.set_local_translation(kiss3d::nalgebra::Translation3::new(pos.x, pos.y, pos.z));
                    cube.set_color(1.0, 1.0, 0.2); // Yellow for collectibles
                } else {
                    // Default: render as a small cube
                    let mut cube = window.add_cube(size.x.max(0.5), size.y.max(0.5), size.z.max(0.5));
                    cube.set_local_translation(kiss3d::nalgebra::Translation3::new(pos.x, pos.y, pos.z));
                    cube.set_color(0.7, 0.7, 0.7); // Gray for unknown objects
                }
            }
        }
    }

    fn render_debug_info(&mut self) {
        // Debug info rendering would be implemented here
        // This could include FPS, camera position, object counts, etc.
        if self.frame_count % 60 == 0 { // Update every second at 60 FPS
            println!("FPS: {:.1}, Camera: {:?}", self.fps, self.get_camera_position());
        }
    }

    // Event handling methods
    pub fn handle_window_events(&mut self) {
        if let Some(window) = &mut self.window {
            // Handle window resize, close, etc.
            for event in window.events().iter() {
            match event.value {
                kiss3d::event::WindowEvent::FramebufferSize(width, height) => {
                    // Handle window resize
                    println!("Window resized to {}x{}", width, height);
                },
                kiss3d::event::WindowEvent::Key(key, action, _) => {
                    if action == kiss3d::event::Action::Press {
                        match key {
                            kiss3d::event::Key::F1 => self.toggle_debug_info(),
                            kiss3d::event::Key::F2 => self.toggle_wireframe(),
                            kiss3d::event::Key::F3 => {
                                let new_type = match self.camera_type {
                                    CameraType::ArcBall => CameraType::FirstPerson,
                                    CameraType::FirstPerson => CameraType::ArcBall,
                                };
                                self.set_camera_type(new_type);
                            },
                            _ => {},
                        }
                    }
                },
                _ => {},
            }
            }
        }
    }

    // Utility methods for accessing the underlying window
    pub fn window(&self) -> Option<&Window> {
        self.window.as_ref()
    }

    pub fn window_mut(&mut self) -> Option<&mut Window> {
        self.window.as_mut()
    }
}

// Rendering utilities
pub struct RenderStats {
    pub triangles_rendered: u64,
    pub draw_calls: u64,
    pub vertices_processed: u64,
    pub textures_bound: u64,
}

impl RenderStats {
    pub fn new() -> Self {
        Self {
            triangles_rendered: 0,
            draw_calls: 0,
            vertices_processed: 0,
            textures_bound: 0,
        }
    }

    pub fn reset(&mut self) {
        self.triangles_rendered = 0;
        self.draw_calls = 0;
        self.vertices_processed = 0;
        self.textures_bound = 0;
    }
}

// Color utilities
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    pub fn white() -> Self {
        Self::rgb(1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::rgb(0.0, 0.0, 0.0)
    }

    pub fn red() -> Self {
        Self::rgb(1.0, 0.0, 0.0)
    }

    pub fn green() -> Self {
        Self::rgb(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Self {
        Self::rgb(0.0, 0.0, 1.0)
    }

    pub fn yellow() -> Self {
        Self::rgb(1.0, 1.0, 0.0)
    }

    pub fn cyan() -> Self {
        Self::rgb(0.0, 1.0, 1.0)
    }

    pub fn magenta() -> Self {
        Self::rgb(1.0, 0.0, 1.0)
    }

    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Self::rgb(r, g, b)
    }

    pub fn to_hex(&self) -> u32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        (r << 16) | (g << 8) | b
    }

    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        Self::new(
            self.r + (other.r - self.r) * t,
            self.g + (other.g - self.g) * t,
            self.b + (other.b - self.b) * t,
            self.a + (other.a - self.a) * t,
        )
    }
}