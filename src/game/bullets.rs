use crate::{
    game::health::HealthPoints,
    physics::motion::{
        self,
        Acceleration,
    },
    physics::units,
};

use chrono::Duration;
use ggez::{
    graphics,
    Context,
    GameResult,
};

#[derive(Clone)]
pub struct PlayerBullet {
    pub position: motion::Position<units::Pixels>,
}

#[derive(Clone)]
pub struct Basic {
    pub position: motion::Position<units::Pixels>,
}

#[derive(Clone)]
pub enum Bullet {
    Player(PlayerBullet),
    EnemyBasic(Basic),
}

impl Bullet {
    pub fn reposition(&mut self, time: Duration) {
        match self {
            Bullet::Player(bullet)     => bullet.reposition(time),
            Bullet::EnemyBasic(bullet) => bullet.reposition(time),
        }
    }

    pub fn position(&self) -> motion::Position<units::Pixels> {
        match self {
            Bullet::Player(PlayerBullet{ position }) => position.clone(),
            Bullet::EnemyBasic(Basic{ position })    => position.clone(),
        }
    }

    pub fn hitbox_rect(&self) -> graphics::Rect {
        let pos = self.position();

        graphics::Rect::new(pos.x.value(), pos.y.value(), 20.0, 20.0)
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let pos = self.position();
        let rect = graphics::Rect::new(pos.x.value(), pos.y.value(), 20.0, 20.0);
        let bullet_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            graphics::Color::BLUE,
        )?;

        graphics::draw(ctx, &bullet_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        Ok(())
    }
}

impl PlayerBullet {
    pub fn new(position: motion::Position<units::Pixels>) -> Self {
        PlayerBullet {
            position: position,
        }
    }

    pub fn damage(&self) -> HealthPoints {
        HealthPoints::new(10)
    }

    pub fn reposition(&mut self, time: Duration) {
        let dx = Self::horizontal_velocity(motion::Direction::Up, time).distance(time).0;
        let dy = Self::vertical_velocity(motion::Direction::Stationary, time).distance(time).0;

        self.position = motion::Position::new(
            units::Pixels(self.position.x.value() + dx),
            units::Pixels(self.position.y.value() + dy),
        );
    }
}

impl Basic {
    pub fn new(position: motion::Position<units::Pixels>) -> Self {
        Basic {
            position: position,
        }
    }

    pub fn damage(&self) -> HealthPoints {
        HealthPoints::new(5)
    }

    pub fn reposition(&mut self, time: Duration) {
        let dx = Self::horizontal_velocity(motion::Direction::Down, time).distance(time).0;
        let dy = Self::vertical_velocity(motion::Direction::Stationary, time).distance(time).0;

        self.position = motion::Position::new(
            units::Pixels(self.position.x.value() + dx),
            units::Pixels(self.position.y.value() + dy),
        );
    }
}
