use entity::Entity;
use config::TILE_WIDTH;
use config::TILE_HEIGHT;
use cgmath::{Vector2};

pub struct Camera {
    player : Box<Entity>,
    offset : Vector2<i32>
}

impl Camera {
    pub fn new(player : Box<Entity>) -> Camera {
        return Camera{
            player : player,
            offset : Vector2::<i32>::new(0i32, 0i32)
        };
    }

    // pub fn get_render_offset(&self) -> (i32, i32) {
    //     return (
    //         // self.size.width as i32 / 2
    //         //     - (self.player.get_position().x * TILE_WIDTH as f32) as i32
    //         //     + self.offset.x,
    //         // self.size.height as i32 / 2
    //         //     - (self.player.get_position().y * TILE_HEIGHT as f32) as i32
    //         //     + self.offset.y
    //     );
    // }
}
