use crate::{
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

pub struct PlayerBullet {
    pub position: motion::Position<units::Pixels>,
}

pub enum Bullet {
    Player(PlayerBullet),
}

impl Bullet {
    pub fn reposition(&mut self, time: Duration) {
        match self {
            Bullet::Player(bullet) => bullet.reposition(time),
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let position = match self {
            Bullet::Player(PlayerBullet{ position }) =>
                graphics::Rect::new(position.0.0, position.1.0, 20.0, 20.0),
        };

        let bullet_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            position,
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

    pub fn reposition(&mut self, time: Duration) {
        let dx = Self::horizontal_velocity(motion::Direction::Up, time).distance(time).0;
        let dy = Self::vertical_velocity(motion::Direction::Stationary, time).distance(time).0;

        self.position = motion::Position(
            units::Pixels(self.position.0.0 + dx),
            units::Pixels(self.position.1.0 + dy),
        );
    }
}
