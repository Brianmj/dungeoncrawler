mod map;
mod player;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
}

use prelude::*;

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
    .with_title("Dungeon Crawler")
        .with_fps_cap(30.)
        .build()?;

     main_loop(ctx, State::new())
}

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self{ map : Map::new(),
        player : Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2))}
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        if let Some(key) = ctx.key {
            if key == VirtualKeyCode::Escape {
                ctx.quit();
            }
        }

        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}