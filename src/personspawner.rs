use piston::input::UpdateArgs;

use cgmath::Vector2;

use rand;
use rand::Rng;

use config;

use entitymanager::EntityManager;
use system::System;
use entity::Entity;
use person::Person;

pub struct PersonSpawner {
	no_of_persons: u32
}

impl PersonSpawner {
	pub fn new() -> PersonSpawner {
		PersonSpawner {
			no_of_persons: 0
		}
	}
}

impl System for PersonSpawner {
	fn update(&mut self, entities: &mut EntityManager, args:UpdateArgs){
		if self.no_of_persons < 5 {
			let mut rng = rand::thread_rng();
			let pos = Vector2::<f64>::new((rng.gen::<u32>() % config::MAP_WIDTH) as f64, (rng.gen::<u32>() % config::MAP_HEIGHT) as f64);
			entities.add_entity(Box::new(Person::new("assets/img/emoji/33.png".to_string(), pos)));
			self.no_of_persons += 1;
		}
	}
}
