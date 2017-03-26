use graphics::{ color, rectangle };
use graphics::context::Context;
use opengl_graphics::GlGraphics;
use piston::input::keyboard::Key;
use std::collections::VecDeque;

use settings::Settings;
use snake::Point;
use snake::Snake;

pub struct Level<'a> {
    settings: &'a Settings,
    pub snake: Snake<'a>,
    pub walls: Vec<Point>,
}

impl<'a> Level<'a> {
    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {
        for wall in self.walls.iter() {
            rectangle(
                color::hex("111111"),
                rectangle::square(
                    wall.x as f64 * self.settings.tile_size,
                    wall.y as f64 * self.settings.tile_size,
                    self.settings.tile_size
                ),
                c.transform, gl
            );
        }
        self.snake.render(c, gl);
    }
}

macro_rules! walls {
    ( $( $x:expr, $y:expr ),* ) => {
        {
            vec![
            $(
                Point { x: $x, y: $y },
            )*
            ]
        }
    };
}

pub fn level(settings: &Settings) -> Level {
    let walls = walls![
        0,0, 1,0, 2,0, 3,0, 4,0, 5,0, 6,0, 7,0, 8,0, 9,0, 10,0, 11,0, 12,0, 13,0, 14,0, 15,0, 16,0, 17,0,
        18,0, 19,0, 20,0, 21,0, 22,0, 23,0, 24,0, 25,0, 26,0, 27,0, 28,0, 29,0,
        29,1, 29,2, 29,3, 29,4, 29,5, 29,6, 29,7, 29,8, 29,9, 29,10, 29,11, 29,12, 29,13, 29,14, 29,15,
        29,16, 29,17, 29,18, 29,19, 29,20, 29,21, 29,22, 29,23, 29,24, 29,25, 29,26, 29,27, 29,28,
        0,29, 1,29, 2,29, 3,29, 4,29, 5,29, 6,29, 7,29, 8,29, 9,29, 10,29, 11,29, 12,29, 13,29, 14,29, 15,29,
        16,29, 17,29, 18,29, 19,29, 20,29, 21,29, 22,29, 23,29, 24,29, 25,29, 26,29, 27,29, 28,29, 29,29,
        0,1, 0,2, 0,3, 0,4, 0,5, 0,6, 0,7, 0,8, 0,9, 0,10, 0,11, 0,12, 0,13, 0,14, 0,15, 0,16, 0,17,
        0,18, 0,19, 0,20, 0,21, 0,22, 0,23, 0,24, 0,25, 0,26, 0,27, 0,28
    ];

    let mut tail = VecDeque::new();
    tail.push_back(Point { x: 12, y: 11 });
    tail.push_back(Point { x: 12, y: 12 });
    tail.push_back(Point { x: 12, y: 13 });
    let snake = Snake::new(tail, Key::Up, settings);

    Level {
        settings: settings,
        snake: snake,
        walls: walls,
    }
}