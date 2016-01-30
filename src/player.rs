use std::any::Any;

use graphics;
use opengl_graphics::*;
use graphics_context::GraphicsContext;
use entity::Entity;
use graphics::{Image, default_draw_state, Graphics};
use config::TILE_WIDTH;
use config::TILE_HEIGHT;

use cgmath::rad;
use cgmath::{Vector2, Vector4};
use cgmath::{Rotation2, Basis2};

use piston::input::*;

pub struct Player
{
    pos : Vector2<f64>,
    dir : Vector2<f64>,
    emotion: String,
}

impl Player
{
    pub fn new(texture: String) -> Player {
        Player {
            pos: Vector2::<f64>::new(50.0f64, 50.0f64),
            dir: Vector2::<f64>::new(0.0f64, 0.0f64),
            emotion: texture
        }
    }

    pub fn key_press(&mut self, button: Button) {
        let mut x_mov = 0.0f64;
        let mut y_mov = 0.0f64;

        match button {
            Button::Keyboard(Key::Left) => { x_mov -= 1.0f64 },
            Button::Keyboard(Key::Right) => { x_mov += 1.0f64 },
            Button::Keyboard(Key::A) => { x_mov -= 1.0f64 },
            Button::Keyboard(Key::D) => { x_mov += 1.0f64 },

            Button::Keyboard(Key::Up) => { y_mov += 1.0f64 },
            Button::Keyboard(Key::Down) => { y_mov -= 1.0f64 },
            Button::Keyboard(Key::W) => { y_mov += 1.0f64 },
            Button::Keyboard(Key::S) => { y_mov -= 1.0f64 },
            _ => {}
        };

        self.dir.x = x_mov;
        self.dir.y = y_mov;

        /*if (x_mov != 0.0f64 || y_mov != 0.0f64) {
            auto position = entity.component<Position>();

            glm::vec2 direction(radius, angle);
            glm::vec2 velocity = float(dt) * glm::normalize(direction);
            velocity *= MAX_SPEED;

            // adjust angle speed to match "radial" speed via arc length
            if (position->position()[0] != 0) { // only adjust if radius != 0 ????
                velocity[1] /= position->position()[0];
            }

            velocity_e->m_accelerating = true;
            glm::vec2 d = velocity_e->m_desired_velocity - velocity;
            if(glm::length(d) > 0.01)
            {
              velocity_e->m_desired_velocity = velocity;
              velocity_e->m_start_velocity = velocity_e->m_velocity;
              velocity_e->m_alpha = 0.0f;
              velocity_e->m_alpha_step = ACCELERATION_STEP;
            }
        }*/
    }
}

impl Entity for Player {
    fn as_any(&mut self) -> &mut Any {
        self
    }

    fn update(&mut self, args: UpdateArgs) {
        self.pos.x += 100.0f64 * args.dt as f64 * self.dir.x;
        self.pos.y += 100.0f64 * args.dt as f64 * self.dir.y;
        println!("x:{} y:{}", self.pos.x, self.pos.y);
    }

    fn render(&mut self, ctx : &mut GraphicsContext, c: graphics::context::Context, gl: &mut GlGraphics) {
        ctx.draw_texture(c, gl, self.emotion.clone(), self.pos.x as u32, self.pos.y as u32, TILE_WIDTH, TILE_HEIGHT);
    }

    fn get_position(&self) -> Vector2<f64> {
        Vector2::<f64>::new(self.pos.x, self.pos.y)
    }
}
