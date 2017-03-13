use graphics::clear;
use graphics::context::Context;
use opengl_graphics::GlGraphics;
use piston::input::{ Button, RenderArgs, UpdateArgs };
use piston::input::keyboard::Key;
use std::collections::VecDeque;

use food::*;
use settings::Settings;
use snake::*;

#[derive(PartialEq)]
pub enum State {
    Playing,
    Paused,
    GameOver,
}

pub struct Game<'a: 'b, 'b> {
    pub food: Vec<Food<'b>>,
    settings: &'a Settings,
    pub snake: Snake<'a>,
    pub state: State,
    time: f64,
    update_time: f64,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(settings: &'a Settings) -> Game {
        let mut tail = VecDeque::new();
        tail.push_back(Point { x: 12, y: 11 });
        tail.push_back(Point { x: 12, y: 12 });
        tail.push_back(Point { x: 12, y: 13 });
        Game {
            food: vec![],
            settings: settings,
            snake: Snake::new(tail, Key::Up, settings),
            state: State::Playing,
            time: settings.update_time,
            update_time: settings.update_time,
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        let ref c = Context::new_abs(args.width as f64,args.height as f64);
        gl.draw(args.viewport(), |_, gl| {
            if self.state == State::GameOver {
                clear(self.settings.board_color, gl);
                return;
            }
            clear(self.settings.board_color, gl);

            if self.food.is_empty() {
                let f = Food::new(self.settings);
                &self.food.push(f);
            }

            for ref mut f in &self.food {
                f.render(c, gl);
            }

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
            self.snake.update();

            let head = self.snake.tail.front().unwrap().clone();
            if self.snake.collides(&head) {
                self.state = State::GameOver;
                println!("Game Over!");
            }
            let i = self.food.iter().position(|ref f| f.point == head);
            if i.is_some() {
                let f = self.food.swap_remove(i.unwrap());
                let p = *self.snake.tail.front().unwrap();
                self.snake.tail.push_back(p);
            }
        }
    }

    pub fn key_press(&mut self, button: &Button) {
        use piston_window::Button::Keyboard;
        use piston_window::Key;

        match *button {
            Keyboard(Key::Escape) => {

            },
            Keyboard(Key::P) if self.state == State::Playing => {
                self.state = State::Paused;
            },
            Keyboard(Key::P) if self.state == State::Paused => {
                self.state = State::Playing;
            },
            _ => {
                self.snake.key_press(button);
            }
        };
    }
}
