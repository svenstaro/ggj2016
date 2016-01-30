use std::collections::HashMap;

use graphics;
use opengl_graphics::*;
use graphics_context::GraphicsContext;

use piston::input::Button;
use piston::input::UpdateArgs;

use entitymanager::EntityManager;
use entity::Entity;
use system::System;

use player::Player;

pub struct App {
    /// next unique id
    player: Player,
    entities: EntityManager,
    systems: Vec<Box<System>>
}

//fn insert(&mut self, k: K, v: V) -> Option<V>

impl App {
    pub fn new(player_texture: String) -> App {
        App {
            player: Player::new(player_texture),
            entities: EntityManager::new(),
            systems: Vec::new()
        }
    }

    pub fn get_player(&mut self) -> &mut Player {
        return &mut self.player;
    }

    pub fn add_entity(&mut self, entity: Box<Entity>) {
        self.entities.add_entity(entity);
    }

    pub fn add_system(&mut self, system: Box<System>){
        self.systems.push(system);
    }

    pub fn key_press(&mut self, args: Button) {
        self.get_player().key_press(args);
    }

    pub fn key_release(&mut self, args: Button) {
        self.get_player().key_release(args);
    }

    pub fn update(&mut self, args: UpdateArgs) {
        for system in &mut self.systems {
            system.update(&mut self.entities, args);
        }
        self.player.update(args);

        for id in 0..self.entities.get_size()+1 {
            let maybe_entity = self.entities.get_entity(id as u32);
            match maybe_entity {
                Some(maybe_entity) => maybe_entity.update(args),
                None => {}//println!("no entity")
            }
        }
    }

    pub fn render(&mut self, ctx : &mut GraphicsContext, context: graphics::context::Context, gl_graphics: &mut GlGraphics) {
        for id in 0..self.entities.get_size()+1 {
            let maybe_entity = self.entities.get_entity(id as u32);
            match maybe_entity {
                Some(maybe_entity) => maybe_entity.render(ctx, context, gl_graphics),
                None => {}
            }
        }

        self.player.render(ctx, context, gl_graphics);
    }
}
