use vek::Vec3;

use crate::mesh::Mesh;

const CHUNK_SIZE: usize = 32;
const VOXEL_SIZE: f32 = 0.1;

enum Voxel {
    Air,
    Ground,
}

enum QuadSide {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

pub struct Chunk {
    voxels: Vec<Voxel>,
    mesh: Option<Mesh>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            voxels: Vec::with_capacity(
                CHUNK_SIZE * CHUNK_SIZE,
            ),
            mesh: None,
        }
    }

    pub fn generate(&mut self) {
        // for xyz
        let verts = gen_voxel(&Vec3::new(0.0, 0.0, 0.0));

        self.mesh =
            Some(Mesh::build().verts(&verts).finalize());
    }
    
    pub fn get_mesh(&self) -> &Mesh {
        self.mesh.as_ref().expect("Mesh hasn't been generated for this chunk")
    }
}

fn gen_voxel(pos: &Vec3<f32>) -> Vec<f32> {
    let mut verts = Vec::new();
    verts.append(&mut gen_quad(QuadSide::Top, pos));
    verts.append(&mut gen_quad(QuadSide::Bottom, pos));
    verts.append(&mut gen_quad(QuadSide::Left, pos));
    verts.append(&mut gen_quad(QuadSide::Right, pos));
    verts.append(&mut gen_quad(QuadSide::Front, pos));
    verts.append(&mut gen_quad(QuadSide::Back, pos));
    verts
}

#[rustfmt::skip]
fn gen_quad(
    side: QuadSide,
    &Vec3 { x, y, z }: &Vec3<f32>,
) -> Vec<f32> {
    use QuadSide::*;
     match side {
        Top => {
            vec![
                x, y+VOXEL_SIZE, z,
                x+VOXEL_SIZE, y+VOXEL_SIZE, z,
                x+VOXEL_SIZE, y+VOXEL_SIZE, z+VOXEL_SIZE,
                x+VOXEL_SIZE, y+VOXEL_SIZE, z+VOXEL_SIZE,
                x, y+VOXEL_SIZE, z+VOXEL_SIZE,
                x, y+VOXEL_SIZE, z,
            ]
        }
        Bottom => {
            vec![
                x, y, z,
                x + VOXEL_SIZE, y, z + VOXEL_SIZE,
                x+VOXEL_SIZE, y, z,
                x + VOXEL_SIZE, y, z + VOXEL_SIZE,
                x, y, z,
                x, y, z + VOXEL_SIZE,
            ]
        }
        Left => {
            vec![
                x, y+VOXEL_SIZE, z+VOXEL_SIZE,
                x, y, z,
                x, y+VOXEL_SIZE, z,
                x, y, z,
                x, y+VOXEL_SIZE, z+VOXEL_SIZE,
                x, y, z+VOXEL_SIZE,
            ]
        }
        Right => {
            vec![
                x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z,
                x + VOXEL_SIZE, y, z,
                x + VOXEL_SIZE, y, z,
                x + VOXEL_SIZE, y, z + VOXEL_SIZE,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE,
            ]
        }
        Front => {
            vec![
                x, y, z + VOXEL_SIZE,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE,
                x + VOXEL_SIZE, y, z + VOXEL_SIZE,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE,
                x, y, z + VOXEL_SIZE,
                x, y + VOXEL_SIZE, z + VOXEL_SIZE,
            ]
        }
        Back => {
            vec![
                x, y, z,
                x + VOXEL_SIZE, y, z,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z,
                x, y + VOXEL_SIZE, z,
                x, y, z,
            ]
        }
    }
}
