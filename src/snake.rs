use graphics::rectangle;
use graphics::context::Context;
use opengl_graphics::GlGraphics;
use piston::input::Button;
use piston::input::keyboard::Key;
use std::collections::VecDeque;

use settings::Settings;

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i8,
    pub y: i8,
}

pub struct Snake<'a> {
    pub tail: VecDeque<Point>,
    keys: VecDeque<Key>,
    last_pressed: Key,
    settings: &'a Settings,
}

impl<'a> Snake<'a> {
    pub fn new (tail: VecDeque<Point>, key: Key, settings: &'a Settings) -> Snake {
        Snake {
            tail: tail,
            keys: VecDeque::new(),
            last_pressed: key,
            settings: settings,
        }
    }

    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {
        for p in self.tail.iter() {
            rectangle(
                self.settings.snake_color,
                rectangle::square(
                    p.x as f64 * self.settings.tile_size,
                    p.y as f64 * self.settings.tile_size,
                    self.settings.tile_size),
                c.transform, gl
            );
        }        
    }

    
    pub fn update(&mut self) {
        use piston::input::keyboard::Key::*;
        if self.keys.is_empty() {
            self.keys.push_back(self.last_pressed);
        }
        let k = self.keys.pop_front().unwrap();
        self.mv(match k {
            Right => Point { x: 1, y: 0 },
            Left => Point { x: -1, y: 0 },
            Up => Point { x: 0, y: -1 },
            Down => Point { x: 0, y: 1 },
            _ => panic!("Only arrows are allowed."),
        })
    }

    fn mv(&mut self, p: Point) {
        let mut head = Point {
            x: self.tail.front().unwrap().x + p.x,
            y: self.tail.front().unwrap().y + p.y,
        };
        if head.x >= self.settings.board_width {
            head.x = 0;
        } else if head.x < 0 {
            head.x = self.settings.board_width - 1;
        }
        if head.y >= self.settings.board_height {
            head.y = 0;
        } else if head.y < 0 {
            head.y = self.settings.board_height - 1;
        }

        self.tail.pop_back();
        self.tail.push_front(head);
    }

    pub fn collides(&self, point: &Point) -> bool {
        self.tail.iter().skip(1).any(|&t| &t == point)
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
        if Snake::opposite_arrow(key) != self.last_pressed {
            self.keys.push_back(key);
            self.last_pressed = key;
        } 
    }

    fn opposite_arrow(key: Key) -> Key {
        match key {
            Key::Down => Key::Up,
            Key::Up => Key::Down,
            Key::Left => Key::Right,
            Key::Right => Key::Left,
            other => other,
        }
    }
}
