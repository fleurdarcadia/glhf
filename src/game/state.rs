use crate::config::ui::UI;
use super::player::Player;

use ggez::{
    event::EventHandler,
    graphics::{
        self,
        Color,
    },
    Context,
    GameError,
    GameResult,
};

/// The main game state container.
pub struct State {
    player: Player,
}

impl State {
    pub fn new(ui: &UI) -> Self {
        State {
            player: Player::new(ui),
        }
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        self.player.draw(ctx)?;

        graphics::present(ctx)?;
        ggez::timer::yield_now();

        Ok(())
    }
}
