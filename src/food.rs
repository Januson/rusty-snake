use graphics::{ color, rectangle };
use graphics::context::Context;
use opengl_graphics::GlGraphics;

use snake::Point;

pub struct Food {
    pub point: Point,
}

impl Food {
    pub fn new() -> Food {
        Food {
            point: Point { x: 20, y: 20 },
        }
    }

    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {
        rectangle(
            color::hex("b83e3e"),
            rectangle::square(
                self.point.x as f64 * ::TILE_SIZE,
                self.point.y as f64 * ::TILE_SIZE, ::TILE_SIZE),
            c.transform, gl
        );
    }

    pub fn update() {}
}
