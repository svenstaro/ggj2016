use entity::Entity;

pub struct Camera {
    player : Box<Entity>
}

impl Camera {
    pub fn new(player : Box<Entity>) -> Camera {
        return Camera{player : player};
    }
}
