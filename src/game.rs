use graphics::clear;
use graphics::context::Context;
use opengl_graphics::GlGraphics;
use piston::input::{ Button, RenderArgs, UpdateArgs };
use piston::input::keyboard::Key;
use rand::{thread_rng, sample};

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
    tiles: Vec<Point>,
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
            tiles: Game::init_tiles(settings),
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
            if self.food.is_empty() {
                let mut food = Vec::new();
                for t in self.find_empty_tiles() {
                    let f = Food::new(t.x, t.y, self.settings);
                    food.push(f);
                }
                self.food.extend(food);
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

    fn init_tiles(settings: &Settings) -> Vec<Point> {
        let mut tiles: Vec<Point> = Vec::new();
        for x in 0..settings.board_width {
            for y in 0..settings.board_height {
                tiles.push(Point {x: x, y: y});
            }
        }
        tiles
    }

    fn find_empty_tiles(&self) -> Vec<&Point> {
        let res: Vec<&Point> = self.tiles.iter()
            .filter(|&x| !self.snake.tail.contains(x))
            .collect::<Vec<_>>();
        let mut rng = thread_rng();
        let r = sample(&mut rng, res, 5);
        r
    }
}
