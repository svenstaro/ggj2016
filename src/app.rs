use std::mem;
use std::collections::HashMap;
use std::any::Any;

use piston::input::Button;
use piston::input::Button::Keyboard;
use piston::input::Key;
use piston::input::{RenderArgs, UpdateArgs};

use std::cell::RefCell;
use std::cell::Ref;
use std::rc::Rc;
use entity::Entity;

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

    pub fn get_player(&mut self) -> &mut Player {
        let from_map: Option<&mut Box<Entity>> = self.entities.get_mut(&0);
        let from_option: &mut Box<Entity> = from_map.unwrap();
        let player: Option<&mut Player> = from_option.as_any().downcast_mut::<Player>();
        match player {
            Some(a) => a,
            _ => panic!("Error")
        }
    }

    pub fn add_entity(&mut self, b: Box<Entity>) {
        self.entities.insert(self.last_entity_id, b);
        self.last_entity_id += 1;
    }

    pub fn key_press(&mut self, args: Button) {
        self.get_player().key_press(args);
        if args == Keyboard(Key::Space) {
            println!("was");
        }
    }

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
