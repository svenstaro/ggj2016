use piston::input::Button;
use piston::input::Button::Keyboard;
use piston::input::Key;
use piston::input::{RenderArgs, UpdateArgs};

use sprite::*;
use entity::Entity;
use std::collections::HashMap;

use player::Player;

use Size;

pub struct App {
    /// next unique id
    last_entity_id: u32,
    scene: Scene<Size>,
    entities: HashMap<u32, Box<Entity>>,
}

//fn insert(&mut self, k: K, v: V) -> Option<V>

impl App {
    pub fn new() -> App {
        let mut hm : HashMap<u32, Box<Entity>> = HashMap::new();
        hm.insert(0, Box::new(Player::new()));

        App {
            last_entity_id: 1,
            scene: Scene::new(),
            entities: hm,
        }
    }

    pub fn key_press(&mut self, args: Button) {
        if args == Keyboard(Key::Space) {
            println!("was");
        }
    }
/*
    pub fn add_entity(&mut self, e: Box<Entity>) {
        self.entities.push(e);
    }*/

    pub fn update(&mut self, args: UpdateArgs) {
        for (id, e) in &mut self.entities {
            e.update(args);
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
        for (id, e) in &mut self.entities {
            e.render(args);
        }
    }
}
