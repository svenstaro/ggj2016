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
mod person;
mod graphics_context;

use entity::Entity;

use graphics_context::GraphicsContext;
use piston_window::{ PistonWindow, WindowSettings };
use piston::input::*;
use piston::event_loop::*;
use opengl_graphics::*;
use rand::Rng;
use cgmath::Vector2;

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

    let mut background_textures :Vec<String> = Vec::new();
    background_textures.push(String::from("assets/img/ground/placeholder_01.jpg"));
    background_textures.push(String::from("assets/img/ground/placeholder_02.jpg"));
    let mut gl = GlGraphics::new(opengl);
    let mut ctx = GraphicsContext::new(800, 600, seed, background_textures);
    ctx.load_textures();

    let player_tex = String::from("assets/img/emoji/78.png");
    let person_tex = String::from("assets/img/emoji/77.png");
    ctx.load_texture(&player_tex);
    ctx.load_texture(&person_tex);

    let mut app = app::App::new(player_tex);
    app.add_entity(Box::new(person::Person::new(person_tex, Vector2::new(50.0, 50.0))));
    // Add player to entities (player instanciated in app)
    //app.add_entity(Box::new(player::Player::new()));

    for e in window {
        if let Some(args) = e.press_args() {
            app.key_press(args);
        }

        if let Some(args) = e.release_args() {
            app.key_release(args);
        }

        if let Some(args) = e.update_args() {
            app.update(args);
        }

        ctx.update_translation(app.get_player().get_position().x as u32, app.get_player().get_position().y as u32);

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                ctx.render(args, c, gl);
                app.render(&mut ctx, gl);
            });
        }
    }


}
