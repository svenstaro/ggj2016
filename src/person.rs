use graphics;
use opengl_graphics::*;
use entity::Entity;
use graphics_context::GraphicsContext;

use cgmath::Vector2;

use piston::input::*;
use config::TILE_WIDTH;
use config::TILE_HEIGHT;

use std::any::Any;

use entitymanager::EntityManager;

pub struct Person {
	emotion: String,
	position: Vector2<f64>
}

impl Person{
		pub fn new(texture: String, position: Vector2<f64>) -> Person{
			return Person{emotion: texture, position: position};
		}
}

impl Entity for Person {
	/// Required to downcast Entity to precise type (see player.rs)
	fn as_any(&mut self) -> &mut Any{
		self
	}

	fn update(&mut self, args: UpdateArgs){

	}

	fn render(&mut self, ctx : &mut GraphicsContext, c: graphics::context::Context, gl: &mut GlGraphics){
		ctx.draw_texture(c, gl, self.emotion.clone(), self.position.x as u32, self.position.y as u32, TILE_WIDTH, TILE_HEIGHT);
	}

	fn get_position(&self) -> Vector2<f64>{
		return self.position;
	}
}
