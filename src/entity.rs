use piston::input::{RenderArgs, UpdateArgs};
use cgmath::{Vector2};
use std::any::Any;


pub trait Entity {
    /// Required to downcast Entity to precise type (see player.rs)
    fn as_any(&mut self) -> &mut Any;

    fn update(&mut self, args: UpdateArgs);
    fn render(&mut self, args: RenderArgs);

    /// self does not have to be mutable
    /// returns the position of the entity
    fn get_position(&self) -> Vector2<f32>;
}
