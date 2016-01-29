

pub struct Player
{
    pos : (f32, f32),
    dir : (f32, f32)
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
    fn update(dt : f64) {

    }

    fn render() {

    }
}
