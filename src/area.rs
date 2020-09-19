use std::collections::HashMap;

use vek::Vec3;

use crate::{
    chunk::{Chunk, CHUNK_SIZE},
    map::{Area, Tile},
};

pub struct World {
    pub chunks: HashMap<Vec3<usize>, Chunk>,
}

impl World {
    pub fn from_area(area: Area) -> World {
        let mut chunks = HashMap::new();
        for y in 0..area.height {
            for z in 0..area.depth {
                for x in 0..area.width {
                    match area.tile_at(x, y, z) {
                        Tile::Wall => {
                            // Build a 10x10x10 wall
                            build(x, y, z, &mut chunks);
                        }
                        _ => {}
                    }
                }
            }
        }

        println!("generated {:?} chunks", chunks.len());

        for chunk in chunks.values_mut() {
            chunk.generate();
        }

        World { chunks }
    }
}

fn build(
    x: usize,
    y: usize,
    z: usize,
    chunks: &mut HashMap<Vec3<usize>, Chunk>,
) {
    let cz = CHUNK_SIZE as f32;
    let expansion = 10;
    for y in y * expansion..(y + 1) * expansion {
        for z in z * expansion..(z + 1) * expansion {
            for x in x * expansion..(x + 1) * expansion {
                let pos_in_chunk = Vec3::new(
                    x % CHUNK_SIZE,
                    y % CHUNK_SIZE,
                    z % CHUNK_SIZE,
                );
                let x = x as f32;
                let y = y as f32;
                let z = z as f32;
                let chunk_pos: Vec3<usize> = Vec3::new(
                    (x / cz).floor(),
                    (y / cz).floor(),
                    (z / cz).floor(),
                )
                .as_();
                let chunk = chunks
                    .entry(chunk_pos)
                    .or_insert(Chunk::empty());
                chunk.set_ground(pos_in_chunk);
            }
        }
    }
}
