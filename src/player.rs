
mod app;

pub trait Entity {
    fn update(&mut self, args: UpdateArgs);
    fn render(&mut self, args: RenderArgs);
}


pub struct Player
{
    pos : Position,
    dir : Position
}

impl Player
{
    pub fn new() -> Player {
        App {
            pos: (50.0f32, 50.0f32)
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
}
