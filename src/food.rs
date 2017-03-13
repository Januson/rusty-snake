use graphics::{ color, rectangle };
use graphics::context::Context;
use opengl_graphics::GlGraphics;

use settings::Settings;
use snake::Point;

pub struct Food<'a> {
    pub point: Point,
    settings: &'a Settings,
}

impl<'a> Food<'a> {
    pub fn new(settings: &'a Settings) -> Food {
        Food {
            point: Point { x: 20, y: 20 },
            settings: settings,
        }
    }

    pub fn render(&self, c: &Context, gl: &mut GlGraphics) {
        rectangle(
            color::hex("b83e3e"),
            rectangle::square(
                self.point.x as f64 * self.settings.tile_size,
                self.point.y as f64 * self.settings.tile_size,
                self.settings.tile_size
            ),
            c.transform, gl
        );
    }

    pub fn update() {}
}
