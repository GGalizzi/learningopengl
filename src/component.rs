use vek::{Quaternion, Vec3};
use std::ops;

use crate::util::quat_from_ypr;

#[derive(Clone)]
pub struct Position {
    vector: Vec3<f32>,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Position {
        Position {
            vector: Vec3::new(x, y, z),
        }
    }

    pub fn from_vector(vector: Vec3<f32>) -> Position {
        Position { vector }
    }

    pub fn mul(&mut self, vec: Vec3<f32>) {
        self.vector = Vec3::new(
            vec.x * self.vector.x,
            vec.y * self.vector.y,
            vec.z * self.vector.z,
        );
    }

    pub fn internal(&self) -> Vec3<f32> {
        self.vector.clone()
    }

    pub fn move_towards(
        &mut self,
        direction: Vec3<f32>,
    ) -> Position {
        let mut vector = self.vector + direction;
        // *vector.y_mut() = 0.0;
        Position::from_vector(vector)
    }
}

impl ops::Mul<f32> for Position {
    type Output = Position;

    fn mul(self, rhs: f32) -> Self::Output {
        let vec = self.vector * rhs;
        Position::from_vector(vec)
    }
}

pub struct Rotation {
    pub quat: Quaternion<f32>,
    yaw: f32,
    pitch: f32,
}

impl Rotation {
    pub fn new() -> Rotation {
        Rotation {
            quat: quat_from_ypr(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    pub fn update_quat(&mut self, s: f32) {
        self.quat = Quaternion::slerp(
            self.quat,
            quat_from_ypr(
                self.yaw, self.pitch, 0.0,
            ).conjugate(),
            5.25 * s,
        );
    }

    pub fn rotate_on_y(&mut self, degrees: f32, s: f32) {
        self.yaw -= degrees.to_radians();
        self.update_quat(s);
    }

    pub fn rotate_on_x(&mut self, degrees: f32, s: f32) {
        self.pitch = (self.pitch - degrees.to_radians())
            .clamp(-1.1, 1.1);
        self.update_quat(s);
    }
}

pub struct BoundingBox {
    pub size: f32,
    pub height: f32,
}

impl BoundingBox {
    pub fn new(size: f32, height: f32) -> BoundingBox {
        BoundingBox { size, height }
    }
}

#[derive(Debug, Clone)]
pub struct Velocity {
    vec: Vec3<f32>,
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity {
            vec: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn from(vec: Vec3<f32>) -> Velocity {
        Velocity { vec }
    }

    pub fn internal(&self) -> Vec3<f32> {
        self.vec
    }

    pub fn apply_drag(&mut self, n: f32) {
        let mut vec = self.vec * n;
        if vec.magnitude() < 0.05 {
            vec = Vec3::zero();
        }
        self.vec = vec;
    }

    pub fn normalize(&mut self) {
        self.vec = self.vec.normalized();
    }
}

impl ops::Add<Vec3<f32>> for &Velocity {
    type Output = Velocity;

    fn add(self, rhs: Vec3<f32>) -> Self::Output {
        let mut nv = Velocity::new();
        nv.vec = self.vec + rhs;
        nv
    }
}
