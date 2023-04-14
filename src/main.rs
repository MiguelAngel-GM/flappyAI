mod common;
mod state;
mod player;
mod obstacle;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::state::*;
    pub use crate::obstacle::*;
    pub use crate::player::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const FRAME_DURATION: f32 = 20.0;
    pub const DRAGON_FRAMES: [u16; 12] = [64, 64, 1, 1, 2, 2, 3, 3, 2, 2, 1, 1];
}

use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_title("Flappy Dragon")
        .with_tile_dimensions(16, 16)
        .build()?;

    main_loop(context, State::new())
}
