use graphics::color;

pub struct Settings {
    pub board_width: i8,
    pub board_height: i8,
    pub tile_size: f64,
    pub update_time: f64,
    pub board_color: [f32; 4],
    pub snake_color: [f32; 4],
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            board_width: 30,
            board_height: 30,
            tile_size: 25.0,
            update_time: 0.15,
            board_color: color::hex("192731"),
            snake_color: color::hex("8ba673"),
        }
    }
}
