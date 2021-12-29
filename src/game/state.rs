use crate::config::ui::UI;
use crate::physics::{motion, units};
use super::player;
use super::bullets;

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
    bullets: Vec<bullets::Bullet>,

    input_queue: Vec<player::Action>,
    last_tick_time: DateTime<Utc>,
    ui: UI,
}

impl State {
    pub fn new(ui: UI) -> Self {
        State {
            player: player::Player::new(&ui),
            bullets: vec![],
            input_queue: vec![],
            last_tick_time: Utc::now(),
            ui: ui,
        }
    }

    pub fn position_player_in_game_space(&mut self) {
        let max_x = self.ui.width - self.player.width;
        let max_y = self.ui.height - self.player.height;

        let new_x = if self.player.position.x.value() < 0.0 {
            0.0
        } else if self.player.position.x.value() > max_x {
            max_x
        } else {
            self.player.position.x.value()
        };

        let new_y = if self.player.position.y.value() < 0.0 {
            0.0
        } else if self.player.position.y.value() > max_y {
            max_y
        } else {
            self.player.position.y.value()
        };

        self.player.position = motion::Position::new(units::Pixels(new_x), units::Pixels(new_y));
    }

    pub fn cleanup_out_of_bounds_bullets(&mut self) {
        let ui_rect = self.ui.hitbox_rect();

        let mut remaining_bullets: Vec<bullets::Bullet> = vec![];

        for bullet in self.bullets.iter() {
            if bullet.hitbox_rect().overlaps(&ui_rect) {
                remaining_bullets.push(bullet.clone());
            }
        }

        self.bullets = remaining_bullets;
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let time_since_last_tick = Utc::now() - self.last_tick_time;

        // Process the entire input queue and then clear it.
        for input in self.input_queue.iter() {
            match input {
                player::Action::Move(direction) => {
                    self.player.reposition(*direction, time_since_last_tick);
                }

                player::Action::Shoot => {
                    let bullet = bullets::Bullet::Player(bullets::PlayerBullet::new(
                        self.player.position
                    ));

                    self.bullets.push(bullet);
                }
            }
        }

        for bullet in self.bullets.iter_mut() {
            bullet.reposition(time_since_last_tick);
        }

        // Cleanup after updating everything
        self.input_queue = vec![];
        self.cleanup_out_of_bounds_bullets();
        self.position_player_in_game_space();

        self.last_tick_time = Utc::now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);

        self.player.draw(ctx)?;

        for bullet in self.bullets.iter() {
            bullet.draw(ctx)?;
        }

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
