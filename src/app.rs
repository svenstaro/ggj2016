extern crate piston;

use piston::input::Button;
use piston::input::Button::Keyboard;
use piston::input::Key;
use piston::input::{RenderArgs, UpdateArgs};

// use opengl_graphics::{
//     GlGraphics,
//     Texture,
// };

pub struct App {
    lol: i32
}

impl App {
    pub fn new() -> App {
        return App {lol: 2};
    }

    pub fn key_press(&mut self, args: Button) {
        if args == Keyboard(Key::Space) {
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
    }

    pub fn update(&mut self, args: UpdateArgs) {
    }
}
