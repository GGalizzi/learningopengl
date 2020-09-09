use vek::*;

pub fn quat_from_ypr(yaw: f32, pitch: f32, roll: f32) -> vek::Quaternion<f32> {
    vek::Quaternion::rotation_y(yaw) * vek::Quaternion::rotation_x(pitch) * vek::Quaternion::rotation_z(roll)
}
pub fn quat_from_ypr2(yaw: f32, pitch: f32, roll: f32) -> vek::Quaternion<f32> {
    let (y0, w0) = (yaw * 0.5).sin_cos();
    let (x1, w1) = (pitch * 0.5).sin_cos();
    let (z2, w2) = (roll * 0.5).sin_cos();

    let x3 = w0 * x1;
    let y3 = y0 * w1;
    let z3 = -y0 * x1;
    let w3 = w0 * w1;

    let x4 = x3 * w2 + y3 * z2;
    let y4 = -x3 * z2 + y3 * w2;
    let z4 = w3 * z2 + z3 * w2;
    let w4 = w3 * w2 - z3 * z2;

    vek::Quaternion::from_xyzw(

    x4, y4, z4, w4
    )
}

#[inline]
fn quat_to_axes(rotation: Quaternion<f32>) -> (Vec4<f32>, Vec4<f32>, Vec4<f32>) {
    let Quaternion{x, y, z, w} = rotation;
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;

    let x_axis = Vec4::new(1.0 - (yy + zz), xy + wz, xz - wy, 0.0);
    let y_axis = Vec4::new(xy - wz, 1.0 - (xx + zz), yz + wx, 0.0);
    let z_axis = Vec4::new(xz + wy, yz - wx, 1.0 - (xx + yy), 0.0);
    (x_axis, y_axis, z_axis)
}

#[inline]
pub fn from_rotation_translation(rotation: Quaternion<f32>, translation: Vec3<f32>) -> Mat4<f32> {
    let (x_axis, y_axis, z_axis) = quat_to_axes(rotation);
    /*Mat4::from_col_arrays([
        [x_axis.x, x_axis.y, x_axis.z, x_axis.w],
        [y_axis.x, y_axis.y, y_axis.z, y_axis.w],
        [z_axis.x, z_axis.y, z_axis.z, z_axis.w],
        [translation.x, translation.y, translation.x, 1.0]
    ])*/

    Mat4::from_col_arrays([
        [x_axis.x, y_axis.x, z_axis.x, translation.x],
        [x_axis.y, y_axis.y, z_axis.y, translation.y],
        [x_axis.z, y_axis.z, z_axis.z, translation.z],
        [x_axis.w, y_axis.w, z_axis.w, 1.0],
    ])
}