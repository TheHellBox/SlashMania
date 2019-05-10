use crate::openxr_module::OpenXR;

use glium::texture::{DepthFormat, DepthTexture2d, MipmapsOption};
use glium::{vertex::VertexBufferAny, Program, Texture2d};
use std::collections::HashMap;

use std::rc::Rc;

pub mod backend;
mod draw;
mod shaders;

pub struct Window {
    context: Rc<glium::backend::Context>,
    xr: OpenXR,
    shaders: HashMap<String, Program>,
    models: HashMap<String, VertexBufferAny>,
    textures: HashMap<String, Texture2d>,
    depth_textures: Option<(DepthTexture2d, DepthTexture2d)>,
}

impl Window {
    pub fn new() -> Self {
        let mut backend = backend::Backend::new();

        #[cfg(feature = "rd")]
        let raw_context = backend.context;

        let xr = OpenXR::new(&mut backend);
        let context =
            unsafe { glium::backend::Context::new(backend, false, Default::default()) }.unwrap();

        Self {
            context,
            xr,
            depth_textures: None,
            shaders: HashMap::new(),
            models: HashMap::new(),
            textures: HashMap::new(),
        }
    }
    pub fn create_depth_textures(&mut self) {
        let depth_texture_left = DepthTexture2d::empty_with_format(
            &self.context,
            DepthFormat::F32,
            MipmapsOption::EmptyMipmaps,
            self.xr.swapchain.resolution.0,
            self.xr.swapchain.resolution.1,
        )
        .unwrap();
        let depth_texture_right = DepthTexture2d::empty_with_format(
            &self.context,
            DepthFormat::F32,
            MipmapsOption::EmptyMipmaps,
            self.xr.swapchain.resolution.0,
            self.xr.swapchain.resolution.1,
        )
        .unwrap();
        self.depth_textures = Some((depth_texture_left, depth_texture_right));
    }
    pub fn get_texture_array(&mut self) -> Option<glium::texture::srgb_texture2d_array::SrgbTexture2dArray> {
        let swapchain_image = self.xr.swapchain.get_images();
        if let Some(swapchain_image) = swapchain_image {
            if self.depth_textures.is_none() {
                self.create_depth_textures();
            }

            self.xr.frame_stream_begin();

            let texture_array = unsafe {
                glium::texture::srgb_texture2d_array::SrgbTexture2dArray::from_id(
                    &self.context,
                    glium::texture::SrgbFormat::U8U8U8U8,
                    swapchain_image,
                    false,
                    glium::texture::MipmapsOption::NoMipmap,
                    glium::texture::Dimensions::Texture2dArray {
                        width: self.xr.swapchain.resolution.0,
                        height: self.xr.swapchain.resolution.1,
                        array_size: 2,
                    },
                )
            };
            Some(texture_array)
        } else {
            None
        }
    }
    pub fn finish_draw(&mut self) {
        self.context.finish();
        self.xr.swapchain.release_images();
        self.xr.frame_stream_end();
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
        let simple2d = glium::Program::from_source(
            &self.context,
            SHADER2D_SIMPLE_VERT,
            SHADER2D_SIMPLE_FRAG,
            None,
        )
        .unwrap();
        self.shaders.insert("simple".to_string(), simple);
        self.shaders.insert("simple2d".to_string(), simple2d);
    }
    pub fn load_default_models(&mut self) {
        use crate::obj_loader::{box_vertex_buf, load_obj};
        self.models.insert(
            "block".to_string(),
            load_obj("./assets/models/block.obj", &self.context),
        );
        self.models.insert(
            "cube".to_string(),
            load_obj("./assets/models/cube.obj", &self.context),
        );
        self.models
            .insert("box_2d".to_string(), box_vertex_buf(&self.context));
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

#[derive(Copy, Clone)]
pub struct Vertex2D {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}
implement_vertex!(Vertex2D, position, tex_coords);
