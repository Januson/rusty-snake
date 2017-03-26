use graphics::clear;
use graphics::context::Context;
use opengl_graphics::GlGraphics;
use piston::input::{ Button, RenderArgs, UpdateArgs };
use rand::{thread_rng, Rng, sample};

use food::Food;
use level::Level;
use level::level;
use settings::Settings;
use snake::Point;
use snake::Snake;

#[derive(PartialEq)]
pub enum State {
    Playing,
    Paused,
    GameOver,
}

pub struct Game<'a: 'b, 'b> {
    pub food: Vec<Food<'b>>,
    level: Level<'a>,
    score: u64,
    settings: &'a Settings,
    pub state: State,
    tiles: Vec<Point>,
    time: f64,
    update_time: f64,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(settings: &'a Settings) -> Game {
        let level = level(settings);
        Game {
            food: vec![],
            level: level,
            score: 0,
            settings: settings,
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

            self.level.render(c, gl);

            for ref mut f in &self.food {
                f.render(c, gl);
            }
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
            self.level.snake.update();

            let head = self.level.snake.tail.front().unwrap().clone();
            if self.level.walls.iter().any(|w| w == &head) || self.level.snake.collides(&head) {
                self.state = State::GameOver;
                println!("Game Over!");
                println!("Score: {}", self.score);
            }
            let i = self.food.iter().position(|ref f| f.point == head);
            if i.is_some() {
                let f = self.food.swap_remove(i.unwrap());
                self.score += f.score as u64;
                let p = *self.level.snake.tail.front().unwrap();
                self.level.snake.tail.push_back(p);
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
                self.level.snake.key_press(button);
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
            .filter(|&x| !self.level.snake.tail.contains(x))
            .filter(|&x| !self.level.walls.contains(x))
            .collect::<Vec<_>>();
        let mut rng = thread_rng();
        let n: usize = rng.gen_range(1, self.settings.max_food) as usize;
        let r = sample(&mut rng, res, n);
        r
    }
}
