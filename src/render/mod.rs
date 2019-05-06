use crate::openxr_module::OpenXR;

use glium::texture::{DepthFormat, DepthTexture2d, MipmapsOption};
use glium::{vertex::VertexBuffer, Program, Texture2d};
use std::collections::HashMap;

use renderdoc::prelude::*;
use renderdoc::{OverlayBits, RenderDoc, V110};

use std::rc::Rc;

pub mod backend;
mod draw;
mod shaders;

pub struct RenderDocWrapper{
    renderdoc: RenderDoc<V110>,
    device: *mut std::ffi::c_void
}

pub struct Window {
    context: Rc<glium::backend::Context>,
    xr: OpenXR,
    shaders: HashMap<String, Program>,
    models: HashMap<String, VertexBuffer<Vertex>>,
    textures: HashMap<String, Texture2d>,
    depth_textures: Option<(DepthTexture2d, DepthTexture2d)>,
    renderdoc: Option<RenderDocWrapper>
}

impl Window {
    pub fn new() -> Self {
        let mut backend = backend::Backend::new();

        #[cfg(feature = "rd")]
        let raw_context = backend.context;

        let xr = OpenXR::new(&mut backend);
        let context =
            unsafe { glium::backend::Context::new(backend, false, Default::default()) }.unwrap();

        let renderdoc = {
            #[cfg(feature = "rd")]
            {
                let mut rd = RenderDoc::new().unwrap();
                rd.set_active_window(raw_context as *mut _, std::ptr::null());
                rd.mask_overlay_bits(OverlayBits::DEFAULT, OverlayBits::DEFAULT);
                let mut rdw = RenderDocWrapper{
                    renderdoc: rd,
                    device: raw_context as *mut _
                };
                Some(rdw)
            }
            #[cfg(not(feature = "rd"))]
            None
        };
        Self {
            context,
            xr,
            depth_textures: None,
            shaders: HashMap::new(),
            models: HashMap::new(),
            textures: HashMap::new(),
            renderdoc: renderdoc
        }
    }
    pub fn create_depth_textures(&mut self) {
        let depth_texture_left = DepthTexture2d::empty_with_format(
            &self.context,
            DepthFormat::F32,
            MipmapsOption::EmptyMipmaps,
            self.xr.swapchains.resolution_left.0,
            self.xr.swapchains.resolution_left.1,
        )
        .unwrap();
        let depth_texture_right = DepthTexture2d::empty_with_format(
            &self.context,
            DepthFormat::F32,
            MipmapsOption::EmptyMipmaps,
            self.xr.swapchains.resolution_right.0,
            self.xr.swapchains.resolution_right.1,
        )
        .unwrap();
        self.depth_textures = Some((depth_texture_left, depth_texture_right));
    }
    pub fn draw(&mut self) {
        let swapchain_image = self.xr.swapchains.get_images();
        if let Some((swapchain_image_left, swapchain_image_right)) = swapchain_image {
            if self.depth_textures.is_none() {
                self.create_depth_textures();
            }
            let depth_textures = self.depth_textures.as_ref().unwrap();

            self.xr.frame_stream_begin();
            let texture_left = unsafe {
                glium::texture::texture2d::Texture2d::from_id(
                    &self.context,
                    glium::texture::UncompressedFloatFormat::U8U8U8U8,
                    swapchain_image_left,
                    false,
                    glium::texture::MipmapsOption::NoMipmap,
                    glium::texture::Dimensions::Texture2d {
                        width: self.xr.swapchains.resolution_left.0,
                        height: self.xr.swapchains.resolution_left.1,
                    },
                )
            };
            let texture_right = unsafe {
                glium::texture::texture2d::Texture2d::from_id(
                    &self.context,
                    glium::texture::UncompressedFloatFormat::U8U8U8U8,
                    swapchain_image_right,
                    false,
                    glium::texture::MipmapsOption::NoMipmap,
                    glium::texture::Dimensions::Texture2d {
                        width: self.xr.swapchains.resolution_right.0,
                        height: self.xr.swapchains.resolution_right.1,
                    },
                )
            };

            if let Some(renderdoc_wrapper) = &mut self.renderdoc{
                renderdoc_wrapper.renderdoc.start_frame_capture(renderdoc_wrapper.device, std::ptr::null());
            }

            let left_eye_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                &self.context,
                &texture_left,
                &depth_textures.0,
            )
            .unwrap();
            self.draw_image(left_eye_buffer, false);

            if let Some(renderdoc_wrapper) = &mut self.renderdoc{
                renderdoc_wrapper.renderdoc.end_frame_capture(renderdoc_wrapper.device, std::ptr::null());
                std::thread::sleep(std::time::Duration::from_millis(10000));
            }

            let right_eye_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                &self.context,
                &texture_right,
                &depth_textures.1,
            )
            .unwrap();
            self.draw_image(right_eye_buffer, true);

            self.xr.swapchains.release_images();
            self.xr.frame_stream_end();
        }
    }
    pub fn update_xr(&mut self) {
        self.xr.update();
    }
    pub fn compile_shaders(&mut self) {
        use shaders::*;
        println!("Compiling shaders...");
        let simple = glium::Program::from_source(
            &self.context,
            SHADER_SIMPLE_VERT,
            SHADER_SIMPLE_FRAG,
            None,
        )
        .unwrap();
        self.shaders.insert("simple".to_string(), simple);
    }
    pub fn load_default_models(&mut self) {
        use crate::obj_loader::load_obj;
        self.models.insert(
            "block".to_string(),
            load_obj("./assets/models/block.obj", &self.context),
        );
        self.models.insert(
            "cube".to_string(),
            load_obj("./assets/models/cube.obj", &self.context),
        );
        self.models.insert(
            "test_scene".to_string(),
            load_obj("./assets/models/test_scene.obj", &self.context),
        );
    }
    pub fn load_default_textures(&mut self) {
        use crate::textures::load_texture;
        self.textures.insert(
            "dev".to_string(),
            load_texture("./assets/textures/dev.png".to_string(), &self.context),
        );
        self.textures.insert(
            "mine".to_string(),
            load_texture("./assets/textures/mine.png".to_string(), &self.context),
        );
        self.textures.insert(
            "note_red".to_string(),
            load_texture("./assets/textures/note_red.png".to_string(), &self.context),
        );
        self.textures.insert(
            "obstacle".to_string(),
            load_texture("./assets/textures/obstacle.png".to_string(), &self.context),
        );
        self.textures.insert(
            "note_blue".to_string(),
            load_texture("./assets/textures/note_blue.png".to_string(), &self.context),
        );
        self.textures.insert(
            "note_middle_red".to_string(),
            load_texture(
                "./assets/textures/note_middle_red.png".to_string(),
                &self.context,
            ),
        );
        self.textures.insert(
            "note_middle_blue".to_string(),
            load_texture(
                "./assets/textures/note_middle_blue.png".to_string(),
                &self.context,
            ),
        );
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, normal, tex_coords);
