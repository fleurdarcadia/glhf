use crate::config::ui::UI;
use crate::physics::motion;
use super::player;

use chrono::prelude::*;
use chrono::offset::Utc;

use ggez::{
    event::{
        EventHandler,
        KeyCode,
        KeyMods,
    },
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
    player: player::Player,

    input_queue: Vec<player::Action>,
    last_tick_time: DateTime<Utc>,
}

impl State {
    pub fn new(ui: &UI) -> Self {
        State {
            player: player::Player::new(ui),
            input_queue: vec![],
            last_tick_time: Utc::now(),
        }
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Process the entire input queue and then clear it.
        for input in self.input_queue.iter() {
            match input {
                player::Action::Move(direction) => {
                    self.player.reposition(direction, Utc::now() - self.last_tick_time);
                }

                _ => {},
            }
        }

        self.input_queue = vec![];

        self.last_tick_time = Utc::now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        self.player.draw(ctx)?;

        graphics::present(ctx)?;
        ggez::timer::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        key_code: KeyCode,
        _key_mods: KeyMods,
        _repeat: bool
    ) {
        if let Some(action) = player::Action::from_key_code(key_code) {
            self.input_queue.push(action);
        }
    }
}
