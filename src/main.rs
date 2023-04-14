mod state;
mod player;
mod obstacle;
mod data;
mod model;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::state::*;
    pub use crate::obstacle::*;
    pub use crate::player::*;
    pub use crate::data::*;
    pub use crate::model::*;
    pub use std::io;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const FRAME_DURATION: f32 = 20.0;
}

use prelude::*;

fn main() -> BError {
    let mut mode = String::new();

    println!("Select mode: Player (0), AI (1)");
    io::stdin().read_line(&mut mode).expect("Failed to read user input");

    let context = BTermBuilder::new()
                .with_font("../resources/flappy32.png", 32, 32)
                .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
                .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
                .with_title("Flappy Dragon")
                .with_tile_dimensions(16, 16)
                .build()?;

    match mode.trim() {
        "1" => {
            main_loop(context, State::new(false))
        }
        _ => {
            main_loop(context, State::new(true))
        }
    }

}
