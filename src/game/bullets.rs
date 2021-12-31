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


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Owner {
    Player,
    Enemy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Basic,
}

#[derive(Clone)]
pub struct Bullet {
    owner: Owner,
    kind: Kind,
    position: motion::Position<units::Pixels>,
    dimensions: motion::Dimensions<units::Pixels>,
}

impl Bullet {
    pub fn new(
        owner: Owner,
        kind: Kind,
        position: motion::Position<units::Pixels>,
    ) -> Self {
        Bullet {
            owner: owner,
            kind: kind,
            position: position,
            dimensions: kind.dimensions(),
        }
    }

    pub fn position(&self) -> motion::Position<units::Pixels> {
        self.position
    }

    pub fn owner(&self) -> Owner {
        self.owner
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }
    
    pub fn reposition(&mut self, time: Duration) {
        let dx = Self::horizontal_velocity(motion::Direction::Stationary, time).distance(time).0;
        let dy = Self::vertical_velocity(motion::Direction::Down, time).distance(time).0;

        self.position = motion::Position::new(
            units::Pixels(self.position.x.value() + dx),
            units::Pixels(self.position.y.value() + dy),
        );
    }

    pub fn hitbox_rect(&self) -> graphics::Rect {
        graphics::Rect::new(
            self.position.x.value(),
            self.position.y.value(),
            self.dimensions.width.value(),
            self.dimensions.height.value(),
        )
    }

    pub fn damage(&self) -> HealthPoints {
        match (self.owner, self.kind) {
            (Owner::Player, Kind::Basic) => HealthPoints::new(10),
            (Owner::Enemy, Kind::Basic)  => HealthPoints::new(5),
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let bullet_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.hitbox_rect(),
            graphics::Color::BLUE,
        )?;

        graphics::draw(ctx, &bullet_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        Ok(())
    }
}

impl Kind {
    pub fn dimensions(&self) -> motion::Dimensions<units::Pixels> {
        match self {
            Kind::Basic => motion::Dimensions::new(units::Pixels(20.0), units::Pixels(20.0)),
        }
    }
}

