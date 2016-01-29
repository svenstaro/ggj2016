extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate time;
extern crate rand;
extern crate ai_behavior;
extern crate sprite;

mod app;
mod camera;
mod entity;

use piston_window::{ PistonWindow, WindowSettings };
use piston::input::*;
use piston::event_loop::*;
use graphics::*;

pub struct Position {
    x: f32,
    y: f32
}

pub struct Size {
    width: u32,
    height: u32
}

impl ImageSize for Size {
    fn get_size(&self) -> (u32, u32) {
        return (self.width, self.height);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("GGJ2016", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });
    window.set_ups(60);

    let mut app = app::App::new();

    for e in window {
        if let Some(args) = e.render_args() {
            app.render(args);
        }

        if let Some(args) = e.update_args() {
            app.update(args);
        }

        if let Some(args) = e.press_args() {
            app.key_press(args);
        }
    }
}
