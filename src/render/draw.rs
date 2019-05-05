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
            (0.5, 0.1, 0.1),
            Translation3::new(-3.0, 0.0, 0.0),
            UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(1.0, 0.0, 0.0, 0.0)))
            .into();

        target.draw(
            &self.models["test_scene"],
            &NoIndices(PrimitiveType::TrianglesList),
            &self.shaders["simple"],
            &uniform! { transform: transform, projection: projection, view: view, tex: &self.textures["note_red"]},
            &get_params()
        ).unwrap();
    }
}

pub fn get_params() -> DrawParameters<'static> {
    use glium::{draw_parameters, Depth, DepthTest};
    DrawParameters {
        depth: Depth {
            test: DepthTest::IfMore,
            write: true,
            ..Default::default()
        },
        backface_culling: draw_parameters::BackfaceCullingMode::CullClockwise,
        blend: draw_parameters::Blend::alpha_blending(),
        ..Default::default()
    }
}

pub fn calc_transform(
    scale: (f32, f32, f32),
    position: Translation3<f32>,
    rotation: UnitQuaternion<f32>,
) -> Matrix4<f32> {
    let scale_matrix: Matrix4<f32> = Matrix4::new(
        scale.0, 0.0, 0.0, 0.0,
        0.0, scale.1, 0.0, 0.0,
        0.0, 0.0, scale.2, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    let translation_matrix = position.to_homogeneous();
    let rotation_matrix = rotation.to_homogeneous();
    translation_matrix * rotation_matrix * scale_matrix
}
