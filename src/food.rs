use piston_window::Context;
use piston_window::G2d;
use piston_window::rectangle;

use settings::Settings;
use snake::Point;

pub struct Food<'a> {
    pub point: Point,
    pub score: u16,
    settings: &'a Settings,
}

impl<'a> Food<'a> {
    pub fn new(x: i8, y: i8, settings: &'a Settings) -> Food {
        Food {
            point: Point { x: x, y: y },
            score: 200,
            settings: settings,
        }
    }

    pub fn render(&self, c: &Context, g: &mut G2d) {
        rectangle(
            self.settings.food_color,
            rectangle::square(
                self.point.x as f64 * self.settings.tile_size,
                self.point.y as f64 * self.settings.tile_size,
                self.settings.tile_size
            ),
            c.transform, g
        );
    }

    pub fn update() {}
}
