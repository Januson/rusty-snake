use graphics::{ color, rectangle };
use graphics::context::Context;
use opengl_graphics::GlGraphics;
use piston::input::Button;
use piston::input::keyboard::Key;
use std::collections::VecDeque;

use game::Game;

pub struct Point {
    pub x: i8,
    pub y: i8,
}

pub struct Snake {
    tail: VecDeque<Point>,
    keys: VecDeque<Key>,
    last_pressed: Key,
}

impl Snake {
    pub fn new (tail: VecDeque<Point>, key: Key) -> Snake {
        Snake {
            tail: tail,
            keys: VecDeque::new(),
            last_pressed: key,
        }
    }

    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {
        for p in self.tail.iter() {
            rectangle(
                color::hex("8ba673"),
                rectangle::square(p.x as f64 * ::TILE_SIZE, p.y as f64 * ::TILE_SIZE, ::TILE_SIZE),
                c.transform, gl
            );
        }        
    }

    
    pub fn update(g: &mut Game) {
        use piston::input::keyboard::Key::*;
        if g.snake.keys.is_empty() {
            g.snake.keys.push_back(g.snake.last_pressed);
        }
        let k = g.snake.keys.pop_front().unwrap();
        Snake::mv(g, match k {
            Right => Point { x: 1, y: 0 },
            Left => Point { x: -1, y: 0 },
            Up => Point { x: 0, y: -1 },
            Down => Point { x: 0, y: 1 },
            _ => panic!("Only arrows are allowed."),
        })
    }

    fn mv(g: &mut Game, p: Point) {
        let mut xy = Point {
            x: g.snake.tail.front().unwrap().x + p.x,
            y: g.snake.tail.front().unwrap().y + p.y,
        };
        if xy.x >= ::BOARD_WIDTH {
            xy.x = 0;
        } else if xy.x < 0 {
            xy.x = ::BOARD_WIDTH - 1;
        }
        if xy.y >= ::BOARD_HEIGHT {
            xy.y = 0;
        } else if xy.y < 0 {
            xy.y = ::BOARD_HEIGHT - 1;
        }


        g.snake.tail.pop_back();
        g.snake.tail.push_front(xy);
    }

    pub fn key_press(&mut self, button: &Button) {
        use piston_window::Button::Keyboard;
        use piston::input::keyboard::Key::*;

        let key = match *button {
            Keyboard(Right) => Right,
            Keyboard(Left) => Left,
            Keyboard(Up) => Up,
            Keyboard(Down) => Down,
            _ => return,
        };
        if true {
            self.keys.push_back(key);
            self.last_pressed = key;
        } 
    }
}
