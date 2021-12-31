use crate::{
    game::health::HealthPoints,
    physics::motion::*,
    physics::units,
};

use chrono::Duration;
use ggez::{
    graphics,
    Context,
    GameResult,
};


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Owner {
    Player,
    Enemy,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Basic,
}

#[derive(Clone)]
pub struct Bullet {
    owner: Owner,
    kind: Kind,
    position: Position<units::Pixels>,
    dimensions: Dimensions<units::Pixels>,
}

impl Bullet {
    pub fn new(
        owner: Owner,
        kind: Kind,
        position: Position<units::Pixels>,
    ) -> Self {
        Bullet {
            owner: owner,
            kind: kind,
            position: position,
            dimensions: kind.dimensions(),
        }
    }

    pub fn position(&self) -> Position<units::Pixels> {
        self.position
    }

    pub fn owner(&self) -> Owner {
        self.owner
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }
    
    pub fn reposition(&mut self, time: Duration) {
        let dx = self.horizontal_velocity(time).distance(time).0;
        let dy = self.vertical_velocity(time).distance(time).0;

        self.position = Position::new(
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
    pub fn dimensions(&self) -> Dimensions<units::Pixels> {
        match self {
            Kind::Basic => Dimensions::new(units::Pixels(20.0), units::Pixels(20.0)),
        }
    }
}

impl Acceleration<units::PixelsPerMs> for Bullet {
    fn horizontal_velocity(&self, time: Duration) -> Velocity<units::PixelsPerMs> {
        Velocity::new(0.0)
    }

    fn vertical_velocity(&self, time: Duration) -> Velocity<units::PixelsPerMs> {
        match self.owner {
            Owner::Enemy  => Velocity::new(0.5),
            Owner::Player => Velocity::new(-2.0),
        }
        
    }
}
