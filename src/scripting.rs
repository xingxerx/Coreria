use crate::math::Vector3D;
use crate::game_objects::Player;
use crate::environment::Environment;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
pub enum ScriptCommand {
    // Player commands
    MovePlayer(Vector3D),
    SetPlayerHealth(i32),
    SetPlayerPosition(Vector3D),
    
    // Environment commands
    ChangeWeather(String),
    SetTimeOfDay(f32),
    SpawnObject(String, Vector3D),
    
    // Game logic commands
    ShowMessage(String),
    PlaySound(String),
    TriggerEvent(String),
    
    // Conditional commands
    If(String, Vec<ScriptCommand>),
    Wait(f32),
    
    // Custom commands
    Custom(String, Vec<String>),
}

#[derive(Debug, Clone)]
pub struct Script {
    pub name: String,
    pub commands: Vec<ScriptCommand>,
    pub variables: HashMap<String, String>,
    pub triggers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ScriptEvent {
    pub name: String,
    pub condition: String,
    pub script_name: String,
    pub cooldown: f32,
    pub last_triggered: f32,
}

pub struct ScriptEngine {
    pub scripts: HashMap<String, Script>,
    pub events: Vec<ScriptEvent>,
    pub global_variables: HashMap<String, String>,
    pub execution_queue: Vec<(String, usize)>, // (script_name, command_index)
    pub wait_timers: HashMap<String, f32>,
}

impl ScriptEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            scripts: HashMap::new(),
            events: Vec::new(),
            global_variables: HashMap::new(),
            execution_queue: Vec::new(),
            wait_timers: HashMap::new(),
        };
        
        // Load default scripts
        engine.load_default_scripts();
        engine
    }

    pub fn load_script_from_file(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let script = self.parse_script(&content)?;
        self.scripts.insert(script.name.clone(), script);
        Ok(())
    }

    pub fn parse_script(&self, content: &str) -> Result<Script, Box<dyn std::error::Error>> {
        let mut script = Script {
            name: "unnamed".to_string(),
            commands: Vec::new(),
            variables: HashMap::new(),
            triggers: Vec::new(),
        };

        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            
            if line.is_empty() || line.starts_with("//") {
                i += 1;
                continue;
            }

            if line.starts_with("script ") {
                script.name = line[7..].trim().to_string();
            } else if line.starts_with("var ") {
                let parts: Vec<&str> = line[4..].split('=').collect();
                if parts.len() == 2 {
                    script.variables.insert(
                        parts[0].trim().to_string(),
                        parts[1].trim().to_string(),
                    );
                }
            } else if line.starts_with("trigger ") {
                script.triggers.push(line[8..].trim().to_string());
            } else {
                // Parse command
                if let Ok(command) = self.parse_command(line) {
                    script.commands.push(command);
                }
            }
            
            i += 1;
        }

        Ok(script)
    }

    fn parse_command(&self, line: &str) -> Result<ScriptCommand, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty command".into());
        }

        match parts[0].to_lowercase().as_str() {
            "move_player" => {
                if parts.len() >= 4 {
                    let x: f32 = parts[1].parse()?;
                    let y: f32 = parts[2].parse()?;
                    let z: f32 = parts[3].parse()?;
                    Ok(ScriptCommand::MovePlayer(Vector3D::new(x, y, z)))
                } else {
                    Err("Invalid move_player command".into())
                }
            },
            "set_health" => {
                if parts.len() >= 2 {
                    let health: i32 = parts[1].parse()?;
                    Ok(ScriptCommand::SetPlayerHealth(health))
                } else {
                    Err("Invalid set_health command".into())
                }
            },
            "set_position" => {
                if parts.len() >= 4 {
                    let x: f32 = parts[1].parse()?;
                    let y: f32 = parts[2].parse()?;
                    let z: f32 = parts[3].parse()?;
                    Ok(ScriptCommand::SetPlayerPosition(Vector3D::new(x, y, z)))
                } else {
                    Err("Invalid set_position command".into())
                }
            },
            "change_weather" => {
                if parts.len() >= 2 {
                    Ok(ScriptCommand::ChangeWeather(parts[1].to_string()))
                } else {
                    Err("Invalid change_weather command".into())
                }
            },
            "set_time" => {
                if parts.len() >= 2 {
                    let time: f32 = parts[1].parse()?;
                    Ok(ScriptCommand::SetTimeOfDay(time))
                } else {
                    Err("Invalid set_time command".into())
                }
            },
            "spawn" => {
                if parts.len() >= 5 {
                    let object_type = parts[1].to_string();
                    let x: f32 = parts[2].parse()?;
                    let y: f32 = parts[3].parse()?;
                    let z: f32 = parts[4].parse()?;
                    Ok(ScriptCommand::SpawnObject(object_type, Vector3D::new(x, y, z)))
                } else {
                    Err("Invalid spawn command".into())
                }
            },
            "message" => {
                let message = line[8..].to_string(); // Skip "message "
                Ok(ScriptCommand::ShowMessage(message))
            },
            "sound" => {
                if parts.len() >= 2 {
                    Ok(ScriptCommand::PlaySound(parts[1].to_string()))
                } else {
                    Err("Invalid sound command".into())
                }
            },
            "event" => {
                if parts.len() >= 2 {
                    Ok(ScriptCommand::TriggerEvent(parts[1].to_string()))
                } else {
                    Err("Invalid event command".into())
                }
            },
            "wait" => {
                if parts.len() >= 2 {
                    let duration: f32 = parts[1].parse()?;
                    Ok(ScriptCommand::Wait(duration))
                } else {
                    Err("Invalid wait command".into())
                }
            },
            _ => {
                // Custom command
                let command_name = parts[0].to_string();
                let args = parts[1..].iter().map(|s| s.to_string()).collect();
                Ok(ScriptCommand::Custom(command_name, args))
            }
        }
    }

    pub fn execute_script(&mut self, script_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.scripts.contains_key(script_name) {
            self.execution_queue.push((script_name.to_string(), 0));
            println!("📜 Executing script: {}", script_name);
        } else {
            return Err(format!("Script '{}' not found", script_name).into());
        }
        Ok(())
    }

    pub fn update(&mut self, delta_time: f32, player: &mut Player, environment: &mut Environment) {
        // Update wait timers
        let mut completed_waits = Vec::new();
        for (script_name, timer) in &mut self.wait_timers {
            *timer -= delta_time;
            if *timer <= 0.0 {
                completed_waits.push(script_name.clone());
            }
        }

        // Resume scripts that finished waiting
        for script_name in completed_waits {
            self.wait_timers.remove(&script_name);
        }

        // Simple script execution - just execute the first script in queue
        if !self.execution_queue.is_empty() {
            let (script_name, _) = self.execution_queue[0].clone();
            if let Some(script) = self.scripts.get(&script_name).cloned() {
                for command in &script.commands {
                    if let Err(e) = self.execute_command(command, player, environment) {
                        eprintln!("Script execution error: {}", e);
                        break;
                    }
                }
            }
            self.execution_queue.clear();
        }

        // Check for triggered events (simplified)
        let mut events_to_trigger = Vec::new();

        // First pass: collect events that need to be triggered
        for (i, event) in self.events.iter().enumerate() {
            if event.last_triggered >= event.cooldown {
                if self.evaluate_condition(&event.condition, player, environment) {
                    events_to_trigger.push((i, event.script_name.clone()));
                }
            }
        }

        // Second pass: update timers and trigger events
        for (i, script_name) in events_to_trigger {
            self.execution_queue.push((script_name, 0));
            self.events[i].last_triggered = 0.0;
        }

        // Update all event timers
        for event in &mut self.events {
            event.last_triggered += delta_time;
        }
    }

    fn execute_command(
        &mut self,
        command: &ScriptCommand,
        player: &mut Player,
        environment: &mut Environment,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match command {
            ScriptCommand::MovePlayer(position) => {
                player.position = *position;
                println!("📜 Player moved to: {:?}", position);
                Ok(true)
            },
            ScriptCommand::SetPlayerHealth(health) => {
                player.health = *health;
                println!("📜 Player health set to: {}", health);
                Ok(true)
            },
            ScriptCommand::SetPlayerPosition(position) => {
                player.position = *position;
                println!("📜 Player position set to: {:?}", position);
                Ok(true)
            },
            ScriptCommand::ChangeWeather(weather) => {
                // This would need to be implemented in the environment
                println!("📜 Weather changed to: {}", weather);
                Ok(true)
            },
            ScriptCommand::SetTimeOfDay(time) => {
                environment.current_time = *time;
                println!("📜 Time set to: {:.1}:00", time);
                Ok(true)
            },
            ScriptCommand::SpawnObject(object_type, position) => {
                println!("📜 Spawned {} at {:?}", object_type, position);
                Ok(true)
            },
            ScriptCommand::ShowMessage(message) => {
                println!("💬 {}", message);
                Ok(true)
            },
            ScriptCommand::PlaySound(sound) => {
                println!("🔊 Playing sound: {}", sound);
                Ok(true)
            },
            ScriptCommand::TriggerEvent(event) => {
                println!("⚡ Event triggered: {}", event);
                Ok(true)
            },
            ScriptCommand::Wait(duration) => {
                // Start waiting
                self.wait_timers.insert("current_script".to_string(), *duration);
                println!("⏳ Waiting for {:.1} seconds", duration);
                Ok(false) // Don't continue to next command yet
            },
            ScriptCommand::Custom(command, args) => {
                println!("🔧 Custom command: {} with args: {:?}", command, args);
                Ok(true)
            },
            _ => Ok(true),
        }
    }



    fn evaluate_condition(&self, condition: &str, player: &Player, environment: &Environment) -> bool {
        // Simple condition evaluation
        match condition {
            "player_low_health" => player.health < 30,
            "night_time" => environment.current_time > 19.0 || environment.current_time < 6.0,
            "rainy_weather" => matches!(environment.weather, crate::environment::WeatherType::Rainy),
            _ => false,
        }
    }

    fn load_default_scripts(&mut self) {
        // Welcome script
        let welcome_script = Script {
            name: "welcome".to_string(),
            commands: vec![
                ScriptCommand::ShowMessage("🌟 Welcome to Epoch of Elria!".to_string()),
                ScriptCommand::ShowMessage("🎮 Use WASD to move around the world.".to_string()),
                ScriptCommand::PlaySound("welcome.wav".to_string()),
            ],
            variables: HashMap::new(),
            triggers: vec!["game_start".to_string()],
        };

        // Healing script
        let healing_script = Script {
            name: "heal_player".to_string(),
            commands: vec![
                ScriptCommand::ShowMessage("✨ You feel a warm healing energy...".to_string()),
                ScriptCommand::SetPlayerHealth(100),
                ScriptCommand::PlaySound("heal.wav".to_string()),
            ],
            variables: HashMap::new(),
            triggers: vec!["low_health".to_string()],
        };

        // Weather change script
        let weather_script = Script {
            name: "dynamic_weather".to_string(),
            commands: vec![
                ScriptCommand::ShowMessage("🌤️ The weather is changing...".to_string()),
                ScriptCommand::ChangeWeather("rainy".to_string()),
                ScriptCommand::Wait(10.0),
                ScriptCommand::ChangeWeather("clear".to_string()),
                ScriptCommand::ShowMessage("☀️ The sun comes out again!".to_string()),
            ],
            variables: HashMap::new(),
            triggers: Vec::new(),
        };

        self.scripts.insert("welcome".to_string(), welcome_script);
        self.scripts.insert("heal_player".to_string(), healing_script);
        self.scripts.insert("dynamic_weather".to_string(), weather_script);

        // Add some events
        self.events.push(ScriptEvent {
            name: "low_health_event".to_string(),
            condition: "player_low_health".to_string(),
            script_name: "heal_player".to_string(),
            cooldown: 30.0,
            last_triggered: 0.0,
        });
    }

    pub fn list_scripts(&self) -> Vec<String> {
        self.scripts.keys().cloned().collect()
    }

    pub fn get_script_info(&self, script_name: &str) -> Option<String> {
        if let Some(script) = self.scripts.get(script_name) {
            Some(format!(
                "📜 Script: {}\n\
                 📋 Commands: {}\n\
                 🔧 Variables: {}\n\
                 ⚡ Triggers: {:?}",
                script.name,
                script.commands.len(),
                script.variables.len(),
                script.triggers
            ))
        } else {
            None
        }
    }
}
