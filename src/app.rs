use piston::input::Button;
use piston::input::Button::Keyboard;
use piston::input::Key;
use piston::input::{RenderArgs, UpdateArgs};

use entity::Entity;

pub struct App {
    entities: Vec<Box<Entity>>
}

impl App {
    pub fn new() -> App {
        return App {entities: vec![]};
    }

    pub fn key_press(&mut self, args: Button) {
        if args == Keyboard(Key::Space) {
            println!("was");
        }
    }

    pub fn add_entity(&mut self, e: Box<Entity>) {
        self.entities.push(e);
    }

    pub fn update(&mut self, args: UpdateArgs) {
        for e in &mut self.entities {
            e.update(args);
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
        for e in &mut self.entities {
            e.render(args);
        }
    }
}
