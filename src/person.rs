use graphics;
use opengl_graphics::*;
use entity::Entity;

use cgmath::rad;
use cgmath::{Vector2, Vector4};
use cgmath::{Rotation2, Basis2};

use piston::input::*;
use graphics::{Image, default_draw_state, Graphics};

use std::any::Any;

pub struct Person {
	emotion: Texture,
	position: Vector2<f64>
}

impl Person{
		pub fn new(texture: Texture, position: Vector2<f64>) -> Person{
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

	fn render(&mut self, context: graphics::context::Context, gl_graphics: &mut GlGraphics, args: RenderArgs){
		let (tex_width, tex_height) = self.emotion.get_size();
		// let rect = Rectangle::new([1.0,0.0,0.0,1.0], )
		let image = Image::new().rect([self.position.x, self.position.y, tex_width as f64, tex_height as f64]);//Rectangle::new(self.position.x, self.position.y, tex_width, tex_height));
		image.draw(&self.emotion, default_draw_state(), context.transform, gl_graphics);
	}

	fn get_position(&self) -> Vector2<f64>{
		return self.position;
	}
}
