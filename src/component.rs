use glam::{Quat, Vec3};
use std::ops;
pub struct Position {
    vector: Vec3,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Position {
        Position {
            vector: Vec3::new(x, y, z),
        }
    }

    pub fn from_vector(vector: Vec3) -> Position {
        Position { vector }
    }

    pub fn internal(&self) -> Vec3 {
        self.vector.clone()
    }

    pub fn move_towards(
        &mut self,
        direction: Vec3,
    ) -> Position {
        let mut vector = self.vector + direction;
        *vector.y_mut() = 0.0;
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
    pub quat: Quat,
    yaw: f32,
    pitch: f32,
}

impl Rotation {
    pub fn new() -> Rotation {
        Rotation {
            quat: Quat::from_rotation_ypr(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    pub fn update_quat(&mut self, s: f32) {
        self.quat = self.quat.slerp(
            Quat::from_rotation_ypr(
                self.yaw, self.pitch, 0.0,
            )
            .conjugate(),
            5.25 * s,
        );
    }

    pub fn rotate_on_y(&mut self, degrees: f32, s: f32) {
        self.yaw += -degrees.to_radians();
        self.update_quat(s);
    }

    pub fn rotate_on_x(&mut self, degrees: f32, s: f32) {
        self.pitch += -degrees.to_radians();
        self.update_quat(s);
    }
}
