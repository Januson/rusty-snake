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
mod snake;

const BOARD_WIDTH: i8 = 30;
const BOARD_HEIGHT: i8 = 30;
const TILE_SIZE: f64 = 25.0;
const UPDATE_TIME: f64 = 0.15;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
        "snake",
        [BOARD_WIDTH as u32 * TILE_SIZE as u32,
        BOARD_HEIGHT as u32 * TILE_SIZE as u32]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new();
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
