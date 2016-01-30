use std::collections::HashMap;

use entity::Entity;

pub struct EntityManager {
	last_entity_id: u32,
    entities: HashMap<u32, Box<Entity>>
}

impl EntityManager {
	pub fn new() -> EntityManager {
		EntityManager{
			last_entity_id: 0,
			entities: HashMap::new()
		}
	}

	pub fn add_entity(&mut self, entity: Box<Entity>){
		self.entities.insert(self.last_entity_id, entity);
		self.last_entity_id += 1;
	}

	pub fn remove_entity(&mut self, id:u32){
		self.entities.remove(&id);
	}

	pub fn get_entity(&mut self, id: u32) -> Option<&mut Box<Entity>>{
		return self.entities.get_mut(&id);
	}

	pub fn get_size(&self) -> usize {
		return self.entities.len();
	}
}
