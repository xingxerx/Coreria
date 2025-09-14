use nalgebra::{Vector3, Point3};
use noise::{NoiseFn, Perlin};
use fnv::FnvHashMap;
use kiss3d::scene::SceneNode;
use std::thread;

pub const CHUNK_SIZE: usize = 8;  // ULTRA PERFORMANCE: Reduced from 16 to 8
pub const CHUNK_HEIGHT: usize = 32; // ULTRA PERFORMANCE: Reduced from 64 to 32
pub const WORLD_HEIGHT: i32 = 32;   // ULTRA PERFORMANCE: Reduced accordingly
pub const SEA_LEVEL: i32 = 16;      // ULTRA PERFORMANCE: Reduced accordingly

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air,
    Stone,
    Dirt,
    Grass,
    Sand,
    Water,
    Bedrock,
}

impl BlockType {
    pub fn is_solid(&self) -> bool {
        match self {
            BlockType::Air | BlockType::Water => false,
            _ => true,
        }
    }

    pub fn color(&self) -> Point3<f32> {
        match self {
            BlockType::Air => Point3::new(0.0, 0.0, 0.0),
            BlockType::Stone => Point3::new(0.5, 0.5, 0.5),
            BlockType::Dirt => Point3::new(0.6, 0.4, 0.2),
            BlockType::Grass => Point3::new(0.2, 0.8, 0.2),
            BlockType::Sand => Point3::new(0.9, 0.8, 0.6),
            BlockType::Water => Point3::new(0.2, 0.4, 0.8),
            BlockType::Wood => Point3::new(0.6, 0.3, 0.1),
            BlockType::Leaves => Point3::new(0.1, 0.6, 0.1),
            BlockType::Bedrock => Point3::new(0.1, 0.1, 0.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub z: i32,
}

impl ChunkCoord {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn from_world_pos(world_x: f32, world_z: f32) -> Self {
        Self {
            x: (world_x / CHUNK_SIZE as f32).floor() as i32,
            z: (world_z / CHUNK_SIZE as f32).floor() as i32,
        }
    }
}

pub struct Block {
    pub block_type: BlockType,
    pub node: Option<SceneNode>,
}

impl Block {
    pub fn new(block_type: BlockType) -> Self {
        Self {
            block_type,
            node: None,
        }
    }
}

pub struct Chunk {
    pub coord: ChunkCoord,
    pub blocks: FnvHashMap<(usize, usize, usize), Block>, // Only store non-air blocks
    pub is_generated: bool,
    pub is_meshed: bool,
}

impl Chunk {
    pub fn new(coord: ChunkCoord) -> Self {
        Self {
            coord,
            blocks: FnvHashMap::default(), // Start with empty HashMap
            is_generated: false,
            is_meshed: false,
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&Block> {
        if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
            self.blocks.get(&(x, y, z))
        } else {
            None
        }
    }

    pub fn get_block_type(&self, x: usize, y: usize, z: usize) -> BlockType {
        if let Some(block) = self.get_block(x, y, z) {
            block.block_type
        } else {
            BlockType::Air // Default to air if no block exists
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block_type: BlockType) {
        if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
            if block_type == BlockType::Air {
                // Remove air blocks to save memory
                self.blocks.remove(&(x, y, z));
            } else {
                self.blocks.insert((x, y, z), Block::new(block_type));
            }
        }
    }

    pub fn world_to_local(&self, world_x: f32, world_z: f32) -> (usize, usize) {
        let local_x = (world_x - (self.coord.x * CHUNK_SIZE as i32) as f32) as usize;
        let local_z = (world_z - (self.coord.z * CHUNK_SIZE as i32) as f32) as usize;
        (local_x.min(CHUNK_SIZE - 1), local_z.min(CHUNK_SIZE - 1))
    }
}

pub struct TerrainGenerator {
    height_noise: Perlin,
    cave_noise: Perlin,
    biome_noise: Perlin,
    seed: u32,
}

impl TerrainGenerator {
    pub fn new(seed: u32) -> Self {
        let height_noise = Perlin::new(seed);
        let cave_noise = Perlin::new(seed + 1);
        let biome_noise = Perlin::new(seed + 2);

        Self {
            height_noise,
            cave_noise,
            biome_noise,
            seed,
        }
    }

    pub fn generate_chunk(&self, chunk: &mut Chunk) {
        let chunk_world_x = chunk.coord.x * CHUNK_SIZE as i32;
        let chunk_world_z = chunk.coord.z * CHUNK_SIZE as i32;

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk_world_x + x as i32;
                let world_z = chunk_world_z + z as i32;

                // Generate height using noise
                let height = self.get_height(world_x as f64, world_z as f64);
                let biome = self.get_biome(world_x as f64, world_z as f64);

                // Generate terrain column
                for y in 0..CHUNK_HEIGHT {
                    let world_y = y as i32;
                    let block_type = self.get_block_type(world_x, world_y, world_z, height, biome);
                    chunk.set_block(x, y, z, block_type);
                }
            }
        }

        chunk.is_generated = true;
    }

    fn get_height(&self, x: f64, z: f64) -> i32 {
        let scale = 0.01;
        let height_noise = self.height_noise.get([x * scale, z * scale]);
        let height = (height_noise * 30.0 + SEA_LEVEL as f64) as i32;
        height.max(1).min(WORLD_HEIGHT - 1)
    }

    fn get_biome(&self, x: f64, z: f64) -> BiomeType {
        let scale = 0.005;
        let biome_noise = self.biome_noise.get([x * scale, z * scale]);
        
        if biome_noise < -0.3 {
            BiomeType::Desert
        } else if biome_noise < 0.3 {
            BiomeType::Plains
        } else {
            BiomeType::Forest
        }
    }

    fn get_block_type(&self, x: i32, y: i32, z: i32, surface_height: i32, biome: BiomeType) -> BlockType {
        // Bedrock at bottom
        if y <= 1 {
            return BlockType::Bedrock;
        }

        // Air above surface
        if y > surface_height {
            if y <= SEA_LEVEL {
                return BlockType::Water;
            } else {
                return BlockType::Air;
            }
        }

        // Cave generation
        let cave_scale = 0.05;
        let cave_noise = self.cave_noise.get([x as f64 * cave_scale, y as f64 * cave_scale, z as f64 * cave_scale]);
        if cave_noise > 0.6 && y > 10 && y < surface_height - 5 {
            return BlockType::Air;
        }

        // Surface blocks based on biome
        if y == surface_height {
            match biome {
                BiomeType::Desert => BlockType::Sand,
                BiomeType::Plains => BlockType::Grass,
                BiomeType::Forest => BlockType::Grass,
            }
        } else if y > surface_height - 4 {
            match biome {
                BiomeType::Desert => BlockType::Sand,
                _ => BlockType::Dirt,
            }
        } else {
            BlockType::Stone
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum BiomeType {
    Plains,
    Forest,
    Desert,
}

pub struct World {
    pub chunks: FnvHashMap<ChunkCoord, Chunk>,
    pub terrain_generator: TerrainGenerator,
    pub loaded_chunks: Vec<ChunkCoord>,
    pub render_distance: i32,
}

impl World {
    pub fn new(seed: u32) -> Self {
        Self {
            chunks: FnvHashMap::default(),
            terrain_generator: TerrainGenerator::new(seed),
            loaded_chunks: Vec::new(),
            render_distance: 8,
        }
    }

    pub fn update_chunks(&mut self, player_pos: Vector3<f32>) {
        let player_chunk = ChunkCoord::from_world_pos(player_pos.x, player_pos.z);
        
        // Load chunks around player
        for x in -self.render_distance..=self.render_distance {
            for z in -self.render_distance..=self.render_distance {
                let chunk_coord = ChunkCoord::new(player_chunk.x + x, player_chunk.z + z);
                
                if !self.chunks.contains_key(&chunk_coord) {
                    let mut chunk = Chunk::new(chunk_coord);
                    self.terrain_generator.generate_chunk(&mut chunk);
                    self.chunks.insert(chunk_coord, chunk);
                    self.loaded_chunks.push(chunk_coord);
                }
            }
        }

        // Unload distant chunks
        self.loaded_chunks.retain(|&coord| {
            let distance = ((coord.x - player_chunk.x).abs().max((coord.z - player_chunk.z).abs()));
            if distance > self.render_distance + 2 {
                self.chunks.remove(&coord);
                false
            } else {
                true
            }
        });
    }

    pub fn get_block_at(&self, world_x: f32, world_y: f32, world_z: f32) -> BlockType {
        let chunk_coord = ChunkCoord::from_world_pos(world_x, world_z);
        
        if let Some(chunk) = self.chunks.get(&chunk_coord) {
            let (local_x, local_z) = chunk.world_to_local(world_x, world_z);
            let local_y = world_y as usize;
            
            if let Some(block) = chunk.get_block(local_x, local_y, local_z) {
                return block.block_type;
            }
        }
        
        BlockType::Air
    }

    pub fn is_solid_at(&self, world_x: f32, world_y: f32, world_z: f32) -> bool {
        self.get_block_at(world_x, world_y, world_z).is_solid()
    }

    /// Find the surface height at a given world position
    /// Returns the Y coordinate of the highest solid block + 1 (safe spawn height)
    pub fn get_surface_height(&self, world_x: f32, world_z: f32) -> f32 {
        // First, ensure the chunk is loaded at this position
        let chunk_coord = ChunkCoord::from_world_pos(world_x, world_z);

        if let Some(chunk) = self.chunks.get(&chunk_coord) {
            let (local_x, local_z) = chunk.world_to_local(world_x, world_z);

            // Search from top to bottom for the first solid block
            for y in (0..CHUNK_HEIGHT).rev() {
                if let Some(block) = chunk.get_block(local_x, y, local_z) {
                    if block.block_type.is_solid() {
                        // Return the height above the solid block (safe spawn position)
                        return (y as f32) + 1.5; // +1.5 to ensure player doesn't spawn inside block
                    }
                }
            }
        }

        // Fallback: return sea level + some height if no surface found
        SEA_LEVEL as f32 + 5.0
    }

    /// Find a safe spawn position near the given coordinates
    /// This ensures the player spawns on solid ground, not underground or in water
    pub fn find_safe_spawn_position(&mut self, preferred_x: f32, preferred_z: f32) -> Vector3<f32> {
        // Generate chunks around the preferred spawn location first
        let spawn_chunk = ChunkCoord::from_world_pos(preferred_x, preferred_z);

        // Load a 3x3 area of chunks around spawn to ensure terrain is generated
        for x in -1..=1 {
            for z in -1..=1 {
                let chunk_coord = ChunkCoord::new(spawn_chunk.x + x, spawn_chunk.z + z);
                if !self.chunks.contains_key(&chunk_coord) {
                    let mut chunk = Chunk::new(chunk_coord);
                    self.terrain_generator.generate_chunk(&mut chunk);
                    self.chunks.insert(chunk_coord, chunk);
                }
            }
        }

        // Try the preferred position first
        let surface_height = self.get_surface_height(preferred_x, preferred_z);
        let spawn_pos = Vector3::new(preferred_x, surface_height, preferred_z);

        // Verify this is a safe position (not in water, not too high)
        if surface_height > SEA_LEVEL as f32 && surface_height < (WORLD_HEIGHT - 5) as f32 {
            println!("🏔️  Safe spawn found at ({:.1}, {:.1}, {:.1})", spawn_pos.x, spawn_pos.y, spawn_pos.z);
            return spawn_pos;
        }

        // If preferred position isn't safe, search in a spiral pattern
        println!("🔍 Searching for safe spawn position...");
        for radius in 1..=10 {
            for angle in 0..8 {
                let angle_rad = (angle as f32) * std::f32::consts::PI / 4.0;
                let search_x = preferred_x + angle_rad.cos() * (radius as f32) * 2.0;
                let search_z = preferred_z + angle_rad.sin() * (radius as f32) * 2.0;

                let search_height = self.get_surface_height(search_x, search_z);

                if search_height > SEA_LEVEL as f32 && search_height < (WORLD_HEIGHT - 5) as f32 {
                    let safe_pos = Vector3::new(search_x, search_height, search_z);
                    println!("🏔️  Safe spawn found at ({:.1}, {:.1}, {:.1}) after search", safe_pos.x, safe_pos.y, safe_pos.z);
                    return safe_pos;
                }
            }
        }

        // Ultimate fallback: spawn at a reasonable height above sea level
        let fallback_pos = Vector3::new(preferred_x, SEA_LEVEL as f32 + 10.0, preferred_z);
        println!("⚠️  Using fallback spawn position at ({:.1}, {:.1}, {:.1})", fallback_pos.x, fallback_pos.y, fallback_pos.z);
        fallback_pos
    }
}
