use piston::input::Button;
use piston::input::Button::Keyboard;
use piston::input::Key;
use piston::input::{RenderArgs, UpdateArgs};

use std::cell::RefCell;
use std::cell::Ref;
use std::rc::Rc;
use entity::Entity;
use std::collections::HashMap;

use player::Player;

pub struct App {
    /// next unique id
    player : Rc<RefCell<Player>>,
    last_entity_id: u32,
    entities: HashMap<u32, Rc<RefCell<Entity>>>,
}

//fn insert(&mut self, k: K, v: V) -> Option<V>

impl App {
    pub fn new() -> App {
        let mut hm : HashMap<u32, Rc<RefCell<Entity>>> = HashMap::new();
        let mut player = Rc::new(RefCell::new(Player::new()));
        hm.insert(0, player.clone());
        App {
            last_entity_id: 1,
            entities: hm,
            player : player.clone()
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
            e.borrow_mut().update(args);
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
        for (id, e) in &mut self.entities {
            e.borrow_mut().render(args);
        }
    }
}
