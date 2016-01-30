use std::collections::HashMap;

use graphics;
use opengl_graphics::*;
use graphics_context::GraphicsContext;

use piston::input::Button;
use piston::input::UpdateArgs;
use std::rc::Rc;
use std::cell::RefCell;

use entity::Entity;

use player::Player;

pub struct App {
    /// next unique id
    entities: Vec<Rc<RefCell<Box<Entity>>>>,
}

//fn insert(&mut self, k: K, v: V) -> Option<V>

impl App {
    pub fn new(player_text: String) -> App {
        let mut hm : Vec<Rc<RefCell<Box<Entity>>>> = Vec::new();
        hm.push(Rc::new(RefCell::new(Box::new(Player::new(player_text)))));

        App {
            entities: hm
        }
    }

    pub fn get_player_mut(&mut self) -> &mut Player {
        //let from_map = ;//;
        //let from_option = from_map;
        let player: Option<&mut Player> = self.entities.get(0).unwrap().borrow_mut().as_any_mut().downcast_mut::<Player>();
        match player {
            Some(a) => a,
            _ => panic!("Error")
        }
    }

    pub fn get_player_ref(&self) -> &Player {
        let from_map = self.entities.get(0).unwrap().borrow();
        //let from_option: &Box<Entity> = from_map.unwrap();
        let player: Option<&Player> = from_map.as_any_ref().downcast_ref::<Player>();
        match player {
            Some(a) => a,
            _ => panic!("Error")
        }
    }

    pub fn add_entity(&mut self, b: Box<Entity>) {
        self.entities.push(Rc::new(RefCell::new(b)));
    }

    pub fn key_press(&mut self, args: Button) {
        self.get_player_mut().key_press(args);
    }

    pub fn key_release(&mut self, args: Button) {
        self.get_player_mut().key_release(args);
    }

    pub fn update(&mut self, args: UpdateArgs) {
        for e in self.entities.clone() {
            e.borrow_mut().update(&self, args);
        }
    }

    pub fn render(&mut self, ctx : &mut GraphicsContext, gl:&mut GlGraphics) {
        for e in self.entities.clone() {
            e.borrow_mut().render(ctx, gl);
        }
    }
}
