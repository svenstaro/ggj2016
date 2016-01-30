use graphics::context::Context;
use opengl_graphics::*;
use graphics::{ Image, clear, default_draw_state };
use graphics::rectangle::square;
use rand::{Rng, SeedableRng, XorShiftRng};
use piston_window::{ PistonWindow, WindowSettings };
use std::collections::HashMap;
use piston::event_loop::*;
use std::path::Path;
use piston::input::*;


use config::TILE_WIDTH;
use config::TILE_HEIGHT;

pub struct GraphicsContext {
    textures : HashMap<String, Texture>,
    translation : (u32, u32),
    width : u32,
    height : u32,
    seed : [u32;4],
    background_tile_textures : Vec<String>
}

impl GraphicsContext {
    fn draw_background(&mut self, context:Context, gl:&mut GlGraphics) {
        let mut rng1: XorShiftRng = SeedableRng::from_seed(self.seed);
        let (width, height) = (TILE_WIDTH, TILE_HEIGHT);
        for i in 0..(self.width / width) + 1 {
            for j in 0..(self.height / height) + 1 {
                let rand = rng1.gen::<u32>() % self.background_tile_textures.len() as u32;
                let filename = self.background_tile_textures.get(rand as usize).unwrap().clone();
                self.draw_texture(context, gl, filename, i * width, j * height, width, height);
            }
        }
    }

    fn transform_camera_coords(&mut self, x : u32, y: u32) -> (u32, u32) {
        let (transl_x, transl_y) = self.translation;
        return (
            x - transl_x + (self.width / 2),
            y - transl_y + (self.height / 2)
        );
    }

    pub fn new(width : u32, height : u32, seed : [u32;4], texts : Vec<String>) -> GraphicsContext {
        let opengl = OpenGL::V3_2;
        GraphicsContext{
            textures : HashMap::new(),
            translation : (0, 0),
            width : width,
            height : height,
            seed : seed,
            background_tile_textures : texts
        }
    }

    pub fn render(&mut self, args : RenderArgs, context:Context, gl:&mut GlGraphics) {
        clear([0.5, 0.2, 0.9, 1.0], gl);
        self.draw_background(context, gl);
    }

    pub fn load_texture(&mut self, filename : String) {
        self.textures.insert(filename.clone(), Texture::from_path(Path::new(&filename)).unwrap());
    }

    pub fn draw_texture(&mut self, context: Context, gl:&mut GlGraphics, filename : String, x : u32, y: u32, width: u32, height: u32) {
        let (x, y) = self.transform_camera_coords(x, y);
        let txt: &Texture = self.textures.get(&filename).unwrap();
        let image = Image::new().rect(square(x as f64, y as f64, width as f64)); //TODO: Do not ignore height
        image.draw(txt, default_draw_state(), context.transform, gl);
    }
}
