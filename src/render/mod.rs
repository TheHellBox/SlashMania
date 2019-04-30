#![allow(unused)]
use crate::SCALE;
use crate::parser::*;
use crate::openxr_module::OpenXR;

use std::collections::HashMap;
use nalgebra::{Translation3, Matrix4, UnitQuaternion};
use glium::{Display, Program, Surface, Frame, vertex::VertexBufferAny, Texture2d, DrawParameters};

use std::rc::Rc;
use std::os::raw::*;
use x11::{xlib, glx};
use std::ffi::{c_void, CString};

pub mod camera;
pub mod shaders;
pub mod backend;

pub struct Window{
    pub context: Rc<glium::backend::Context>,
    pub xr: OpenXR,
    pub shaders: HashMap<String, Program>,
    pub models: HashMap<String, VertexBufferAny>,
    pub textures: HashMap<String, Texture2d>,
    pub camera: camera::Camera,
}

impl Window{
    // I have no experience working with X11, so expect monkey code
    pub fn new() -> Self{
        let backend = backend::Backend::new();
        let xr = OpenXR::new(&backend);
        let context = unsafe {
            glium::backend::Context::new(backend, false, Default::default())
        }.unwrap();
        Self{
            context,
            xr,
            shaders: HashMap::new(),
            models: HashMap::new(),
            textures: HashMap::new(),
            camera: camera::Camera::new(800 as f32, 600 as f32)
        }
    }
    pub fn draw(&mut self){
        use glium::texture::{DepthTexture2d, DepthFormat, UncompressedFloatFormat, MipmapsOption};

        let swapchain_image = self.xr.get_swapchain_image();
        if let Some(swapchain_image) = swapchain_image{
            self.xr.frame_stream_begin();
            let texture_array = unsafe{
                glium::texture::texture2d_array::Texture2dArray::from_id(
                    &self.context,
                    glium::texture::UncompressedFloatFormat::U8U8U8U8,
                    swapchain_image,
                    false,
                    glium::texture::MipmapsOption::NoMipmap,
                    glium::texture::Dimensions::Texture2dArray{
                        width: self.xr.resolution.0,
                        height: self.xr.resolution.1,
                        array_size: 2,
                    }
                )
            };
            let texture_left = texture_array.layer(0).unwrap().mipmap(0).unwrap();
            let texture_right = texture_array.layer(1).unwrap().mipmap(0).unwrap();

            let depthtexture = DepthTexture2d::empty_with_format(&self.context, DepthFormat::F32, MipmapsOption::EmptyMipmaps, self.xr.resolution.0, self.xr.resolution.1).unwrap();
            let mut target = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&self.context, texture_left, &depthtexture).unwrap();
            target.clear_color_and_depth((0.6, 0.6, 0.6, 1.0), 1.0);

            let depthtexture = DepthTexture2d::empty_with_format(&self.context, DepthFormat::F32, MipmapsOption::EmptyMipmaps, self.xr.resolution.0, self.xr.resolution.1).unwrap();
            let mut target = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(&self.context, texture_right, &depthtexture).unwrap();
            target.clear_color_and_depth((0.6, 0.6, 0.6, 1.0), 1.0);

            self.xr.release_swapchain_image();
            self.xr.frame_stream_end();
        }
    }
    pub fn update_xr(&mut self){
        self.xr.update();
    }
    pub fn compile_shaders(&mut self){
        use shaders::*;
        println!("Compiling shaders...");
        let simple = glium::Program::from_source(&self.context, SHADER_SIMPLE_VERT, SHADER_SIMPLE_FRAG, None).unwrap();
        self.shaders.insert("simple".to_string(), simple);
    }
    pub fn load_default_models(&mut self){
        use crate::obj_loader::load_obj;
        self.models.insert("block".to_string(), load_obj("./assets/models/block.obj", &self.context));
        self.models.insert("cube".to_string(), load_obj("./assets/models/cube.obj", &self.context));
    }
    pub fn load_default_textures(&mut self){
        use crate::textures::load_texture;
        self.textures.insert("dev".to_string(), load_texture("./assets/textures/dev.png".to_string(), &self.context));
        self.textures.insert("mine".to_string(), load_texture("./assets/textures/mine.png".to_string(), &self.context));
        self.textures.insert("note_red".to_string(), load_texture("./assets/textures/note_red.png".to_string(), &self.context));
        self.textures.insert("obstacle".to_string(), load_texture("./assets/textures/obstacle.png".to_string(), &self.context));
        self.textures.insert("note_blue".to_string(), load_texture("./assets/textures/note_blue.png".to_string(), &self.context));
        self.textures.insert("note_middle_red".to_string(), load_texture("./assets/textures/note_middle_red.png".to_string(), &self.context));
        self.textures.insert("note_middle_blue".to_string(), load_texture("./assets/textures/note_middle_blue.png".to_string(), &self.context));
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, normal, tex_coords);


pub fn calc_transform(scale: (f32, f32, f32), position: Translation3<f32>, rotation: UnitQuaternion<f32>) -> Matrix4<f32>{
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
pub fn get_params() -> DrawParameters<'static>{
    use glium::{Depth, draw_parameters, DepthTest};
    DrawParameters {
        depth: Depth {
            test: DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: draw_parameters::BackfaceCullingMode::CullClockwise,
        blend: draw_parameters::Blend::alpha_blending(),
        .. Default::default()
    }
}
