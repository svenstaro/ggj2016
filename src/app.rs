use piston::input::Button;
use piston::input::Button::Keyboard;
use piston::input::Key;
use piston::input::{RenderArgs, UpdateArgs};

use sprite::*;
use entity::Entity;

use Size;

pub struct App {
    scene: Scene<Size>,
    entities: Vec<Box<Entity>>
}

impl App {
    pub fn new() -> App {
        return App {scene: Scene::new(), entities: vec![]};
    }

    pub fn key_press(&mut self, args: Button) {
        if args == Keyboard(Key::Space) {
            println!("was");
        }
    }

    pub fn update(&mut self, args: UpdateArgs) {
        for &entity in &self.entities {
            entity.update(args)
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
    }
}
