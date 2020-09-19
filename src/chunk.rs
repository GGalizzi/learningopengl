use rand::{self, Rng};

use vek::Vec3;

use crate::mesh::Mesh;

pub const CHUNK_SIZE: usize = 64;
pub const VOXEL_SIZE: f32 = 0.1;

struct VertNormal(Vec<f32>, Vec<f32>);

impl VertNormal {
    pub fn new(v: Vec<f32>, n: Vec<f32>) -> VertNormal {
        VertNormal(v, n)
    }

    pub fn append(&mut self, mut vn: VertNormal) {
        self.0.append(&mut vn.0);
        self.1.append(&mut vn.1);
    }
}

#[derive(Copy, Clone)]
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
        let mut voxels = Vec::with_capacity(
            CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE,
        );

        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let mut rng = rand::thread_rng();
                    let n: u32 = rng.gen_range(0, 100);
                    if n < 99 {
                        voxels.push(Voxel::Air);
                        continue;
                    }
                    voxels.push(Voxel::Ground);
                }
            }
        }

        Chunk { mesh: None, voxels }
    }

    pub fn empty() -> Chunk {
        let mut voxels = Vec::with_capacity(
            CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE,
        );

        voxels.resize(
            CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE,
            Voxel::Air,
        );

        Chunk { mesh: None, voxels }
    }

    pub fn set_ground(
        &mut self,
        Vec3 { x, y, z }: Vec3<usize>,
    ) {
        self.voxels[(z as usize * CHUNK_SIZE + x as usize) +
            y as usize * CHUNK_SIZE * CHUNK_SIZE] =
            Voxel::Ground;
    }

    pub fn generate(&mut self) {
        let mut vert_normal = VertNormal::new(
            Vec::with_capacity(CHUNK_SIZE.pow(3)),
            Vec::with_capacity(CHUNK_SIZE.pow(3)),
        );
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let x = x as f32;
                    let y = y as f32;
                    let z = z as f32;
                    let v = Vec3::new(x, y, z);
                    if self.is_pos_free(v) {
                        continue;
                    }
                    let vn = self.gen_voxel(&v);
                    vert_normal.append(vn);
                }
            }
        }

        self.mesh = Some(
            Mesh::build()
                .verts(&vert_normal.0)
                .normals(&vert_normal.1)
                .finalize(),
        );
    }

    fn gen_voxel(&self, pos: &Vec3<f32>) -> VertNormal {
        let mut vert_normal =
            VertNormal::new(Vec::new(), Vec::new());

        if self.is_pos_free(pos + Vec3::up()) {
            vert_normal.append(gen_quad(
                QuadSide::Top,
                pos * VOXEL_SIZE,
            ));
        }
        if self.is_pos_free(pos + Vec3::down()) {
            vert_normal.append(gen_quad(
                QuadSide::Bottom,
                pos * VOXEL_SIZE,
            ));
        }
        if self.is_pos_free(pos + Vec3::left()) {
            vert_normal.append(gen_quad(
                QuadSide::Left,
                pos * VOXEL_SIZE,
            ));
        }
        if self.is_pos_free(pos + Vec3::right()) {
            vert_normal.append(gen_quad(
                QuadSide::Right,
                pos * VOXEL_SIZE,
            ));
        }
        if self.is_pos_free(pos + Vec3::back_rh()) {
            vert_normal.append(gen_quad(
                QuadSide::Front,
                pos * VOXEL_SIZE,
            ));
        }
        if self.is_pos_free(pos + Vec3::forward_rh()) {
            vert_normal.append(gen_quad(
                QuadSide::Back,
                pos * VOXEL_SIZE,
            ));
        }
        vert_normal
    }

    fn is_pos_free(
        &self,
        Vec3 { x, y, z }: Vec3<f32>,
    ) -> bool {
        if x < 0.0 ||
            y < 0.0 ||
            z < 0.0 ||
            x >= CHUNK_SIZE as f32 ||
            y >= CHUNK_SIZE as f32 ||
            z >= CHUNK_SIZE as f32
        {
            return true;
        }
        if let Some(Voxel::Ground) = self.voxels.get(
            (z as usize * CHUNK_SIZE + x as usize) +
                y as usize * CHUNK_SIZE * CHUNK_SIZE,
        ) {
            return false;
        }
        true
    }

    pub fn get_mesh(&self) -> &Mesh {
        self.mesh.as_ref().expect(
            "Mesh hasn't been generated for this chunk",
        )
    }
}

#[rustfmt::skip]
fn gen_quad(
    side: QuadSide,
    Vec3 { x, y, z }: Vec3<f32>,
) -> VertNormal {
    use QuadSide::*;
     match side {
        Top => {
            VertNormal::new(vec![
                x, y+VOXEL_SIZE, z,
                x+VOXEL_SIZE, y+VOXEL_SIZE, z,
                x+VOXEL_SIZE, y+VOXEL_SIZE, z+VOXEL_SIZE,
                x+VOXEL_SIZE, y+VOXEL_SIZE, z+VOXEL_SIZE,
                x, y+VOXEL_SIZE, z+VOXEL_SIZE,
                x, y+VOXEL_SIZE, z,
            ], vec![
                0.0, 1.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 1.0, 0.0,
            ])
        }
        Bottom => {
            VertNormal::new(vec![
                x, y, z,
                x + VOXEL_SIZE, y, z + VOXEL_SIZE,
                x+VOXEL_SIZE, y, z,
                x + VOXEL_SIZE, y, z + VOXEL_SIZE,
                x, y, z,
                x, y, z + VOXEL_SIZE,
            ], vec![
                0.0, -1.0, 0.0,
                0.0, -1.0, 0.0,
                0.0, -1.0, 0.0,
                0.0, -1.0, 0.0,
                0.0, -1.0, 0.0,
                0.0, -1.0, 0.0,
            ])
        }
        Left => {
            VertNormal::new(vec![
                x, y+VOXEL_SIZE, z+VOXEL_SIZE,
                x, y, z,
                x, y+VOXEL_SIZE, z,
                x, y, z,
                x, y+VOXEL_SIZE, z+VOXEL_SIZE,
                x, y, z+VOXEL_SIZE,
            ], vec![
                -1.0, 0.0, 0.0,
                -1.0, 0.0, 0.0,
                -1.0, 0.0, 0.0,
                -1.0, 0.0, 0.0,
                -1.0, 0.0, 0.0,
                -1.0, 0.0, 0.0,
            ])
        }
        Right => {
            VertNormal::new(vec![
                x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z,
                x + VOXEL_SIZE, y, z,
                x + VOXEL_SIZE, y, z,
                x + VOXEL_SIZE, y, z + VOXEL_SIZE,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE,
            ], vec![
                1.0, 0.0, 0.0,
                1.0, 0.0, 0.0,
                1.0, 0.0, 0.0,
                1.0, 0.0, 0.0,
                1.0, 0.0, 0.0,
                1.0, 0.0, 0.0,
            ])
        }
        Front => {
            VertNormal::new(vec![
                x, y, z + VOXEL_SIZE,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE,
                x + VOXEL_SIZE, y, z + VOXEL_SIZE,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z + VOXEL_SIZE,
                x, y, z + VOXEL_SIZE,
                x, y + VOXEL_SIZE, z + VOXEL_SIZE,
            ], vec![
                0.0, 0.0, -1.0,
                0.0, 0.0, -1.0,
                0.0, 0.0, -1.0,
                0.0, 0.0, -1.0,
                0.0, 0.0, -1.0,
                0.0, 0.0, -1.0,
            ])
        }
        Back => {
            VertNormal::new(vec![
                x, y, z,
                x + VOXEL_SIZE, y, z,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z,
                x + VOXEL_SIZE, y + VOXEL_SIZE, z,
                x, y + VOXEL_SIZE, z,
                x, y, z,
            ], vec![
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
            ])
        }
    }
}
