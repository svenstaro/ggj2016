
use entity::Entity;

use cgmath::rad;
use cgmath::{Vector2, Vector4};
use cgmath::{Rotation2, Basis2};

use piston::input::{RenderArgs, UpdateArgs};

pub struct Player
{
    pos : Vector2<f32>,
    dir : Vector2<f32>
}

impl Player
{
    pub fn new() -> Player {
        Player {
            pos: Vector2::<f32>::new(50.0f32, 50.0f32),
            dir: Vector2::<f32>::new(0.0f32, 0.0f32)
        }
    }
}

impl Entity for Player {
    fn update(&mut self, args: UpdateArgs) {
        self.pos.x += args.dt as f32 * self.dir.x;
        self.pos.y += args.dt as f32 * self.dir.y;
        println!("x:{} y:{}", self.pos.x, self.pos.y);
    }

    fn render(&mut self, args: RenderArgs) {
        //println!("playerrender");
    }

    fn get_position(&self) -> Vector2<f32> {
        Vector2::<f32>::new(self.pos.x, self.pos.y)
    }
}
