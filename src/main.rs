mod config;
mod game;
mod physics;

use game::{
    state::State,
    player::Player,
};
use config::ui::UI;

//use ggez::{
//    graphics,
//    Context,
//    GameResult,
//};

fn main() -> ggez::GameResult {
    let config = UI::default();
    let (ctx, events_loop) = ggez::ContextBuilder::new("Shooter", "Arcadia Rose <fleurdarcadia@protonmail.com>")
        .window_setup(ggez::conf::WindowSetup::default().title("Shooter"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(config.width, config.height))
        .build()?;

    let state = State::new(&config);

    ggez::event::run(ctx, events_loop, state);
}
