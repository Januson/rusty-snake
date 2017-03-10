use graphics::{ color, clear };
use graphics::context::Context;
use opengl_graphics::GlGraphics;
use piston::input::{ Button, RenderArgs, UpdateArgs };
use piston::input::keyboard::Key;
use std::collections::VecDeque;

use snake::*;

#[derive(PartialEq)]
enum State {
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    pub snake: Snake,
    state: State,
    time: f64,
    update_time: f64,
}

impl Game {
    pub fn new() -> Game {
        let mut tail = VecDeque::new();
        tail.push_back(Point { x: 2, y: 3 });
        tail.push_back(Point { x: 2, y: 2 });
        tail.push_back(Point { x: 2, y: 1 });
        Game {
            snake: Snake::new(tail, Key::Up),
            state: State::Playing,
            time: ::UPDATE_TIME,
            update_time: ::UPDATE_TIME,
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let ref c = Context::new_abs(args.width as f64,args.height as f64);
        let bg_color = color::hex("222d4a");
        gl.draw(args.viewport(), |_, gl| {
            if self.state == State::GameOver {
                clear(color::hex("000000"), gl);
                return;
            }
            clear(color::hex("001122"), gl);

            //self.snake.render(Context::new_viewport(args.viewport()).transform, gl);
            self.snake.render(c, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        match self.state {
            State::Paused | State::GameOver => return,
            _ => {},
        }
        self.time += args.dt;
        if self.time > self.update_time {
            self.time -= self.update_time;
            Snake::update(self);
        }
    }

    pub fn key_press(&mut self, button: &Button) {
        use piston_window::Button::Keyboard;
        use piston_window::Key;

        match (*button, &self.state) {
            (Keyboard(Key::Escape), _) => {

            },
            _ => {
                self.snake.key_press(button);
            }
        };
    }
}
