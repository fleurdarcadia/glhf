use crate::{
    game::{
        bullets,
        health::{Health, HealthPoints},
    },
    physics::{
        motion,
        units,
    },
};

use chrono::prelude::*;
use chrono::Duration;
use ggez::{
    graphics::{
        self,
        Color,
    },
    Context,
    GameResult,
};


#[derive(Clone)]
pub struct Enemy {
    pub position: motion::Position<units::Pixels>,
    pub dimensions: motion::Dimensions<units::Pixels>,
   
    health: HealthPoints,
    bullet_rotation: Vec<bullets::Bullet>,
    current_bullet_index: usize,
    last_fired: DateTime<Utc>,
}

impl Enemy {
    pub fn new(
        pos: motion::Position<units::Pixels>,
        dim: motion::Dimensions<units::Pixels>,
        health: HealthPoints,
        bullets: Vec<bullets::Bullet>,
    ) -> Self {
        Enemy {
            position: pos,
            dimensions: dim,
            health: health,
            bullet_rotation: bullets,
            current_bullet_index: 0usize,
            last_fired: Utc::now(),
        }
    }

    pub fn hitbox_rect(&self) -> graphics::Rect {
        graphics::Rect::new(
            self.position.x.value(),
            self.position.y.value(),
            self.dimensions.width.value(),
            self.dimensions.height.value(),
        )
    }

    pub fn fire_bullet(&mut self) -> Option<bullets::Bullet> {
        let time_since_last_update = Utc::now() - self.last_fired;

        if time_since_last_update.num_milliseconds() / 500 > 0 {
            let bullet = self.bullet_rotation[self.current_bullet_index].clone();

            self.last_fired = Utc::now();
            self.current_bullet_index = (self.current_bullet_index + 1) % self.bullet_rotation.len();

            Some(bullet)
        } else {
            None
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let position = graphics::Rect::new(
            self.position.x.value(),
            self.position.y.value(),
            self.dimensions.width.value(),
            self.dimensions.height.value(),
        );

        let enemy_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            position,
            Color::MAGENTA,
        )?;

        graphics::draw(ctx, &enemy_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        Ok(())
    }
}

impl Health for Enemy {
    fn health(&self) -> HealthPoints {
        self.health
    }

    fn restore_health(&mut self, _amt: HealthPoints) -> HealthPoints {
        // Enemies cannot heal
        self.health
    }

    fn take_damage(&mut self, amount: HealthPoints) -> HealthPoints {
        self.health = self.health - amount;
        self.health
    }
}
