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

fn draw_background(x: u32, y: u32, context: graphics::context::Context, gl_graphics: &mut GlGraphics, txt: &Texture) {
    let (width, height) = txt.get_size();
    for i in 0..x + 1 {
        for j in 0..y + 1 {
            let image = Image::new().rect(square((i * width) as f64, (j * height) as f64, width as f64));
            image.draw(txt, default_draw_state(), context.transform, gl_graphics);
        }
    }
}

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
    let size_image = 160;
    // Add player to entities (player instanciated in app)
    //app.add_entity(Box::new(player::Player::new()));

    let texture = Texture::from_path(Path::new("assets/img/ground/placeholder_01.jpg")).unwrap();

    for e in window {
        if let Some(args) = e.press_args() {
            app.key_press(args);
        }

        if let Some(args) = e.update_args() {
            app.update(args);
        }

        if let Some(args) = e.render_args() {
            let grid_width = args.width / size_image;
            let grid_height = args.height / size_image;
            gl.draw(args.viewport(), |c, gl| {
                clear([0.5, 0.2, 0.9, 1.0], gl);
                draw_background(grid_width, grid_height, c, gl, &texture);
            });
            app.render(args);
        }
    }
}
