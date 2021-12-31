use crate::{
    config::ui::UI,
    game::health::{Health, HealthPoints},
    physics::motion::*,
    physics::units,
};

use chrono::prelude::*;
use chrono::Duration;

use ggez::{
    event::KeyCode,
    graphics::{
        self,
        Color
    },
    Context,
    GameResult,
};

/// The player's state.
pub struct Player {
    pub position: Position<units::Pixels>,
    pub dimensions: Dimensions<units::Pixels>,

    horizontal_direction: Direction,
    vertical_direction: Direction,

    health: HealthPoints,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KeyPress {
    Pressed,
    Released,
}

/// The various actions the player can take.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Action {
    Move(Direction),
    StopMoving(Direction),
    Shoot,
}

impl Player {
    pub fn new(ui: &UI) -> Self {
        Player {
            position: Position::new(
                  units::Pixels(ui.width / 2.0 - 12.0),
                  units::Pixels(ui.height - 64.0)
            ),
            dimensions: Dimensions::new(
                units::Pixels(24.0),
                units::Pixels(32.0)
            ),
            horizontal_direction: Direction::Stationary,
            vertical_direction: Direction::Stationary,
            health: HealthPoints::new(250),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let position = graphics::Rect::new(
            self.position.x.value(),
            self.position.y.value(),
            self.dimensions.width.value(),
            self.dimensions.height.value(),
        );
 
        let player_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            position,
            Color::RED,
        )?;

        graphics::draw(ctx, &player_rect, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        Ok(())
    }
    
    /// Reposition the player in some direction.
    /// The player's velocity is an inherent characteristic, however the time since
    /// the last tick must be taken into account to compute distance.
    pub fn reposition(&mut self, action: Action, time: Duration) {
        match action {
            Action::Move(dir) => {
                if dir.is_horizontal() {
                    self.horizontal_direction = dir;
                } else if dir.is_vertical() {
                    self.vertical_direction = dir;
                }
            },
            Action::StopMoving(dir) => {
                if dir.is_horizontal() {
                    self.horizontal_direction = Direction::Stationary;
                } else if dir.is_vertical() {
                    self.vertical_direction = Direction::Stationary;
                }
            },
            _ => {},
        }

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
}

impl Health for Player {
    fn health(&self) -> HealthPoints {
        self.health
    }

    fn restore_health(&mut self, amount: HealthPoints) -> HealthPoints {
        self.health = self.health - amount;
        self.health
    }

    fn take_damage(&mut self, amount: HealthPoints) -> HealthPoints {
        self.health = self.health - amount;
        self.health
    }
}

impl Acceleration<units::PixelsPerMs> for Player {
    fn horizontal_velocity(&self, time: Duration) -> Velocity<units::PixelsPerMs> {
        match self.horizontal_direction {
            Direction::Right => Velocity::new(0.5),
            Direction::Left  => Velocity::new(-0.5),
            _                => Velocity::new(0.0),
        }
    }
    
    fn vertical_velocity(&self, time: Duration) -> Velocity<units::PixelsPerMs> {
        match self.vertical_direction {
            Direction::Down => Velocity::new(0.5),
            Direction::Up   => Velocity::new(-0.5),
            _               => Velocity::new(0.0),
        }
    }
}

impl Action {
    pub fn from_key_code(key_code: KeyCode, key: KeyPress) -> Option<Self> {
        let action = if key == KeyPress::Released {
            Action::StopMoving
        } else {
            Action::Move
        };

        match key_code {
            KeyCode::Up    => Some(action(Direction::Up)),
            KeyCode::Down  => Some(action(Direction::Down)),
            KeyCode::Left  => Some(action(Direction::Left)),
            KeyCode::Right => Some(action(Direction::Right)),
            KeyCode::Space => Some(Action::Shoot),
            _              => None,
        }
    }
}
