use crate::openxr_module::xrmath;
use crate::render::Window;

use glium::index::{NoIndices, PrimitiveType};
use glium::{DrawParameters, Surface};
use nalgebra::{Matrix4, Translation3, UnitQuaternion};

impl Window {
    pub fn draw_image(&self, mut target: glium::framebuffer::SimpleFrameBuffer, right_eye: bool) {
        let fov = {
            if right_eye {
                self.xr.views[1].fov
            } else {
                self.xr.views[0].fov
            }
        };
        let pose = {
            if right_eye {
                self.xr.views[1].pose
            } else {
                self.xr.views[0].pose
            }
        };
        let position = pose.position;
        let orientation = pose.orientation;

        target.clear_color_and_depth((0.3, 0.3, 0.3, 1.0), 1.0);

        let projection: [[f32; 4]; 4] = xrmath::projection_opengl_fov(fov, 0.1)
            .into();
        let view: [[f32; 4]; 4] = xrmath::view(position, orientation).into();

        let transform: [[f32; 4]; 4] = calc_transform(
            Translation3::new(-2.0, 0.0, 0.0),
            UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(1.0, 0.0, 0.0, 0.0)))
            .into();
        let model = self.models.get("cube");
        let texture = self.textures.get("note_red");
        let shader = self.shaders.get("simple");

        if let (Some(model), Some(texture), Some(shader)) = (model, texture, shader){
            target.draw(
                model,
                &NoIndices(PrimitiveType::TrianglesList),
                shader,
                &uniform! { transform: transform, projection: projection, view: view, tex: texture},
                &get_params()
            ).unwrap();
        }
    }
}

pub fn get_params() -> DrawParameters<'static> {
    use glium::{draw_parameters, Depth, DepthTest};
    DrawParameters {
        ..Default::default()
    }
}

pub fn calc_transform(
    position: Translation3<f32>,
    rotation: UnitQuaternion<f32>,
) -> Matrix4<f32> {
    nalgebra::Isometry3::from_parts(position, rotation).inverse().to_homogeneous()
}
