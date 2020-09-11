use vek::*;

pub fn quat_from_ypr(
    yaw: f32,
    pitch: f32,
    roll: f32,
) -> vek::Quaternion<f32> {
    vek::Quaternion::rotation_y(yaw) *
        vek::Quaternion::rotation_x(pitch) *
        vek::Quaternion::rotation_z(roll)
}

pub fn nznormalize(v: Vec3<f32>) -> Vec3<f32> {
    if let Some(v) = v.try_normalized() {
        return v;
    }
    v
}

pub fn max_dim(vec: Vec3<f32>) -> Vec3<f32> {
    if vec.x.abs() > vec.y.abs() && vec.x.abs() > vec.y.abs()
    {
        return Vec3::new(vec.x, 0.0, 0.0);
    } else if vec.y.abs() > vec.z.abs() &&
        vec.y.abs() > vec.x.abs()
    {
        return Vec3::new(0.0, vec.y, 0.0);
    } else {
        return Vec3::new(0.0, 0.0, vec.z);
    }
}
