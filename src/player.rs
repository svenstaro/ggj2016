
use Position;
use entity::Entity;

use piston::input::{RenderArgs, UpdateArgs};

pub struct Player
{
    pos : Position,
    dir : Position
}

impl Player
{
    pub fn new() -> Player {
        Player {
            pos: Position::new(50.0f32, 50.0f32),
            dir: Position::new(0.0f32, 0.0f32)
        }
    }
}

impl Entity for Player {
    fn update(&mut self, args: UpdateArgs) {
        println!("playerupdate");
    }

    fn render(&mut self, args: RenderArgs) {
        println!("playerrender");
    }

    fn get_position(&self) -> &Position {
        return &self.pos;
    }
}
