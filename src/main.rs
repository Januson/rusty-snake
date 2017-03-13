extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::{ Events, EventSettings };
use piston::input::*;
use piston::window::WindowSettings;

mod food;
mod game;
mod settings;
mod snake;

use settings::Settings;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = Settings::new();
    let mut window: Window = WindowSettings::new(
        "snake",
        [settings.board_width as u32 * settings.tile_size as u32,
        settings.board_height as u32 * settings.tile_size as u32]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new(&settings);
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(ref args) = e.render_args() {
            game.render(args, &mut gl);
        }

        if let Some(ref args) = e.update_args() {
            game.update(args);
        }
        
        if let Some(ref args) = e.press_args() {
            game.key_press(args);
        }
    }
}
