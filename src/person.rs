use graphics;
use opengl_graphics::*;
use entity::Entity;
use graphics_context::GraphicsContext;
use app::App;
use std::rc::Rc;
use std::cell::RefCell;

use cgmath::Vector2;

use piston::input::*;
use config::TILE_WIDTH;
use config::TILE_HEIGHT;

use std::any::Any;

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
	fn as_any_mut(&mut self) -> &mut Any{
		self
	}

	fn as_any_ref(&self) -> &Any{
		self
	}

	fn update(&mut self, app: &App, args: UpdateArgs){
		let p = app.get_player_ref();
	}

	fn render(&mut self, ctx : &mut GraphicsContext, gl: &mut GlGraphics){
		ctx.draw_texture(gl, self.emotion.clone(), self.position.x as u32, self.position.y as u32, TILE_WIDTH, TILE_HEIGHT);
	}

	fn get_position(&self) -> Vector2<f64>{
		return self.position;
	}
}
