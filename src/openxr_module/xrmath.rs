use nalgebra::{Matrix4, Projective3, Translation3, UnitQuaternion, Vector3};
use openxr as xr;

fn projection_opengl(left: f32, right: f32, up: f32, down: f32, znear: f32) -> Projective3<f32> {
    let tan_angle_width = right - left;
    let tan_angle_height = up - down;

    let half_width = 2.0 / tan_angle_width;
    let half_height = 2.0 / tan_angle_height;

    Projective3::from_matrix_unchecked(Matrix4::new(
        half_width,
        0.0,
        0.0,
        0.0,
        0.0,
        half_height,
        0.0,
        0.0,
        (right + left) / tan_angle_width,
        (up + down) / tan_angle_height,
        -1.0,
        -1.0,
        0.0,
        0.0,
        -(znear * 2.0),
        0.0,
    ))
}

pub fn projection_opengl_fov(fov: xr::Fovf, znear: f32) -> Projective3<f32> {
    let tan_left = fov.angle_left.tan();
    let tan_right = fov.angle_right.tan();
    let tan_up = fov.angle_up.tan();
    let tan_down = fov.angle_down.tan();
    projection_opengl(tan_left, tan_right, tan_up, tan_down, znear)
}

pub fn view(position: xr::Vector3f, orientation: xr::Quaternionf) -> Matrix4<f32> {
    let scale_matrix: Matrix4<f32> = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let position: Vector3<f32> = position.into();
    let position = Translation3::from(position);

    let orientation: UnitQuaternion<f32> = orientation.into();

    let translation_matrix = position.to_homogeneous();
    let orientation_matrix = orientation.to_homogeneous();
    translation_matrix * (scale_matrix * orientation_matrix)
}
