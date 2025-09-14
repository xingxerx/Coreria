use winit::event::{KeyboardInput, ElementState, VirtualKeyCode};
use cgmath::{Vector3, Matrix4, perspective, Deg, Point3, InnerSpace};
use std::collections::HashSet;
use std::time::{Instant, Duration};

#[derive(Debug, Clone)]
pub struct Player {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub on_ground: bool,
    acceleration: f32,
    max_speed: f32,
    friction: f32,
    jump_force: f32,
}

impl Player {
    pub fn new(position: Vector3<f32>) -> Self {
        Self {
            position,
            velocity: Vector3::new(0.0, 0.0, 0.0),
            on_ground: false,
            acceleration: 25.0,
            max_speed: 6.0,
            friction: 0.88,
            jump_force: 8.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Apply gravity
        if !self.on_ground {
            self.velocity.y -= 20.0 * delta_time;
        }

        // Apply friction
        self.velocity.x *= self.friction;
        self.velocity.z *= self.friction;

        // Update position
        self.position += self.velocity * delta_time;

        // Ground collision (simple)
        if self.position.y < 0.0 {
            self.position.y = 0.0;
            self.velocity.y = 0.0;
            self.on_ground = true;
        } else {
            self.on_ground = false;
        }
    }

    pub fn move_horizontal(&mut self, direction: Vector3<f32>, delta_time: f32) {
        let acceleration = direction * self.acceleration * delta_time;
        self.velocity += acceleration;

        // Clamp to max speed
        let horizontal_speed = (self.velocity.x * self.velocity.x + self.velocity.z * self.velocity.z).sqrt();
        if horizontal_speed > self.max_speed {
            let scale = self.max_speed / horizontal_speed;
            self.velocity.x *= scale;
            self.velocity.z *= scale;
        }
    }

    pub fn jump(&mut self) {
        if self.on_ground {
            self.velocity.y = self.jump_force;
            self.on_ground = false;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Platform {
    pub position: Vector3<f32>, // Retain position since it's essential
    pub size: Vector3<f32>,     // Retain size since it's essential
    pub color: Vector3<f32>,      // Consider using the color for rendering
}

impl Platform {
    pub fn new(position: Vector3<f32>, size: Vector3<f32>, color: Vector3<f32>) -> Self {
        Self { position, size, color }
    }
}

#[derive(Debug, Clone)]
pub struct TimeSystem {
    start_time: Instant,
    cycle_duration: Duration, // 15 minutes total
    day_duration: Duration,   // 10 minutes
}

impl TimeSystem {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            cycle_duration: Duration::from_secs(15 * 60), // 15 minutes
            day_duration: Duration::from_secs(10 * 60),   // 10 minutes
        }
    }

    pub fn get_time_info(&self) -> TimeInfo {
        let elapsed = self.start_time.elapsed();
        let cycle_time = elapsed.as_secs_f32() % self.cycle_duration.as_secs_f32();
        let day_time = self.day_duration.as_secs_f32();
        
        let is_day = cycle_time < day_time;
        let phase_progress = if is_day {
            cycle_time / day_time
        } else {
            (cycle_time - day_time) / (self.cycle_duration.as_secs_f32() - day_time)
        };

        let transition_factor = if is_day {
            // Day: sine wave for smooth lighting
            (phase_progress * std::f32::consts::PI).sin()
        } else {
            // Night: cosine wave for different feel
            (phase_progress * std::f32::consts::PI).cos().abs()
        };

        TimeInfo {
            is_day,
            phase_progress,
            transition_factor,
            cycle_time,
            total_elapsed: elapsed.as_secs_f32(),
            time_of_day: if is_day { phase_progress * 0.5 } else { 0.5 + phase_progress * 0.5 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct TimeInfo {
    pub is_day: bool,             // Keep is_day
    pub phase_progress: f32,      // Could be used for more advanced effects
    pub transition_factor: f32,   // Keep transition_factor
    pub cycle_time: f32,
    pub total_elapsed: f32,
    pub time_of_day: f32, // 0.0 = midnight, 0.5 = noon, 1.0 = midnight
}

pub struct GameState {
    pub player: Player,
    pub platforms: Vec<Platform>,
    pub time_system: TimeSystem,
    pub pressed_keys: HashSet<VirtualKeyCode>,
    pub camera_position: Point3<f32>,
    pub camera_target: Point3<f32>,
    pub view_matrix: Matrix4<f32>,
    pub projection_matrix: Matrix4<f32>,
    last_update: Instant,
}

impl GameState {
    pub fn new() -> Self {
        let player = Player::new(Vector3::new(0.0, 5.0, 0.0));
        
        // Create platforms with Epoch of Elria colors
        let platforms: Vec<Platform> = vec![
            Platform::new(Vector3::new(0.0, -0.25, 0.0), Vector3::new(20.0, 0.5, 20.0), Vector3::new(0.18, 0.25, 0.33)), // Deep blue ground
            Platform::new(Vector3::new(5.0, 2.0, 5.0), Vector3::new(4.0, 0.5, 4.0), Vector3::new(1.0, 0.70, 0.07)),      // Neon orange
            Platform::new(Vector3::new(-3.0, 4.0, -2.0), Vector3::new(3.0, 0.5, 3.0), Vector3::new(0.5, 0.2, 0.8)),     // Purple
            Platform::new(Vector3::new(8.0, 6.0, -5.0), Vector3::new(2.5, 0.5, 2.5), Vector3::new(0.0, 0.75, 1.0)),     // Neon blue
        ];

        let camera_position = Point3::new(0.0, 8.0, 15.0);
        let camera_target = Point3::new(0.0, 0.0, 0.0);
        let view_matrix = Matrix4::look_at_rh(camera_position, camera_target, Vector3::unit_y());
        let projection_matrix = perspective(Deg(45.0), 1280.0 / 720.0, 0.1, 100.0);

        Self {
            player,
            platforms,
            time_system: TimeSystem::new(),
            pressed_keys: HashSet::new(),
            camera_position,
            camera_target,
            view_matrix,
            projection_matrix,
            last_update: Instant::now(),
        }
    }

    pub fn handle_input(&mut self, input: &KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            match input.state {
                ElementState::Pressed => {
                    self.pressed_keys.insert(keycode);
                    if keycode == VirtualKeyCode::Space {
                        self.player.jump();
                    }
                }
                ElementState::Released => {
                    self.pressed_keys.remove(&keycode);
                }
            }
        }
    }

    pub fn update(&mut self) {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(self.last_update).as_secs_f32();
        self.last_update = current_time;

        // Handle movement input
        let mut movement = Vector3::new(0.0, 0.0, 0.0);
        
        if self.pressed_keys.contains(&VirtualKeyCode::W) || self.pressed_keys.contains(&VirtualKeyCode::Up) {
            movement.z -= 1.0;
        }
        if self.pressed_keys.contains(&VirtualKeyCode::S) || self.pressed_keys.contains(&VirtualKeyCode::Down) {
            movement.z += 1.0;
        }
        if self.pressed_keys.contains(&VirtualKeyCode::A) || self.pressed_keys.contains(&VirtualKeyCode::Left) {
            movement.x -= 1.0;
        }
        if self.pressed_keys.contains(&VirtualKeyCode::D) || self.pressed_keys.contains(&VirtualKeyCode::Right) {
            movement.x += 1.0;
        }

        // Normalize diagonal movement
        if movement.x != 0.0 && movement.z != 0.0 {
            movement = movement.normalize();
        }

        self.player.move_horizontal(movement, delta_time);
        self.player.update(delta_time);

        // Update camera to follow player
        self.camera_target = Point3::new(self.player.position.x, self.player.position.y + 1.0, self.player.position.z);
        self.camera_position = Point3::new(
            self.player.position.x + 0.0,
            self.player.position.y + 3.0,
            self.player.position.z + 15.0,
        );
        self.view_matrix = Matrix4::look_at_rh(self.camera_position, self.camera_target, Vector3::unit_y());

        // Reset if player falls too far
        if self.player.position.y < -10.0 {
            self.player.position = Vector3::new(0.0, 5.0, 0.0);
            self.player.velocity = Vector3::new(0.0, 0.0, 0.0);
        }
    }

    pub fn get_time_info(&self) -> TimeInfo {
        self.time_system.get_time_info()
    }
}
