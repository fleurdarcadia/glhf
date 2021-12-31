use crate::{
    config::ui::UI,
    game::{
        bullets,
        enemies,
        health::{
            self,
            Health,
        },
        player,
    },
    physics::{
        motion,
        units,
    },
};


use chrono::prelude::*;
use chrono::Duration;
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
                bullets::Bullet::new(
                    bullets::Owner::Enemy,
                    bullets::Kind::Basic,
                    motion::Position::new(units::Pixels(316.0), units::Pixels(64.0)),
                ),
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

    pub fn process_input_queue(&mut self, time_since_last_tick: Duration) {
        for input in self.input_queue.iter() {
            match input {
                player::Action::Shoot => {
                    let bullet = bullets::Bullet::new(
                        bullets::Owner::Player,
                        bullets::Kind::Basic,
                        self.player.position,
                    );

                    self.bullets.push(bullet);
                },

                mvmt@_ => self.player.reposition(*mvmt, time_since_last_tick),
            }
        }

        self.input_queue = vec![];
    }

    pub fn connect_bullets_with_player(&mut self) -> Vec<usize> {
        let hitbox = self.player.hitbox_rect();
        
        let mut spent_bullet_indices = vec![];
        let mut bullet_index = 0usize;

        for bullet in self.bullets.iter() {
            if bullet.owner() == bullets::Owner::Enemy && bullet.hitbox_rect().overlaps(&hitbox) {
                self.player.take_damage(bullet.damage());
                spent_bullet_indices.push(bullet_index);
            }

            bullet_index += 1;
        }

        spent_bullet_indices
    }

    pub fn connect_bullets_with_enemies(&mut self) -> Vec<usize> {
        let mut spent_bullet_indices = vec![];

        for enemy in self.enemies.iter_mut() {
            let hitbox = enemy.hitbox_rect();

            let mut bullet_index = 0usize;
            for bullet in self.bullets.iter() {
                if bullet.owner() == bullets::Owner::Player && bullet.hitbox_rect().overlaps(&hitbox) {
                    enemy.take_damage(bullet.damage());
                    spent_bullet_indices.push(bullet_index);
                }
                bullet_index += 1;
            }
        }

        spent_bullet_indices
    }

    pub fn cleanup_defeated_enemies(&mut self) {
        let mut remaining_enemies = vec![];

        for enemy in self.enemies.iter() {
            if !enemy.health().empty() {
                remaining_enemies.push(enemy.clone());
            }
        }

        self.enemies = remaining_enemies;
    }

    pub fn trigger_enemy_behaviours(&mut self) {
        for enemy in self.enemies.iter_mut() {
            if let Some(bullet) = enemy.fire_bullet() {
                self.bullets.push(bullet);
            }
        }
    }

    pub fn update_bullets(&mut self, time_since_last_tick: Duration) {
        for bullet in self.bullets.iter_mut() {
            bullet.reposition(time_since_last_tick);
        }
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

    pub fn cleanup_spent_bullets(&mut self, indices: Vec<usize>) {
        let mut remaining_bullets: Vec<bullets::Bullet> = vec![];
        let mut index = 0usize;

        for bullet in self.bullets.iter() {
            let mut should_include = true;

            for excluded in indices.iter() {
                if index == *excluded {
                    should_include = false;
                    break;
                }
            }

            if should_include {
                remaining_bullets.push(bullet.clone());
            }

            index += 1;
        }

        self.bullets = remaining_bullets;
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

        self.process_input_queue(time_since_last_tick);
        self.position_player_in_game_space();
        let spent_bullet_indices = self.connect_bullets_with_player();
        self.cleanup_spent_bullets(spent_bullet_indices);
        println!("Player health: {:?}", self.player.health());

        let spent_bullet_indices = self.connect_bullets_with_enemies();
        self.cleanup_defeated_enemies();
        self.trigger_enemy_behaviours();
        
        self.cleanup_spent_bullets(spent_bullet_indices);
        self.update_bullets(time_since_last_tick);
        self.cleanup_out_of_bounds_bullets();

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
        if let Some(action) = player::Action::from_key_code(key_code, player::KeyPress::Pressed) {
            self.input_queue.push(action);
        }
    }
}
