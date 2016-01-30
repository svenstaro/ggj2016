extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate time;
extern crate rand;
extern crate ai_behavior;
extern crate cgmath;
extern crate opengl_graphics;

mod app;
mod entity;
mod player;
mod config;

use player::Player;
use entity::Entity;

use piston_window::{ PistonWindow, WindowSettings };
use piston::input::*;
use piston::event_loop::*;
use opengl_graphics::*;
use graphics::{ Image, clear, default_draw_state };
use graphics::rectangle::square;
use std::path::Path;
use rand::{Rng, SeedableRng, XorShiftRng};

use cgmath::rad;
use cgmath::{ Vector2, Vector4 };
use cgmath::{ Rotation2, Basis2 };

fn transform_camera_coords(player : &Player, x : u32, y: u32, width : u32, height : u32) -> (i32, i32) {
    return (
        x as i32 - player.get_position().x as i32 + (width as f32 / 2f32) as i32 ,
        y as i32 - player.get_position().y as i32 + (height as f32 / 2f32) as i32
    );
}

fn draw_background(x: u32, y: u32, context: graphics::context::Context, gl_graphics: &mut GlGraphics, textures: &Vec<Texture>, seed: [u32;4], player : &mut Player) {
    let mut rng1: XorShiftRng = SeedableRng::from_seed(seed);
    let txt: &Texture = textures.get(0).unwrap();
    let (width, height) = txt.get_size();
    for i in 0..(x/width) + 1 {
        for j in 0..(y/height) + 1 {
            let (k, l) = transform_camera_coords(player, i, j, x, y);
            let rand = rng1.gen::<u32>() % textures.len() as u32;
            let txt: &Texture = textures.get(rand as usize).unwrap();
            let (width, height) = txt.get_size();
            let image = Image::new().rect(square((i * width) as f64, (j * height) as f64, width as f64));
            image.draw(txt, default_draw_state(), context.transform, gl_graphics);
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let seed: [u32;4] = [rng.gen::<u32>(), rng.gen::<u32>(), rng.gen::<u32>(), rng.gen::<u32>()];
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

    let mut textures :Vec<Texture>= Vec::new();
    textures.push(Texture::from_path(Path::new("assets/img/ground/placeholder_01.jpg")).unwrap());
    textures.push(Texture::from_path(Path::new("assets/img/ground/placeholder_02.jpg")).unwrap());

    for e in window {
        if let Some(args) = e.press_args() {
            app.key_press(args);
            println!("asda");
        }

        if let Some(args) = e.update_args() {
            app.update(args);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                clear([0.5, 0.2, 0.9, 1.0], gl);
                draw_background(args.width, args.height, c, gl, &textures, seed, app.get_player());
            });
            app.render(args);
        }
    }
}
