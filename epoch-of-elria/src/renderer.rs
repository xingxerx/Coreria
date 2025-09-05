use crate::world::{World, BlockType, ChunkCoord, CHUNK_SIZE, CHUNK_HEIGHT};
use kiss3d::window::Window;
use kiss3d::scene::SceneNode;
use nalgebra::{Vector3, Translation3};
use std::collections::HashMap;

pub struct BlockRenderer {
    block_nodes: HashMap<(ChunkCoord, usize, usize, usize), SceneNode>,
    rendered_chunks: Vec<ChunkCoord>,
}

impl BlockRenderer {
    pub fn new() -> Self {
        Self {
            block_nodes: HashMap::new(),
            rendered_chunks: Vec::new(),
        }
    }

    pub fn update_rendering(&mut self, world: &World, window: &mut Window, player_pos: Vector3<f32>) {
        let player_chunk = ChunkCoord::from_world_pos(player_pos.x, player_pos.z);
        let render_distance = 4; // Smaller render distance for performance

        // Remove nodes for chunks that are too far away
        self.rendered_chunks.retain(|&chunk_coord| {
            let distance = (chunk_coord.x - player_chunk.x).abs().max((chunk_coord.z - player_chunk.z).abs());
            if distance > render_distance + 1 {
                // Remove all block nodes for this chunk
                for x in 0..CHUNK_SIZE {
                    for y in 0..CHUNK_HEIGHT {
                        for z in 0..CHUNK_SIZE {
                            let key = (chunk_coord, x, y, z);
                            if let Some(mut node) = self.block_nodes.remove(&key) {
                                window.remove_node(&mut node);
                            }
                        }
                    }
                }
                false
            } else {
                true
            }
        });

        // Render chunks around player
        for x in -render_distance..=render_distance {
            for z in -render_distance..=render_distance {
                let chunk_coord = ChunkCoord::new(player_chunk.x + x, player_chunk.z + z);
                
                if !self.rendered_chunks.contains(&chunk_coord) {
                    if let Some(chunk) = world.chunks.get(&chunk_coord) {
                        self.render_chunk(chunk_coord, chunk, window);
                        self.rendered_chunks.push(chunk_coord);
                    }
                }
            }
        }
    }

    fn render_chunk(&mut self, chunk_coord: ChunkCoord, chunk: &crate::world::Chunk, window: &mut Window) {
        let chunk_world_x = chunk_coord.x * CHUNK_SIZE as i32;
        let chunk_world_z = chunk_coord.z * CHUNK_SIZE as i32;

        // Iterate through all blocks in the chunk (only non-air blocks are stored)
        for ((x, y, z), block) in &chunk.blocks {
            if self.should_render_block(chunk, *x, *y, *z) {
                let world_x = chunk_world_x + *x as i32;
                let world_y = *y as i32;
                let world_z = chunk_world_z + *z as i32;

                let mut cube = window.add_cube(1.0, 1.0, 1.0);
                cube.set_local_translation(Translation3::new(
                    world_x as f32,
                    world_y as f32,
                    world_z as f32,
                ));
                cube.set_color(
                    block.block_type.color().x,
                    block.block_type.color().y,
                    block.block_type.color().z,
                );

                let key = (chunk_coord, *x, *y, *z);
                self.block_nodes.insert(key, cube);
            }
        }
    }

    fn should_render_block(&self, chunk: &crate::world::Chunk, x: usize, y: usize, z: usize) -> bool {
        // Only render blocks that have at least one face exposed to air
        let directions = [
            (0, 1, 0),  // Up
            (0, -1, 0), // Down
            (1, 0, 0),  // Right
            (-1, 0, 0), // Left
            (0, 0, 1),  // Forward
            (0, 0, -1), // Back
        ];

        for (dx, dy, dz) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            let nz = z as i32 + dz;

            // Check bounds
            if nx < 0 || nx >= CHUNK_SIZE as i32 ||
               ny < 0 || ny >= CHUNK_HEIGHT as i32 ||
               nz < 0 || nz >= CHUNK_SIZE as i32 {
                return true; // Edge of chunk, assume exposed
            }

            // Check if neighbor is air (no block stored = air)
            let neighbor_type = chunk.get_block_type(nx as usize, ny as usize, nz as usize);
            if neighbor_type == BlockType::Air {
                return true; // Has exposed face
            }
        }

        false // Completely surrounded
    }

    pub fn cleanup(&mut self, window: &mut Window) {
        for (_, mut node) in self.block_nodes.drain() {
            window.remove_node(&mut node);
        }
        self.rendered_chunks.clear();
    }
}

// Optimized rendering for better performance
pub struct OptimizedBlockRenderer {
    chunk_meshes: HashMap<ChunkCoord, SceneNode>,
    rendered_chunks: Vec<ChunkCoord>,
}

impl OptimizedBlockRenderer {
    pub fn new() -> Self {
        Self {
            chunk_meshes: HashMap::new(),
            rendered_chunks: Vec::new(),
        }
    }

    pub fn update_rendering(&mut self, world: &World, window: &mut Window, player_pos: Vector3<f32>) {
        let player_chunk = ChunkCoord::from_world_pos(player_pos.x, player_pos.z);
        let render_distance = 6;

        // Remove meshes for chunks that are too far away
        self.rendered_chunks.retain(|&chunk_coord| {
            let distance = (chunk_coord.x - player_chunk.x).abs().max((chunk_coord.z - player_chunk.z).abs());
            if distance > render_distance + 1 {
                if let Some(mut mesh) = self.chunk_meshes.remove(&chunk_coord) {
                    window.remove_node(&mut mesh);
                }
                false
            } else {
                true
            }
        });

        // Render chunks around player
        for x in -render_distance..=render_distance {
            for z in -render_distance..=render_distance {
                let chunk_coord = ChunkCoord::new(player_chunk.x + x, player_chunk.z + z);
                
                if !self.rendered_chunks.contains(&chunk_coord) {
                    if let Some(chunk) = world.chunks.get(&chunk_coord) {
                        self.render_chunk_optimized(chunk_coord, chunk, window);
                        self.rendered_chunks.push(chunk_coord);
                    }
                }
            }
        }
    }

    fn render_chunk_optimized(&mut self, chunk_coord: ChunkCoord, chunk: &crate::world::Chunk, window: &mut Window) {
        // For now, use simple cube rendering
        // In a full implementation, this would generate a single mesh for the entire chunk
        let chunk_world_x = chunk_coord.x * CHUNK_SIZE as i32;
        let chunk_world_z = chunk_coord.z * CHUNK_SIZE as i32;

        // Create a group node for the chunk
        let mut chunk_group = window.add_group();

        let mut block_count = 0;
        // Iterate through all blocks in the chunk (only non-air blocks are stored)
        for ((x, y, z), block) in &chunk.blocks {
            if *y < 80 && self.should_render_block_optimized(chunk, *x, *y, *z) { // Limit height for performance
                let world_x = chunk_world_x + *x as i32;
                let world_y = *y as i32;
                let world_z = chunk_world_z + *z as i32;

                let mut cube = chunk_group.add_cube(0.98, 0.98, 0.98); // Slightly smaller for gaps
                cube.set_local_translation(Translation3::new(
                    world_x as f32,
                    world_y as f32,
                    world_z as f32,
                ));
                cube.set_color(
                    block.block_type.color().x,
                    block.block_type.color().y,
                    block.block_type.color().z,
                );

                block_count += 1;
                if block_count > 1000 { // Limit blocks per chunk for performance
                    break;
                }
            }
        }

        self.chunk_meshes.insert(chunk_coord, chunk_group);
    }

    fn should_render_block_optimized(&self, chunk: &crate::world::Chunk, x: usize, y: usize, z: usize) -> bool {
        // Simplified check - only render surface blocks and some underground
        if y > 50 {
            return true; // Always render above ground
        }

        // For underground, only render if exposed
        let directions = [(0, 1, 0), (1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1)];

        for (dx, dy, dz) in directions.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            let nz = z as i32 + dz;

            if nx < 0 || nx >= CHUNK_SIZE as i32 ||
               ny < 0 || ny >= CHUNK_HEIGHT as i32 ||
               nz < 0 || nz >= CHUNK_SIZE as i32 {
                return true;
            }

            let neighbor_type = chunk.get_block_type(nx as usize, ny as usize, nz as usize);
            if neighbor_type == BlockType::Air {
                return true;
            }
        }

        false
    }

    pub fn cleanup(&mut self, window: &mut Window) {
        for (_, mut mesh) in self.chunk_meshes.drain() {
            window.remove_node(&mut mesh);
        }
        self.rendered_chunks.clear();
    }
}
