use std::collections::HashMap;
use piston::input::UpdateArgs;

use entitymanager::EntityManager;

pub trait System{
	fn update(&mut self, entities: &mut EntityManager, args:UpdateArgs);
}
