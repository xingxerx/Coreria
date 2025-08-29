use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

#[derive(Debug, Clone)]
struct GameObject {
    name: String,
    position: Vector3D,
    properties: HashMap<String, String>,
}

impl GameObject {
    fn new(name: String, position: Vector3D) -> Self {
        Self {
            name,
            position,
            properties: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Character {
    name: String,
    description: String,
    position: Vector3D,
    health: i32,
    abilities: Vec<String>,
}

impl Character {
    fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            position: Vector3D::new(0.0, 0.0, 0.0),
            health: 100,
            abilities: Vec::new(),
        }
    }
    
    fn add_ability(&mut self, ability: &str) {
        self.abilities.push(ability.to_string());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                EPOCH OF ELRIA GAME ENGINE                   ║");
    println!("║                     Version 0.1.0                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    
    println!("🌟 Welcome to Epoch of Elria - Rust Edition! 🌟");
    println!();
    println!("Choose your adventure:");
    println!("1. 🎮 Complete Demo - Full game experience");
    println!("2. 🎭 Dream Weaver Mode - Narrative experience");
    println!("3. 💻 Terminal Mode - Classic text adventure");
    println!("4. 🚀 Engine Test - Technical demo");
    println!("5. ❌ Exit");
    println!();
    print!("Enter your choice (1-5): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();
    
    match choice {
        "1" => run_complete_demo()?,
        "2" => run_dream_weaver_mode()?,
        "3" => run_terminal_mode()?,
        "4" => run_engine_test()?,
        "5" => {
            println!("👋 Thanks for trying Epoch of Elria!");
            return Ok(());
        },
        _ => {
            println!("Invalid choice. Please run the program again.");
            return Ok(());
        }
    }

    Ok(())
}

fn run_complete_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 Starting Complete Demo Mode...");
    println!("This combines all game components into one experience!");
    println!();
    
    let mut game_state = create_demo_state();
    let mut running = true;
    
    while running {
        // Clear screen
        print!("\x1B[2J\x1B[H");
        
        // Render game state
        render_demo_frame(&game_state);
        
        // Get user input
        print!("\nEnter command (w/a/s/d to move, c for characters, q to quit): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();
        
        match input.as_str() {
            "w" => game_state.player_y -= 1.0,
            "s" => game_state.player_y += 1.0,
            "a" => game_state.player_x -= 1.0,
            "d" => game_state.player_x += 1.0,
            "c" => show_characters(),
            "q" => running = false,
            _ => println!("Unknown command: {}", input),
        }
        
        game_state.time += 1.0;
        game_state.score += 10;
    }
    
    println!("Thanks for playing! Final score: {}", game_state.score);
    Ok(())
}

fn run_engine_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Engine Test Mode...");
    println!("Testing core game engine components...");
    println!();
    
    // Test basic math
    println!("📐 Testing Vector3D math...");
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(4.0, 5.0, 6.0);
    let v3 = v1.add(&v2);
    println!("Vector addition: {:?} + {:?} = {:?}", v1, v2, v3);
    println!("Vector magnitude: |{:?}| = {:.2}", v1, v1.length());
    println!();
    
    // Test game objects
    println!("🎮 Testing Game Objects...");
    let player = GameObject::new("Player".to_string(), Vector3D::new(0.0, 1.0, 0.0));
    let collectible = GameObject::new("Crystal".to_string(), Vector3D::new(2.0, 1.0, 0.0));
    println!("Created player at: {:?}", player.position);
    println!("Created collectible at: {:?}", collectible.position);
    println!();
    
    // Test character system
    println!("👥 Testing Character System...");
    let mut xing = Character::new("Xing (The Weaver)", "Master of stories and reality architecture");
    xing.add_ability("Weave Platform");
    xing.add_ability("Create Story Sanctuary");
    println!("Character: {}", xing.name);
    println!("Description: {}", xing.description);
    println!("Abilities: {:?}", xing.abilities);
    println!();
    
    println!("✅ All engine tests completed successfully!");
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(())
}

fn run_dream_weaver_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("✨ Starting Dream Weaver Mode...");
    println!("The cosmic battle for narrative freedom begins!");
    println!();
    
    show_dream_weaver_intro();
    
    let characters = vec![
        "Xing (The Weaver) - Master of stories and reality architecture",
        "Xerx (The Liberator) - Fighter against mental oppression", 
        "The Heart - Catalyst of narrative potential",
        "Lyra (Pure Melody) - Awakener of consciousness through harmony",
    ];
    
    println!("🎭 CHARACTERS:");
    for (i, character) in characters.iter().enumerate() {
        println!("{}. {}", i + 1, character);
    }
    
    println!("\nPress Enter to begin the narrative journey...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(())
}

fn run_terminal_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("💻 Starting Terminal Mode...");
    println!("Classic text-based adventure experience!");
    
    let mut score = 0;
    let mut running = true;
    
    while running {
        println!("\n═══════════════════════════════════════");
        println!("🌟 EPOCH OF ELRIA - TERMINAL MODE 🌟");
        println!("═══════════════════════════════════════");
        println!("Score: {}", score);
        println!();
        println!("1. Explore the Metaverse");
        println!("2. Character Interactions");
        println!("3. Reality Manipulation");
        println!("4. Narrative Combat");
        println!("5. View Statistics");
        println!("6. Quit");
        println!();
        print!("Choose an option (1-6): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => {
                println!("🌌 You explore the infinite Metaverse...");
                println!("Reality shifts around you as stories manifest!");
                let bonus = 50;
                score += bonus;
                println!("You gained {} points!", bonus);
            },
            "2" => {
                println!("👥 You interact with the Dream Weaver characters...");
                println!("Their unique abilities resonate with your consciousness!");
                let bonus = 75;
                score += bonus;
                println!("You gained {} points!", bonus);
            },
            "3" => {
                println!("🌀 You manipulate the fabric of reality...");
                println!("New platforms materialize from pure narrative energy!");
                let bonus = 100;
                score += bonus;
                println!("You gained {} points!", bonus);
            },
            "4" => {
                println!("⚔️ You engage in narrative combat with The One...");
                println!("Creativity clashes against absolute order!");
                let bonus = 150;
                score += bonus;
                println!("You gained {} points!", bonus);
            },
            "5" => {
                println!("📊 STATISTICS:");
                println!("Current Score: {}", score);
                println!("Game Version: 0.1.0");
                println!("Engine Status: Active");
                println!("Platform: Rust");
            },
            "6" => {
                running = false;
                println!("Thanks for playing! Final score: {}", score);
            },
            _ => println!("Invalid option. Please choose 1-6."),
        }
    }
    
    Ok(())
}

#[derive(Debug)]
struct DemoState {
    player_x: f64,
    player_y: f64,
    score: i32,
    time: f64,
}

fn create_demo_state() -> DemoState {
    DemoState {
        player_x: 0.0,
        player_y: 0.0,
        score: 0,
        time: 0.0,
    }
}

fn render_demo_frame(state: &DemoState) {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    EPOCH OF ELRIA DEMO                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!("Player Position: ({:.1}, {:.1})", state.player_x, state.player_y);
    println!("Score: {} | Time: {:.1}s", state.score, state.time);
    println!();
    
    // Simple ASCII world
    println!("World View:");
    println!("┌────────────────────────────────────────┐");
    for y in -5..=5 {
        print!("│");
        for x in -19..=19 {
            if (state.player_x as i32 - x).abs() <= 1 && (state.player_y as i32 - y).abs() <= 1 {
                print!("@");
            } else if x == 0 && y == 0 {
                print!("●");
            } else if x % 5 == 0 && y % 3 == 0 {
                print!("■");
            } else {
                print!(" ");
            }
        }
        println!("│");
    }
    println!("└────────────────────────────────────────┘");
    println!("@ = Player, ● = Earth, ■ = Platform");
}

fn show_characters() {
    println!("\n🎭 DREAM WEAVER CHARACTERS:");
    println!("═══════════════════════════════════════");
    println!("1. Xing (The Weaver)");
    println!("   📝 Master of stories and reality architecture");
    println!("   🎯 Abilities: Weave Platform, Create Story Sanctuary, Anchor Reality");
    println!();
    println!("2. Xerx (The Liberator)");
    println!("   📝 Fighter against mental oppression");
    println!("   🎯 Abilities: Liberate Narrative, Break Mental Chains, Awaken Truth");
    println!();
    println!("3. The Heart");
    println!("   📝 Catalyst of narrative potential");
    println!("   🎯 Abilities: Story Catalyst, Amplify Emotions, Connect Souls");
    println!();
    println!("4. Lyra (Pure Melody)");
    println!("   📝 Awakener of consciousness through harmony");
    println!("   🎯 Abilities: Harmonic Resonance, Awaken Consciousness, Purify Sound");
    println!();
    println!("Press Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn show_dream_weaver_intro() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                THE DREAM WEAVER'S HEART                     ║");
    println!("║              Complete Metaverse Experience                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("The cosmic battle for narrative freedom begins in the infinite Metaverse!");
    println!("Four heroes stand against The One's absolute order...");
    println!();
    println!("🎯 GOAL: Transform The One through collaborative storytelling!");
    println!("Use each character's unique abilities to weave a new reality!");
    println!();
}
