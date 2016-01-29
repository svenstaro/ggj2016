use piston::input::{RenderArgs, UpdateArgs};

pub trait Entity {
    fn update(&mut self, args: UpdateArgs);
    fn render(&mut self, args: RenderArgs);
}
