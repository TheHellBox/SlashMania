use nalgebra::{Isometry3, Matrix4, Translation3, UnitQuaternion, Vector3};
use openxr as xr;

fn projection_opengl(left: f32, right: f32, up: f32, down: f32, znear: f32) -> Matrix4<f32> {
    let tan_angle_width = right - left;
    let tan_angle_height = up - down;

    let half_width = 2.0 / tan_angle_width;
    let half_height = 2.0 / tan_angle_height;

    Matrix4::new(
        half_width,
        0.0,
        (right + left) / tan_angle_width,
        0.0,
        0.0,
        half_height,
        (up + down) / tan_angle_height,
        0.0,
        0.0,
        0.0,
        -1.0,
        -(znear * 2.0),
        0.0,
        0.0,
        -1.0,
        0.0,
    )
}

pub fn projection_opengl_fov(fov: xr::Fovf, znear: f32) -> Matrix4<f32> {
    let tan_left = fov.angle_left.tan();
    let tan_right = fov.angle_right.tan();
    let tan_up = fov.angle_up.tan();
    let tan_down = fov.angle_down.tan();
    projection_opengl(tan_left, tan_right, tan_up, tan_down, znear)
}

pub fn view(position: xr::Vector3f, orientation: xr::Quaternionf) -> Matrix4<f32> {
    let position: Vector3<f32> = position.into();
    let position = Translation3::from(position);

    let orientation: UnitQuaternion<f32> = orientation.into();

    Isometry3::from_parts(position, UnitQuaternion::from_euler_angles(0.0, 3.14, 0.0) * orientation)
        .inverse()
        .to_homogeneous()
}
