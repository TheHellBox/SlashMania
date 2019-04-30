use nalgebra::{Matrix4, Perspective3, Quaternion, Translation3, UnitQuaternion};

pub struct Camera {
    pub position: Translation3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub perspective: Matrix4<f32>,
    pub aspect_ratio: f32,
    pub fov: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32) -> Camera {
        let ratio = x / y;
        let fov = 3.14 / 3.0;
        let proj = Perspective3::new(ratio, fov, 1.0, 10000.0).to_homogeneous();
        return Camera {
            position: Translation3::new(0.0, -3.0, 0.0),
            rotation: UnitQuaternion::from_quaternion(Quaternion::new(1.0, 0.0, 0.0, 0.0)),
            perspective: proj,
            aspect_ratio: ratio,
            fov: fov,
        };
    }
    pub fn view(&self) -> Matrix4<f32> {
        return (self.rotation * self.position).to_homogeneous();
    }
}
