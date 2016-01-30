use piston::input::UpdateArgs;
use cgmath::Vector2;
use std::any::Any;
use graphics;
use opengl_graphics::GlGraphics;
use app::App;
use graphics_context::GraphicsContext;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Entity {
    /// Required to downcast Entity to precise type (see player.rs)
   fn as_any_mut(&mut self) -> &mut Any;
   fn as_any_ref(&self) -> &Any;

    fn update(&mut self, app: &App, args: UpdateArgs);
    fn render(&mut self, ctx : &mut GraphicsContext, gl: &mut GlGraphics);

    /// self does not have to be mutable
    /// returns the position of the entity
    fn get_position(&self) -> Vector2<f64>;
}
