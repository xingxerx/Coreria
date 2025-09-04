use crate::math::Vector3D;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum BiomeType {
    Forest,
    Desert,
    Mountains,
    Ocean,
    Plains,
    Tundra,
    Swamp,
    Volcanic,
}

#[derive(Debug, Clone)]
pub enum WeatherType {
    Clear,
    Cloudy,
    Rainy,
    Stormy,
    Snowy,
    Foggy,
    Windy,
}

#[derive(Debug, Clone)]
pub enum TimeOfDay {
    Dawn,
    Morning,
    Noon,
    Afternoon,
    Dusk,
    Night,
    Midnight,
}

#[derive(Debug, Clone)]
pub struct EnvironmentSettings {
    pub ambient_light: (f32, f32, f32),
    pub fog_density: f32,
    pub fog_color: (f32, f32, f32),
    pub sky_color: (f32, f32, f32),
    pub wind_strength: f32,
    pub wind_direction: Vector3D,
    pub temperature: f32,
    pub humidity: f32,
}

#[derive(Debug, Clone)]
pub struct Terrain {
    pub height_map: Vec<Vec<f32>>,
    pub texture_map: Vec<Vec<String>>,
    pub width: usize,
    pub height: usize,
    pub scale: f32,
}

#[derive(Debug, Clone)]
pub struct EnvironmentObject {
    pub id: String,
    pub object_type: String,
    pub position: Vector3D,
    pub rotation: Vector3D,
    pub scale: Vector3D,
    pub properties: HashMap<String, String>,
}

pub struct Environment {
    pub biome: BiomeType,
    pub weather: WeatherType,
    pub time_of_day: TimeOfDay,
    pub settings: EnvironmentSettings,
    pub terrain: Terrain,
    pub objects: Vec<EnvironmentObject>,
    pub day_cycle_speed: f32,
    pub weather_change_timer: f32,
    pub current_time: f32, // 0.0 to 24.0 hours
}

impl Environment {
    pub fn new() -> Self {
        Self {
            biome: BiomeType::Forest,
            weather: WeatherType::Clear,
            time_of_day: TimeOfDay::Morning,
            settings: EnvironmentSettings::default(),
            terrain: Terrain::generate_default(),
            objects: Vec::new(),
            day_cycle_speed: 1.0,
            weather_change_timer: 0.0,
            current_time: 8.0, // Start at 8 AM
        }
    }

    pub fn create_forest_environment() -> Self {
        let mut env = Self::new();
        env.biome = BiomeType::Forest;
        env.settings = EnvironmentSettings {
            ambient_light: (0.3, 0.4, 0.2),
            fog_density: 0.1,
            fog_color: (0.8, 0.9, 0.8),
            sky_color: (0.4, 0.6, 0.8),
            wind_strength: 0.3,
            wind_direction: Vector3D::new(1.0, 0.0, 0.5),
            temperature: 18.0,
            humidity: 0.7,
        };
        
        // Add forest objects
        env.add_trees();
        env.add_rocks();
        env.add_vegetation();
        
        env
    }

    pub fn create_desert_environment() -> Self {
        let mut env = Self::new();
        env.biome = BiomeType::Desert;
        env.weather = WeatherType::Clear;
        env.settings = EnvironmentSettings {
            ambient_light: (0.9, 0.8, 0.6),
            fog_density: 0.05,
            fog_color: (1.0, 0.9, 0.7),
            sky_color: (0.8, 0.7, 0.5),
            wind_strength: 0.5,
            wind_direction: Vector3D::new(1.0, 0.0, 0.0),
            temperature: 35.0,
            humidity: 0.1,
        };
        
        // Add desert objects
        env.add_cacti();
        env.add_sand_dunes();
        env.add_oasis();
        
        env
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update day/night cycle
        self.current_time += delta_time * self.day_cycle_speed;
        if self.current_time >= 24.0 {
            self.current_time -= 24.0;
        }
        
        // Update time of day
        self.update_time_of_day();
        
        // Update weather
        self.weather_change_timer += delta_time;
        if self.weather_change_timer > 300.0 { // Change weather every 5 minutes
            self.change_weather();
            self.weather_change_timer = 0.0;
        }
        
        // Update environment settings based on time and weather
        self.update_lighting();
        self.update_atmosphere();
    }

    fn update_time_of_day(&mut self) {
        self.time_of_day = match self.current_time {
            t if t >= 5.0 && t < 7.0 => TimeOfDay::Dawn,
            t if t >= 7.0 && t < 11.0 => TimeOfDay::Morning,
            t if t >= 11.0 && t < 13.0 => TimeOfDay::Noon,
            t if t >= 13.0 && t < 17.0 => TimeOfDay::Afternoon,
            t if t >= 17.0 && t < 19.0 => TimeOfDay::Dusk,
            t if t >= 19.0 && t < 23.0 => TimeOfDay::Night,
            _ => TimeOfDay::Midnight,
        };
    }

    fn update_lighting(&mut self) {
        let base_light = match self.time_of_day {
            TimeOfDay::Dawn => (0.6, 0.4, 0.3),
            TimeOfDay::Morning => (0.8, 0.8, 0.7),
            TimeOfDay::Noon => (1.0, 1.0, 1.0),
            TimeOfDay::Afternoon => (0.9, 0.8, 0.7),
            TimeOfDay::Dusk => (0.7, 0.5, 0.4),
            TimeOfDay::Night => (0.2, 0.2, 0.3),
            TimeOfDay::Midnight => (0.1, 0.1, 0.2),
        };
        
        // Modify lighting based on weather
        let weather_modifier = match self.weather {
            WeatherType::Clear => 1.0,
            WeatherType::Cloudy => 0.7,
            WeatherType::Rainy => 0.5,
            WeatherType::Stormy => 0.3,
            WeatherType::Snowy => 0.6,
            WeatherType::Foggy => 0.4,
            WeatherType::Windy => 0.9,
        };
        
        self.settings.ambient_light = (
            base_light.0 * weather_modifier,
            base_light.1 * weather_modifier,
            base_light.2 * weather_modifier,
        );
    }

    fn update_atmosphere(&mut self) {
        // Update fog based on weather and time
        self.settings.fog_density = match self.weather {
            WeatherType::Foggy => 0.8,
            WeatherType::Rainy => 0.4,
            WeatherType::Stormy => 0.6,
            _ => 0.1,
        };
        
        // Update wind
        if matches!(self.weather, WeatherType::Stormy | WeatherType::Windy) {
            self.settings.wind_strength = 1.0;
        } else {
            self.settings.wind_strength = 0.3;
        }
    }

    fn change_weather(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        self.weather = match rng.gen_range(0..7) {
            0 => WeatherType::Clear,
            1 => WeatherType::Cloudy,
            2 => WeatherType::Rainy,
            3 => WeatherType::Stormy,
            4 => WeatherType::Snowy,
            5 => WeatherType::Foggy,
            _ => WeatherType::Windy,
        };
    }

    pub fn add_environment_object(&mut self, obj: EnvironmentObject) {
        self.objects.push(obj);
    }

    pub fn get_height_at(&self, x: f32, z: f32) -> f32 {
        // Simple height map lookup
        let map_x = ((x / self.terrain.scale) as usize).min(self.terrain.width - 1);
        let map_z = ((z / self.terrain.scale) as usize).min(self.terrain.height - 1);
        self.terrain.height_map[map_z][map_x]
    }

    pub fn get_environment_info(&self) -> String {
        format!(
            "🌍 Environment Status:\n\
             🏞️  Biome: {:?}\n\
             🌤️  Weather: {:?}\n\
             🕐 Time: {:?} ({:.1}:00)\n\
             🌡️  Temperature: {:.1}°C\n\
             💨 Wind: {:.1} m/s\n\
             🌫️  Fog Density: {:.1}",
            self.biome,
            self.weather,
            self.time_of_day,
            self.current_time,
            self.settings.temperature,
            self.settings.wind_strength,
            self.settings.fog_density
        )
    }

    // Helper methods for adding objects
    fn add_trees(&mut self) {
        for i in 0..20 {
            let tree = EnvironmentObject {
                id: format!("tree_{}", i),
                object_type: "tree".to_string(),
                position: Vector3D::new(
                    (i as f32 * 5.0) - 50.0,
                    0.0,
                    (i as f32 * 3.0) - 30.0,
                ),
                rotation: Vector3D::new(0.0, i as f32 * 0.5, 0.0),
                scale: Vector3D::new(1.0, 2.0 + (i as f32 * 0.1), 1.0),
                properties: HashMap::new(),
            };
            self.objects.push(tree);
        }
    }

    fn add_rocks(&mut self) {
        for i in 0..10 {
            let rock = EnvironmentObject {
                id: format!("rock_{}", i),
                object_type: "rock".to_string(),
                position: Vector3D::new(
                    (i as f32 * 8.0) - 40.0,
                    0.0,
                    (i as f32 * 6.0) - 30.0,
                ),
                rotation: Vector3D::new(0.0, i as f32 * 0.8, 0.0),
                scale: Vector3D::new(0.5 + (i as f32 * 0.1), 0.5, 0.5),
                properties: HashMap::new(),
            };
            self.objects.push(rock);
        }
    }

    fn add_vegetation(&mut self) {
        for i in 0..30 {
            let grass = EnvironmentObject {
                id: format!("grass_{}", i),
                object_type: "grass".to_string(),
                position: Vector3D::new(
                    (i as f32 * 3.0) - 45.0,
                    0.0,
                    (i as f32 * 2.0) - 30.0,
                ),
                rotation: Vector3D::new(0.0, 0.0, 0.0),
                scale: Vector3D::new(0.2, 0.3, 0.2),
                properties: HashMap::new(),
            };
            self.objects.push(grass);
        }
    }

    fn add_cacti(&mut self) {
        for i in 0..8 {
            let cactus = EnvironmentObject {
                id: format!("cactus_{}", i),
                object_type: "cactus".to_string(),
                position: Vector3D::new(
                    (i as f32 * 12.0) - 48.0,
                    0.0,
                    (i as f32 * 8.0) - 32.0,
                ),
                rotation: Vector3D::new(0.0, i as f32 * 0.3, 0.0),
                scale: Vector3D::new(0.8, 1.5, 0.8),
                properties: HashMap::new(),
            };
            self.objects.push(cactus);
        }
    }

    fn add_sand_dunes(&mut self) {
        for i in 0..5 {
            let dune = EnvironmentObject {
                id: format!("dune_{}", i),
                object_type: "sand_dune".to_string(),
                position: Vector3D::new(
                    (i as f32 * 20.0) - 40.0,
                    0.0,
                    (i as f32 * 15.0) - 30.0,
                ),
                rotation: Vector3D::new(0.0, i as f32 * 0.6, 0.0),
                scale: Vector3D::new(5.0, 2.0, 3.0),
                properties: HashMap::new(),
            };
            self.objects.push(dune);
        }
    }

    fn add_oasis(&mut self) {
        let oasis = EnvironmentObject {
            id: "oasis_1".to_string(),
            object_type: "oasis".to_string(),
            position: Vector3D::new(0.0, 0.0, 0.0),
            rotation: Vector3D::new(0.0, 0.0, 0.0),
            scale: Vector3D::new(8.0, 1.0, 8.0),
            properties: HashMap::new(),
        };
        self.objects.push(oasis);
    }
}

impl EnvironmentSettings {
    pub fn default() -> Self {
        Self {
            ambient_light: (0.5, 0.5, 0.5),
            fog_density: 0.1,
            fog_color: (0.8, 0.8, 0.8),
            sky_color: (0.5, 0.7, 1.0),
            wind_strength: 0.3,
            wind_direction: Vector3D::new(1.0, 0.0, 0.0),
            temperature: 20.0,
            humidity: 0.5,
        }
    }
}

impl Terrain {
    pub fn generate_default() -> Self {
        let width = 100;
        let height = 100;
        let mut height_map = vec![vec![0.0; width]; height];
        let mut texture_map = vec![vec!["grass".to_string(); width]; height];
        
        // Generate simple height map
        for z in 0..height {
            for x in 0..width {
                let noise = (x as f32 * 0.1).sin() * (z as f32 * 0.1).cos() * 2.0;
                height_map[z][x] = noise;
                
                // Set texture based on height
                texture_map[z][x] = if noise > 1.0 {
                    "rock".to_string()
                } else if noise < -1.0 {
                    "water".to_string()
                } else {
                    "grass".to_string()
                };
            }
        }
        
        Self {
            height_map,
            texture_map,
            width,
            height,
            scale: 1.0,
        }
    }
}
