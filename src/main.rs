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

use piston_window::{ PistonWindow, WindowSettings };
use piston::input::*;
use piston::event_loop::*;
use opengl_graphics::*;

use cgmath::rad;
use cgmath::{Vector2, Vector4};
use cgmath::{Rotation2, Basis2};

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

    // Add player to entities
    app.add_entity(Box::new(player::Player::new()));

    for e in window {
        if let Some(args) = e.press_args() {
            app.key_press(args);
        }

        if let Some(args) = e.update_args() {
            app.update(args);
        }

        if let Some(args) = e.render_args() {
            app.render(args);
        }
    }
}
