use nalgebra;
use nalgebra::{Matrix4, Translation3, UnitQuaternion, Vector3};
use specs::{Component, VecStorage};
#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Transform {
    pub position: Translation3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn new(
        position: Translation3<f32>,
        rotation: UnitQuaternion<f32>,
        scale: Vector3<f32>,
    ) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
    pub fn transform_matrix(&self) -> Matrix4<f32> {
        let scale_matrix: Matrix4<f32> = Matrix4::new(
            self.scale.x,
            0.0,
            0.0,
            0.0,
            0.0,
            self.scale.y,
            0.0,
            0.0,
            0.0,
            0.0,
            self.scale.z,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        nalgebra::Isometry3::from_parts(self.position, self.rotation).to_homogeneous()
            * scale_matrix
    }
}
