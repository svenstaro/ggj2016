use piston::input::{RenderArgs, UpdateArgs};
use Position;

pub trait Entity {
    fn update(&mut self, args: UpdateArgs);
    fn render(&mut self, args: RenderArgs);
    fn get_position(&self)->&Position;
}
