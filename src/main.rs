extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate time;
extern crate rand;
extern crate ai_behavior;
extern crate cgmath;
extern crate opengl_graphics;

mod app;
mod camera;
mod entity;
mod player;
mod config;

use piston_window::{ PistonWindow, WindowSettings };
use piston::input::*;
use piston::event_loop::*;
use opengl_graphics::*;
use graphics::{ Image, clear, default_draw_state };
use graphics::rectangle::square;
use std::path::Path;

use cgmath::rad;
use cgmath::{ Vector2, Vector4 };
use cgmath::{ Rotation2, Basis2 };

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("GGJ2016", [800, 600])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    window.set_ups(60);
    let mut gl = GlGraphics::new(opengl);

    let mut app = app::App::new();

    // Add player to entities (player instanciated in app)
    //app.add_entity(Box::new(player::Player::new()));
    
    let image = Image::new().rect(square(0.0, 0.0, 200.0));
    let texture = Texture::from_path(Path::new("assets/img/emoji/60.png")).unwrap();

    for e in window {
        if let Some(args) = e.press_args() {
            app.key_press(args);
        }

        if let Some(args) = e.update_args() {
            app.update(args);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                clear([0.5, 0.2, 0.9, 1.0], gl);
                image.draw(&texture, default_draw_state(), c.transform, gl);
            });
            app.render(args);
        }
    }
}
