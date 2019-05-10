use crate::components::*;
use crate::openxr_module::xrmath;
use crate::render::Window;

use glium::index::{NoIndices, PrimitiveType};
use glium::{DrawParameters, Surface};
use specs::Join;

struct OrientationInfo {
    projection: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
}

struct DrawObjectInfo {
    model: String,
    texture: String,
    shader: String,
    transform: [[f32; 4]; 4],
}

impl Window {
    fn start_frame_draw(
        &self,
        target: &mut glium::framebuffer::SimpleFrameBuffer,
        right_eye: bool,
    ) -> OrientationInfo {
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

        target.clear_color_and_depth((0.2, 0.2, 0.2, 1.0), 1.0);

        let projection: [[f32; 4]; 4] = xrmath::projection_opengl_fov(fov, 0.1).into();
        let view: [[f32; 4]; 4] = xrmath::view(position, orientation).into();

        let frame_draw_info = OrientationInfo { projection, view };

        frame_draw_info
    }
    fn draw_object(
        &self,
        orientation: &OrientationInfo,
        object: DrawObjectInfo,
        target: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let model = self.models.get(&object.model);
        let texture = self.textures.get(&object.texture);
        let shader = self.shaders.get(&object.shader);

        if let (Some(model), Some(texture), Some(shader)) = (model, texture, shader) {
            target.draw(
                model,
                &NoIndices(PrimitiveType::TrianglesList),
                shader,
                &uniform! { transform: object.transform, projection: orientation.projection, view: orientation.view, tex: texture},
                &get_params()
            ).unwrap();
        };
    }
}

impl<'a> specs::System<'a> for Window {
    type SystemData = (
        specs::ReadStorage<'a, transform::Transform>,
        specs::ReadStorage<'a, drawable::Drawable>,
    );

    fn run(&mut self, (transforms, drawables): Self::SystemData) {
        let texture_array = self.get_texture_array();
        self.update_xr();
        if let Some(texture_array) = texture_array {
            let mut window_frame = glium::Frame::new(self.context.clone(), (800, 800));
            let depth_textures = self.depth_textures.as_ref().unwrap();
            let texture_left = texture_array.layer(0).unwrap().mipmap(0).unwrap();
            let texture_right = texture_array.layer(1).unwrap().mipmap(0).unwrap();

            let mut left_eye_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                &self.context,
                texture_left,
                &depth_textures.0,
            )
            .unwrap();
            let mut right_eye_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                &self.context,
                texture_right,
                &depth_textures.1,
            )
            .unwrap();

            let orientation_left = self.start_frame_draw(&mut left_eye_buffer, false);
            let orientation_right = self.start_frame_draw(&mut right_eye_buffer, true);

            let mut buffers = [
                (left_eye_buffer, orientation_left),
                (right_eye_buffer, orientation_right),
            ];
            for buffer in &mut buffers {
                for (transform, drawable) in (&transforms, &drawables).join() {
                    if drawable.enabled {
                        let transform_matrix = transform.transform_matrix().into();
                        let draw_object = DrawObjectInfo {
                            model: drawable.model.clone(),
                            texture: drawable.texture.clone(),
                            shader: drawable.shader.clone(),
                            transform: transform_matrix,
                        };
                        self.draw_object(&buffer.1, draw_object, &mut buffer.0);
                    }
                }
            }
            self.finish_draw();

            window_frame
                .draw(
                    &self.models["box_2d"],
                    &NoIndices(PrimitiveType::TrianglesList),
                    &self.shaders["simple2d"],
                    &uniform! {tex: &texture_array},
                    &Default::default(),
                )
                .unwrap();
            window_frame.finish().unwrap();
        }
    }
}

pub fn get_params() -> DrawParameters<'static> {
    use glium::{draw_parameters, Depth, DepthTest};
    DrawParameters {
        depth: Depth {
            test: DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    }
}
