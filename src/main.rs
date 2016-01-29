extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate time;
extern crate rand;

use piston_window::{ PistonWindow, WindowSettings };
use piston::input::*;

mod app;

fn main() {
    let window: PistonWindow = WindowSettings::new("GGJ2016", [800, 600])
        .exit_on_esc(true).build().unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let mut app = app::App::new();
    let mut counter = 0;

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
