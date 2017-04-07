extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate find_folder;

use opengl_graphics::OpenGL;
use piston::event_loop::{ Events, EventSettings };
use piston::input::*;
use piston::window::WindowSettings;
use piston_window::Glyphs;
use piston_window::PistonWindow;

mod food;
mod game;
mod level;
mod settings;
mod snake;

use settings::Settings;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = Settings::new();
    let mut window: PistonWindow = WindowSettings::new(
        "snake",
        [settings.board_width as u32 * settings.tile_size as u32,
        settings.board_height as u32 * settings.tile_size as u32]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    let mut game = game::Game::new(&settings, glyphs);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(ref args) = e.render_args() {
            game.render(&e, &mut window);
        }

        if let Some(ref args) = e.update_args() {
            game.update(args);
        }
        
        if let Some(ref args) = e.press_args() {
            game.key_press(args);
        }
    }
}
