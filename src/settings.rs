use piston_window::color;

pub struct Settings {
    pub board_width: i8,
    pub board_height: i8,
    pub max_food: i8,
    pub tile_size: f64,
    pub update_time: f64,
    pub board_color: [f32; 4],
    pub food_color: [f32; 4],
    pub snake_color: [f32; 4],
    pub wall_color: [f32; 4],
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            board_width: 30,
            board_height: 30,
            max_food: 4,
            tile_size: 20.0,
            update_time: 0.15,
            board_color: color::hex("192731"),
            food_color: color::hex("b83e3e"),
            snake_color: color::hex("8ba673"),
            wall_color: color::hex("111111"),
        }
    }
}
