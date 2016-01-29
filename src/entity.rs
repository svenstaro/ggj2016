use piston::input::{RenderArgs, UpdateArgs};
use cgmath::{Vector2};

pub trait Entity {
    fn update(&mut self, args: UpdateArgs);
    fn render(&mut self, args: RenderArgs);

    /// self does not have to be mutable
    /// returns the position of the entity
    fn get_position(&self) -> Vector2<f32>;
}
