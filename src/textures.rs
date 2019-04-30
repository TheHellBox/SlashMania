use glium::backend::Facade;
use glium::texture::Texture2d;
use image;

pub fn load_texture<F: Facade + ?Sized>(path: String, disp: &F) -> Texture2d {
    use glium::texture::RawImage2d;
    use std::path::Path;
    if let Ok(img) = image::open(Path::new(&path)) {
        let img = img.to_rgba();
        let dis = img.dimensions();
        let glium_raw_tex = RawImage2d::from_raw_rgba_reversed(&img.into_raw(), dis);
        let tex = Texture2d::new(disp, glium_raw_tex).unwrap();
        tex
    } else {
        // Can cause stack overflow
        println!("Texture {} not found, using dev texture", path);
        load_texture("./assets/textures/dev.png".to_string(), disp)
    }
}
