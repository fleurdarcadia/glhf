use crate::config::ui::UI;
use crate::physics::{motion, units};
use super::{health, bullets, enemies, player};

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
    enemies: Vec<enemies::Enemy>,
    bullets: Vec<bullets::Bullet>,

    input_queue: Vec<player::Action>,
    last_tick_time: DateTime<Utc>,
    ui: UI,
}

impl State {
    pub fn new(ui: UI) -> Self {
        let tmp_default_enemy = enemies::Enemy::new(
            motion::Position::new(units::Pixels(300.0), units::Pixels(20.0)),
            motion::Dimensions::new(units::Pixels(32.0), units::Pixels(44.0)),
            health::HealthPoints::new(100),
            vec![
                bullets::Bullet::EnemyBasic(bullets::Basic::new(
                    motion::Position::new(units::Pixels(316.0), units::Pixels(64.0)),
                ))
            ],
        );

        State {
            player: player::Player::new(&ui),
            enemies: vec![tmp_default_enemy],
            bullets: vec![],
            input_queue: vec![],
            last_tick_time: Utc::now(),
            ui: ui,
        }
    }

    pub fn add_enemy(&mut self, enemy: enemies::Enemy) {
        self.enemies.push(enemy);
    }

    pub fn position_player_in_game_space(&mut self) {
        let max_x = self.ui.width - self.player.dimensions.width.value();
        let max_y = self.ui.height - self.player.dimensions.height.value();

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

        for enemy in self.enemies.iter_mut() {
            if let Some(bullet) = enemy.fire_bullet() {
                self.bullets.push(bullet);
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

        for enemy in self.enemies.iter() {
            enemy.draw(ctx)?;
        }

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
