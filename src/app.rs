use piston::input::Button;
use piston::input::Button::Keyboard;
use piston::input::Key;
use piston::input::{RenderArgs, UpdateArgs};

use sprite::*;

use Size;

// use opengl_graphics::{
//     GlGraphics,
//     Texture,
// };

pub struct App {
    pub scene: Scene<Size>
}

impl App {
    pub fn new() -> App {
        return App {scene: Scene::new()};
    }

    pub fn key_press(&mut self, args: Button) {
        if args == Keyboard(Key::Space) {
            println!("was");
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
    }

    pub fn update(&mut self, args: UpdateArgs) {
    }
}
